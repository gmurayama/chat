#[derive(thiserror::Error, Debug)]
pub enum ReplyError {
    #[error("Unknown message \"{0}\"")]
    UnknownMessage(String),
}

#[tracing::instrument(name = "application.messages.reply")]
pub fn reply(message: &str) -> Result<String, ReplyError> {
    match message {
        "hello" => Ok("world".to_string()),
        "ping" => Ok("pong".to_string()),
        _ => Err(ReplyError::UnknownMessage(message.to_string())),
    }
}
