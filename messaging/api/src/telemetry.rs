use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{
    filter::filter_fn, fmt::format::FmtSpan, prelude::__tracing_subscriber_SubscriberExt,
    EnvFilter, Layer, Registry,
};

use crate::settings;

pub fn setup(env: settings::Environment) {
    LogTracer::init().expect("Failed to set logger");
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let emit_bunyan = env == settings::Environment::Production;
    let bunyan_json_layer = JsonStorageLayer.with_filter(filter_fn(move |_| emit_bunyan));
    let bunyan_formatting_layer =
        BunyanFormattingLayer::new("tracing_demo".into(), std::io::stdout)
            .with_filter(filter_fn(move |_| emit_bunyan));

    let emit_pretty_formating = env == settings::Environment::Development;
    let pretty_formatting_layer = tracing_subscriber::fmt::layer()
        .with_span_events(FmtSpan::NEW)
        .with_filter(filter_fn(move |_| emit_pretty_formating));

    let subscriber = Registry::default()
        .with(env_filter)
        .with(bunyan_json_layer)
        .with(bunyan_formatting_layer)
        .with(pretty_formatting_layer);

    set_global_default(subscriber).expect("Failed to set subscriber");
}
