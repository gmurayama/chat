use std::collections::HashMap;

use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "()")]
pub struct ReceiveMessage {
    pub message: String,

    pub from: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SendMessage {
    pub message: String,

    pub to: String,

    pub from: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub user_id: String,

    pub addr: Recipient<ReceiveMessage>,
}

// -------------------
// SessionManager

pub struct SessionManager {
    pub sessions: HashMap<String, Recipient<ReceiveMessage>>,
}

impl SessionManager {
    pub fn new() -> Self {
        SessionManager {
            sessions: HashMap::new(),
        }
    }
}

impl Actor for SessionManager {
    type Context = Context<Self>;
}

impl Handler<Connect> for SessionManager {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
        self.sessions.insert(msg.user_id, msg.addr);
    }
}

impl Handler<SendMessage> for SessionManager {
    type Result = ();

    fn handle(&mut self, msg: SendMessage, ctx: &mut Self::Context) -> Self::Result {
        match self.sessions.get(&msg.to) {
            Some(s) => s.do_send(ReceiveMessage {
                from: msg.from,
                message: msg.message,
            }),
            None => {}
        };
    }
}
