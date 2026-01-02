use actix_web::{dev::ServiceResponse, http::header, middleware::ErrorHandlerResponse, Result};

pub fn add_error_header<B>(mut res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );

    Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
}
