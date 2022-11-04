use std::collections::HashMap;

use actix::prelude::*;


#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct ReceiveMessage {
    pub message: String,

    pub from: String,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct SendMessage {
    pub message: String,

    pub to: String,

    pub from: String,
}

#[derive(Message, Debug)]
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

    #[tracing::instrument(name = "Handler Connect", skip(self, msg, _ctx))]
    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        self.sessions.insert(msg.user_id, msg.addr);
    }
}

impl Handler<SendMessage> for SessionManager {
    type Result = ();

    #[tracing::instrument(name = "Handler SendMessage", skip(self, _ctx))]
    fn handle(&mut self, msg: SendMessage, _ctx: &mut Self::Context) -> Self::Result {
        match self.sessions.get(&msg.to) {
            Some(s) => s.do_send(ReceiveMessage {
                from: msg.from,
                message: msg.message,
            }),
            None => {}
        };
    }
}
