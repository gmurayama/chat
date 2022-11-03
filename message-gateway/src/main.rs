use actix::{Actor, Addr};
use actix_files::{Files, NamedFile};
use actix_web::{error, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use serde::Deserialize;
use std::time::Instant;

use message_gateway::{chat::SessionManager, session::WsSession};

async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
}

#[derive(Deserialize)]
struct ChatQueryParams {
    user_id: String,
}

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
    let server = SessionManager::new().start();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server.clone()))
            .service(Files::new("/static", "./static"))
            .route("/", web::get().to(index))
            .route("/ws", web::get().to(chat))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
