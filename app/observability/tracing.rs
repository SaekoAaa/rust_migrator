use opentelemetry_otlp::{Protocol, SpanExporter, WithExportConfig};
use opentelemetry_sdk::trace::{RandomIdGenerator, Sampler, SdkTracerProvider};

use crate::observability::otel::get_resource;
pub fn init_traces(otel_collector_url: &str) -> SdkTracerProvider {
    let exporter = SpanExporter::builder()
        .with_http()
        .with_endpoint(otel_collector_url)
        .with_protocol(Protocol::HttpJson) //can be changed to `Protocol::HttpJson` to export in JSON format
        .build()
        .expect("Failed to create trace exporter");

    let provider = SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(get_resource())
        .with_sampler(Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(
            1.0,
        ))))
        .with_id_generator(RandomIdGenerator::default())
        .build();
    provider
}
