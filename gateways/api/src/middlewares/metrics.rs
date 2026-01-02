use std::{
    future::{ready, Ready},
    sync::Arc,
    time::Instant,
};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;
use prometheus_client::{
    encoding::EncodeLabelSet,
    metrics::{counter::Counter, family::Family, histogram::Histogram},
    registry::Registry,
};

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct RequestLabel {
    pub method: String,
    pub path: String,
    pub status: u16,
}

#[derive(Clone)]
pub struct Metrics {
    request_duration: Family<RequestLabel, Histogram>,
    request_count: Family<RequestLabel, Counter>,
}

impl Metrics {
    pub fn new(registry: &mut Registry) -> Self {
        let request_count = Family::<RequestLabel, Counter>::default();
        let request_duration = Family::<RequestLabel, Histogram>::new_with_constructor(|| {
            let buckets = [
                1.0, 2.5, 5.0, 10.0, 25.0, 50.0, 75.0, 100.0, 250.0, 500.0, 750.0, 1000.0, 1250.0,
                1500.0, 2000.0,
            ];
            Histogram::new(buckets.into_iter())
        });

        registry.register(
            "request_count",
            "Number of requests received",
            request_count.clone(),
        );

        registry.register(
            "request_duration_ms",
            "Request duration",
            request_duration.clone(),
        );

        Metrics {
            request_duration,
            request_count,
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for Metrics
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = MetricsMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(MetricsMiddleware {
            service,
            request_duration: Arc::new(self.request_duration.clone()),
            request_count: Arc::new(self.request_count.clone()),
        }))
    }
}

pub struct MetricsMiddleware<S> {
    service: S,
    request_duration: Arc<Family<RequestLabel, Histogram>>,
    request_count: Arc<Family<RequestLabel, Counter>>,
}

impl<S, B> Service<ServiceRequest> for MetricsMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let now = Instant::now();
        let path = req.path().to_string();
        let method = req.method().to_string();

        let fut = self.service.call(req);

        if path == "/metrics" {
            return Box::pin(fut);
        }

        let request_duration = self.request_duration.clone();
        let request_count = self.request_count.clone();

        Box::pin(async move {
            let res = fut.await?;

            let elapsed = now.elapsed().as_millis() as f64;

            request_duration
                .get_or_create(&RequestLabel {
                    method: method.clone(),
                    path: path.clone(),
                    status: res.status().as_u16(),
                })
                .observe(elapsed);

            request_count
                .get_or_create(&RequestLabel {
                    method,
                    path,
                    status: res.status().as_u16(),
                })
                .inc();

            Ok(res)
        })
    }
}
