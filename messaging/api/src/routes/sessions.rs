use actix_web::{error, post, web, HttpRequest, HttpResponse, Responder};
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
