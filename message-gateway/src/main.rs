use actix::{Actor, Addr};
use actix_files::{Files, NamedFile};
use actix_web::{
    middleware::Logger, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_web_actors::ws;
use serde::Deserialize;
use std::time::Instant;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::{filter::filter_fn, prelude::*, EnvFilter, Registry};

use message_gateway::{chat::SessionManager, session::WsSession};

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
            user_id: user_id.into(),
            hb: Instant::now(),
            session_manager: server.get_ref().clone(),
        },
        &req,
        stream,
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    LogTracer::init().expect("Failed to set logger");
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let emit_bunyan = false;
    let bunyan_json_layer = JsonStorageLayer.with_filter(filter_fn(move |_| emit_bunyan));
    let bunyan_formatting_layer =
        BunyanFormattingLayer::new("tracing_demo".into(), std::io::stdout)
            .with_filter(filter_fn(move |_| emit_bunyan));

    let pretty_formatting_layer = tracing_subscriber::fmt::layer()
        .with_span_events(FmtSpan::NEW)
        .with_filter(filter_fn(move |_| true));

    let subscriber = Registry::default()
        .with(env_filter)
        .with(bunyan_json_layer)
        .with(bunyan_formatting_layer)
        .with(pretty_formatting_layer);

    set_global_default(subscriber).expect("Failed to set subscriber");

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
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
