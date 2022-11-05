use actix_web::post;

#[tracing::instrument()]
#[post("/messages")]
async fn route_message() -> actix_web::Result<String> {
    return Ok(String::new());
}
