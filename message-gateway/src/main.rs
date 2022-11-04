use actix::{Actor, Addr};
use actix_files::{Files, NamedFile};
use actix_web::{
    middleware::Logger, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_web_actors::ws;
use message_gateway::{settings, telemetry};
use serde::Deserialize;
use std::time::Instant;

use message_gateway::actors::{chat::SessionManager, session::WsSession};

async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
}

#[derive(Deserialize, Debug)]
struct ChatQueryParams {
    user_id: String,
}

#[tracing::instrument(name = "starting websocket", skip(stream, server))]
async fn chat(
    req: HttpRequest,
    query_params: web::Query<ChatQueryParams>,
    stream: web::Payload,
    server: web::Data<Addr<SessionManager>>,
) -> Result<HttpResponse, Error> {
    let user_id = query_params.user_id.clone();

    ws::start(
        WsSession {
            user_id,
            hb: Instant::now(),
            session_manager: server.get_ref().clone(),
        },
        &req,
        stream,
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = settings::get_config().expect("failed to get settings");
    telemetry::setup(settings.app.environment);

    let server = SessionManager::new().start();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(server.clone()))
            .service(Files::new("/static", "./static"))
            .route("/", web::get().to(index))
            .route("/ws", web::get().to(chat))
    })
    .workers(1)
    .bind((settings.app.host, settings.app.port))?
    .run()
    .await
}
