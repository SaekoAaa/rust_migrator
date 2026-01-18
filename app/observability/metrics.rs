use std::sync::OnceLock;

use opentelemetry::global;
use opentelemetry::metrics::{Counter, Histogram};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::metrics::PeriodicReader;

use opentelemetry_sdk::metrics::SdkMeterProvider;

use crate::observability::otel::get_resource;

pub(crate) static MIGRATIONS_COUNTER: OnceLock<Counter<u64>> = OnceLock::new();
pub(crate) static MIGRATIONS_DURATION: OnceLock<Histogram<f64>> = OnceLock::new();

#[tracing::instrument]
pub fn init_metrics(collector_url: &str) -> anyhow::Result<SdkMeterProvider> {
    tracing::debug!("Telemetry address: {}", collector_url);
    let http_exporter = opentelemetry_otlp::MetricExporter::builder()
        .with_http()
        .with_endpoint(collector_url)
        .build()?;
    let http_reader = PeriodicReader::builder(http_exporter).build();
    let meter_provider = SdkMeterProvider::builder()
        .with_reader(http_reader)
        .with_resource(get_resource())
        .build();
    global::set_meter_provider(meter_provider.clone());
    let http_meter = global::meter("http");
    MIGRATIONS_COUNTER
        .set(
            http_meter
                .u64_counter("migrations_counter")
                .with_description("Shows amount of migrations")
                .build(),
        )
        .expect("MIGRATIONS_COUNTER already initialized");

    MIGRATIONS_DURATION
        .set(
            http_meter
                .f64_histogram("migrations_duration")
                .with_description("Shows latency of migrations")
                .build(),
        )
        .expect("MIGRATIONS_DURATION already initialized");
    Ok(meter_provider)
}
