use actix::{prelude::*, Actor, StreamHandler};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

use crate::actors::chat::{Connect, Disconnect, ReceiveMessage, SendMessage, SessionManager};

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientMessage {
    pub msg: String,
    pub to: String,
}

pub struct Heartbeat {
    pub time: Instant,
    pub timeout: Duration,
    pub interval: Duration,
}

pub struct WsSession {
    pub user_id: String,

    pub hb: Heartbeat,

    pub session_manager: Addr<SessionManager>,
}

impl WsSession {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        let interval = self.hb.interval.clone();
        let timeout = self.hb.timeout.clone();

        ctx.run_interval(interval, move |act, ctx| {
            if Instant::now().duration_since(act.hb.time) > timeout {
                log::error!("Websocket Client heartbeat failed, disconnecting!");

                act.session_manager.do_send(Disconnect {
                    user_id: act.user_id.clone(),
                });

                ctx.stop();

                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();

        self.session_manager
            .send(Connect {
                user_id: self.user_id.clone(),
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, _, ctx| {
                if res.is_err() {
                    ctx.stop();
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify chat server
        self.session_manager.do_send(Disconnect {
            user_id: self.user_id.clone(),
        });

        Running::Stop
    }
}

impl Handler<ReceiveMessage> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: ReceiveMessage, ctx: &mut Self::Context) {
        let message = format!("{}: {}", msg.from, msg.message);
        ctx.text(message);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Ok(msg) => msg,
            Err(_) => {
                ctx.stop();
                return;
            }
        };

        match msg {
            ws::Message::Ping(msg) => {
                self.hb.time = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb.time = Instant::now();
            }
            ws::Message::Text(text) => {
                let client_message: ClientMessage = match serde_json::from_str(&text) {
                    Ok(msg) => msg,
                    Err(err) => {
                        let err_msg = err.to_string();
                        ctx.text(err_msg);
                        return;
                    }
                };

                self.session_manager.do_send(SendMessage {
                    from: self.user_id.clone(),
                    to: client_message.to,
                    message: client_message.msg,
                });
            }
            ws::Message::Binary(_) => println!("unexpected binary"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}
