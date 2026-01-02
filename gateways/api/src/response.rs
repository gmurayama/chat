use std::fmt::Display;

use actix_web::error;
use serde::Serialize;
use serde_json::json;

pub fn internal_server_error<T: Display>(err: T) -> error::Error {
    error::ErrorInternalServerError(json!({
        "message": format!("{err}")
    }))
}

pub fn not_found<T: Display>(err: T) -> error::Error {
    error::ErrorNotFound(json!({
        "message": format!("{err}")
    }))
}

pub fn bad_request<T, F>(err: T, details: &[F]) -> error::Error
where
    T: Display,
    F: Serialize,
{
    error::ErrorBadRequest(json!({
        "message": format!("{err}"),
        "details": details
    }))
}
