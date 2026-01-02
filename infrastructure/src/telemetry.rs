use opentelemetry::{global, trace::TracerProvider};
use opentelemetry_otlp::SpanExporter;
use opentelemetry_sdk::{propagation::TraceContextPropagator, trace::SdkTracerProvider, Resource};
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{
    filter::filter_fn, fmt::format::FmtSpan, prelude::__tracing_subscriber_SubscriberExt,
    EnvFilter, Layer, Registry,
};

#[derive(PartialEq)]
pub enum LoggingOptions {
    PrettyPrint,
    JSON,
}

pub struct TelemetrySettings {
    pub host: String,
    pub port: u32,
    pub sampler_param: f64,
}

pub struct LoggingSettings {
    pub format: LoggingOptions,
}

pub struct Settings {
    pub log: LoggingSettings,
    pub telemetry: TelemetrySettings,
    pub service_name: String,
}

pub struct TelemetryGuard {
    tracer_provider: SdkTracerProvider,
}

impl Drop for TelemetryGuard {
    fn drop(&mut self) {
        if let Err(err) = self.tracer_provider.shutdown() {
            log::error!("failed to shutdown tracer provider: {err}");
        }
    }
}

pub fn setup(settings: Settings) -> TelemetryGuard {
    LogTracer::init().expect("Failed to set logger");
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let emit_bunyan = settings.log.format == LoggingOptions::JSON;
    let bunyan_json_layer = JsonStorageLayer.with_filter(filter_fn(move |_| emit_bunyan));
    let bunyan_formatting_layer =
        BunyanFormattingLayer::new(settings.service_name.clone(), std::io::stdout)
            .with_filter(filter_fn(move |_| emit_bunyan));

    let emit_pretty_formating = settings.log.format == LoggingOptions::PrettyPrint;
    let pretty_formatting_layer = tracing_subscriber::fmt::layer()
        .with_span_events(FmtSpan::NEW)
        .with_filter(filter_fn(move |_| emit_pretty_formating));

    global::set_text_map_propagator(TraceContextPropagator::new());

    let exporter = SpanExporter::builder()
        .with_tonic()
        .build()
        .expect("Failed to create span exporter");
    let provider = SdkTracerProvider::builder()
        .with_resource(
            Resource::builder()
                .with_service_name(settings.service_name)
                .build(),
        )
        .with_batch_exporter(exporter)
        .build();
    let tracer = provider.tracer("app");
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    global::set_tracer_provider(provider.clone());

    let subscriber = Registry::default()
        .with(env_filter)
        .with(telemetry)
        .with(bunyan_json_layer)
        .with(bunyan_formatting_layer)
        .with(pretty_formatting_layer);

    set_global_default(subscriber).expect("Failed to set subscriber");

    TelemetryGuard {
        tracer_provider: provider,
    }
}
