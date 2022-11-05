use actix_web::{middleware::Logger, web, App, HttpServer};
use sessions::{routes, settings, telemetry};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = settings::get_config().expect("failed to get settings");
    let shared_settings = settings.clone();
    telemetry::setup(settings.app.environment);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(shared_settings.clone()))
            .service(routes::message::route_message)
    })
    .workers(1)
    .bind((settings.app.host, settings.app.port))?
    .run()
    .await
}
