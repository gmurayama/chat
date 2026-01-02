use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::Error;
use tracing::Span;
use tracing_actix_web::{DefaultRootSpanBuilder, Level, RootSpanBuilder, TracingLogger};

pub struct LevelRootSpanBuilder;

impl RootSpanBuilder for LevelRootSpanBuilder {
    fn on_request_start(request: &ServiceRequest) -> Span {
        let level = match request.path() {
            "/healthcheck" | "/metrics" => Level::DEBUG,
            _ => Level::INFO,
        };
        tracing_actix_web::root_span!(level = level, request)
    }

    fn on_request_end<B: actix_web::body::MessageBody>(
        span: Span,
        outcome: &Result<ServiceResponse<B>, Error>,
    ) {
        DefaultRootSpanBuilder::on_request_end(span, outcome);
    }
}

pub struct Tracing;

impl Tracing {
    pub fn middleware() -> TracingLogger<LevelRootSpanBuilder> {
        TracingLogger::<LevelRootSpanBuilder>::new()
    }
}
