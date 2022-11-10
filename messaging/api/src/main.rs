use actix_web::{middleware::Logger, web, App, HttpServer};
use infrastructure::{
    pg,
    telemetry::{self, JaegerSettings, LoggingOptions, LoggingSettings, Settings},
};
use messaging_api::{routes, settings};
use std::time::Duration;

#[actix_web::main]
async fn main() -> eyre::Result<()> {
    let settings = settings::get_config().expect("failed to get settings");
    let shared_settings = settings.clone();
    telemetry::setup(Settings {
        log: LoggingSettings {
            format: LoggingOptions::PrettyPrint,
        },
        jaeger: JaegerSettings {
            host: settings.jaeger.host,
            port: settings.jaeger.port,
        },
        service_name: settings.app.service_name,
    });

    let pool = pg::connection_pool(pg::PostgresSettings {
        max_size: settings.database.max_size,
        timeout: Some(Duration::from_secs(settings.database.timeout)),
        url: settings.database.url,
    })?;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(shared_settings.clone()))
            .app_data(web::Data::new(pool.clone()))
            .service(routes::sessions::start_session)
    })
    .workers(1)
    .bind((settings.app.host, settings.app.port))?
    .run()
    .await?;

    telemetry::teardown().await;

    Ok(())
}
