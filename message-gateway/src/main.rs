use actix::{Actor, Addr};
use actix_files::{Files, NamedFile};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use message_gateway::{
    actors::conn::Heartbeat,
    settings::{self},
    telemetry,
};
use serde::Deserialize;
use std::time::{Duration, Instant};
use tracing_actix_web::TracingLogger;

use message_gateway::actors::{chat::ConnManager, conn::WsConn};

async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
}

#[derive(Deserialize, Debug)]
struct ChatQueryParams {
    user_id: String,
}

#[tracing::instrument(name = "/ws/chat", skip(stream, settings, session_manager))]
async fn chat(
    req: HttpRequest,
    query_params: web::Query<ChatQueryParams>,
    stream: web::Payload,
    settings: web::Data<settings::Settings>,
    session_manager: web::Data<Addr<ConnManager>>,
) -> Result<HttpResponse, Error> {
    let user_id = query_params.user_id.clone();

    ws::start(
        WsConn {
            user_id,
            hb: Heartbeat {
                time: Instant::now(),
                timeout: Duration::from_secs(settings.ws_conn.timeout),
                interval: Duration::from_secs(settings.ws_conn.interval),
            },
            conn_manager: session_manager.get_ref().clone(),
        },
        &req,
        stream,
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = settings::get_config().expect("failed to get settings");
    let shared_settings = settings.clone();
    telemetry::setup(settings.app.environment);

    let conn_manager = ConnManager::new().start();

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(conn_manager.clone()))
            .app_data(web::Data::new(shared_settings.clone()))
            .service(Files::new("/static", "./static"))
            .route("/", web::get().to(index))
            .route("/ws", web::get().to(chat))
    })
    .workers(1)
    .bind((settings.app.host, settings.app.port))?
    .run()
    .await?;

    telemetry::teardown().await;

    Ok(())
}
