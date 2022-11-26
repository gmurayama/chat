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

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub user_id: String,
}

// -------------------
// ConnManager

pub struct ConnManager {
    pub connections: HashMap<String, Recipient<ReceiveMessage>>,
}

impl ConnManager {
    pub fn new() -> Self {
        ConnManager {
            connections: HashMap::new(),
        }
    }
}

impl Actor for ConnManager {
    type Context = Context<Self>;
}

impl Handler<Connect> for ConnManager {
    type Result = ();

    #[tracing::instrument(name = "actors.chat.Connect", skip(self, msg, _ctx))]
    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        self.connections.insert(msg.user_id, msg.addr);
    }
}

impl Handler<Disconnect> for ConnManager {
    type Result = ();

    #[tracing::instrument(name = "actors.chat.Disconnect", skip(self, _ctx))]
    fn handle(&mut self, msg: Disconnect, _ctx: &mut Self::Context) -> Self::Result {
        self.connections.remove(&msg.user_id);
    }
}

impl Handler<SendMessage> for ConnManager {
    type Result = ();

    #[tracing::instrument(name = "actors.chat.SendMessage", skip(self, _ctx))]
    fn handle(&mut self, msg: SendMessage, _ctx: &mut Self::Context) -> Self::Result {
        match self.connections.get(&msg.to) {
            Some(s) => s.do_send(ReceiveMessage {
                from: msg.from,
                message: msg.message,
            }),
            None => {}
        };
    }
}
