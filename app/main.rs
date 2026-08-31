use crate::infrastructure::app;
use tracing::{info_span, level_filters::LevelFilter};
use tracing_subscriber::{Layer, layer::SubscriberExt, util::SubscriberInitExt};

#[cfg(feature = "telemetry")]
use {
    crate::observability::{metrics::init_metrics, otel::OtelGuard, tracing::init_traces},
    dotenvy::var,
    opentelemetry::trace::TracerProvider as _,
    tracing_opentelemetry::OpenTelemetryLayer,
};
mod infrastructure;
mod observability;
fn main() {
    if let Err(error) = run() {
        tracing::error!(error = %error, "Migration failed");
        eprintln!("Migration failed: {error}");
        std::process::exit(1);
    }
}

fn run() -> anyhow::Result<()> {
    if let Err(e) = dotenvy::dotenv() {
        println!("Dotenv import 2 failed: {}. Fine for docker", e);
    };

    #[cfg(not(feature = "telemetry"))]
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_filter(LevelFilter::INFO))
        .init();

    #[cfg(feature = "telemetry")]
    let _otel_guard = init_telemetry()?;

    let main_span = info_span!("app");
    let _g = main_span.enter();
    app::run()
}

#[cfg(feature = "telemetry")]
fn init_telemetry() -> anyhow::Result<OtelGuard> {
    let level = tracing_subscriber::fmt::layer().with_filter(LevelFilter::INFO);
    let subscriber = tracing_subscriber::registry().with(level);
    let collector_url = var("COLLECTOR_URL").ok();
    let use_traces = var("WITH_TRACING").is_ok_and(|t| t == "true");

    let mut tracing_provider = None;
    if use_traces {
        if let Some(collector_url) = &collector_url {
            let traces = init_traces(&format!("{}/traces", collector_url))?;
            let tracer = traces.tracer("Migrator tracing");
            subscriber.with(OpenTelemetryLayer::new(tracer)).init();
            tracing_provider = Some(traces)
        } else {
            subscriber.init();
        }
    } else {
        subscriber.init();
    }

    let use_metrics = var("WITH_METRICS").is_ok_and(|t| t == "true");
    let mut metrics_provider = None;
    if use_metrics && let Some(collector_url) = collector_url {
        let metrics = init_metrics(&format!("{}/metrics", collector_url))?;
        metrics_provider = Some(metrics)
    }
    Ok(OtelGuard {
        tracer_provider: tracing_provider,
        meter_provider: metrics_provider,
    })
}
