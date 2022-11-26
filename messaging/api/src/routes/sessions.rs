use actix_web::{delete, error, post, web, HttpRequest, HttpResponse, Responder};
use infrastructure::pg;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct StartSessionPath {
    pub user_id: String,
}

#[tracing::instrument(skip(pool))]
#[post("/users/{user_id}/sessions")]
async fn start_session(
    req: HttpRequest,
    path: web::Path<StartSessionPath>,
    pool: web::Data<deadpool_postgres::Pool>,
) -> Result<impl Responder, actix_web::Error> {
    let session_repository = pg::PgSessionRepository::new(&pool);
    let connection_info = req.connection_info();
    let server_addr =
        connection_info
            .realip_remote_addr()
            .ok_or(actix_web::error::ErrorInternalServerError(
                "could not find the server address",
            ))?;

    session_repository
        .add(path.user_id.clone(), server_addr.to_string())
        .await
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok())
}

#[derive(Debug, Deserialize)]
struct DeleteSessionPath {
    pub user_id: String,
    pub session_id: String,
}

#[tracing::instrument(skip(pool))]
#[delete("/users/{user_id}/sessions/{session_id}")]
async fn delete_session(
    req: HttpRequest,
    path: web::Path<DeleteSessionPath>,
    pool: web::Data<deadpool_postgres::Pool>,
) -> Result<impl Responder, actix_web::Error> {
    let session_repository = pg::PgSessionRepository::new(&pool);

    session_repository
        .remove(path.user_id.clone(), path.session_id.clone())
        .await
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok())
}
