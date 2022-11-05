use actix_web::post;

#[post("/messages")]
async fn route_message() -> actix_web::Result<String> {
    return Ok(String::new());
}
