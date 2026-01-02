use actix_web::{web, HttpRequest, HttpResponse};
use actix_ws::Message;
use futures_util::StreamExt as _;

#[tracing::instrument(name = "gateways.api.routes.messages_ws", skip(body))]
pub async fn message_ws(req: HttpRequest, body: web::Payload) -> actix_web::Result<HttpResponse> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;

    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Ping(bytes) => {
                    if session.pong(&bytes).await.is_err() {
                        return;
                    }
                }

                Message::Text(msg) => println!("Got text: {msg}"),
                _ => break,
            }
        }

        let _ = session.close(None).await;
    });

    Ok(response)
}
