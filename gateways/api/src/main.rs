use api::server;
use infrastructure::{self, telemetry};
use prometheus_client::registry::Registry;

mod settings;
use settings::get_config;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let settings = get_config()?;

    let _guard = telemetry::setup(telemetry::Settings {
        log: telemetry::LoggingSettings {
            format: telemetry::LoggingOptions::PrettyPrint,
        },
        telemetry: telemetry::TelemetrySettings {
            host: settings.telemetry.host,
            port: settings.telemetry.port,
            sampler_param: settings.telemetry.sampler_param,
        },
        service_name: settings.app.service_name,
    });

    let registry = Registry::default();

    let server = server::Server::setup(server::Settings {
        app: server::AppSettings {
            host: settings.app.host,
            port: settings.app.port,
            request_timeout_sec: settings.app.request_timeout_sec,
        },
        metrics: server::MetricSettings {
            host: settings.metric.host,
            port: settings.metric.port,
            registry,
        },
    })?;

    server.run().await?;

    Ok(())
}
