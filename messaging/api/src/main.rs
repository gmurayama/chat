use actix_web::{middleware::Logger, web, App, HttpServer};
use infrastructure::telemetry::{self, JaegerSettings, LoggingOptions, LoggingSettings, Settings};
use messaging_api::{routes, settings};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = settings::get_config().expect("failed to get settings");
    let shared_settings = settings.clone();
    telemetry::setup(Settings {
        log: LoggingSettings {
            format: LoggingOptions::PrettyPrint,
        },
        jaeger: JaegerSettings {
            host: "127.0.0.1".into(),
            port: 6831,
        },
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(shared_settings.clone()))
            .service(routes::message::route_message)
    })
    .workers(1)
    .bind((settings.app.host, settings.app.port))?
    .run()
    .await?;

    telemetry::teardown();

    Ok(())
}
