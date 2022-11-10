use std::time::Duration;

use opentelemetry::{
    global,
    sdk::trace::{self, Sampler},
};
use tokio::task;
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

pub struct JaegerSettings {
    pub host: String,
    pub port: u32,
}

pub struct LoggingSettings {
    pub format: LoggingOptions,
}

pub struct Settings {
    pub log: LoggingSettings,
    pub jaeger: JaegerSettings,
    pub service_name: String,
}

pub fn setup(settings: Settings) {
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

    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name(settings.service_name.clone())
        .with_endpoint(format!("{}:{}", settings.jaeger.host, settings.jaeger.port))
        .with_trace_config(trace::config().with_sampler(Sampler::AlwaysOn))
        .install_batch(opentelemetry::runtime::Tokio)
        .unwrap();

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    let subscriber = Registry::default()
        .with(env_filter)
        .with(bunyan_json_layer)
        .with(bunyan_formatting_layer)
        .with(pretty_formatting_layer)
        .with(telemetry);

    set_global_default(subscriber).expect("Failed to set subscriber");
}

pub async fn teardown() {
    let res = task::spawn_blocking(move || {
        global::shutdown_tracer_provider();
    });

    if let Err(_) = tokio::time::timeout(Duration::from_secs(5), res).await {
        log::error!("could not shutdown tracer provider in 5 sec");
    }
}
