use crate::config::AppState;
use crate::error::{Error, Result};
use axum::body::Body;
use axum::extract::State;
use axum::middleware::Next;
use axum::response::IntoResponse;
use http::Request;
use prometheus::{Counter, Encoder, Histogram, HistogramOpts, Registry, TextEncoder};
use std::time::Instant;

#[derive(Clone)]
pub struct Metrics {
    registry: Registry,
    http_requests_total: Counter,
    http_request_duration_seconds: Histogram,
}

impl Metrics {
    pub fn new() -> Result<Self, Error> {
        let registry = Registry::new();

        let http_requests_total = Counter::new("http_requests_total", "Total number of HTTP requests")?;
        let opts = HistogramOpts::new("http_request_duration_seconds", "HTTP request duration in seconds")
            .buckets(vec![0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0]);

        let http_request_duration_seconds = Histogram::with_opts(opts)?;

        registry.register(Box::new(http_requests_total.clone()))?;
        registry.register(Box::new(http_request_duration_seconds.clone()))?;

        Ok(Self {
            registry,
            http_requests_total,
            http_request_duration_seconds,
        })
    }
}

pub async fn metrics_handler(state: State<AppState>) -> impl IntoResponse {
    let encoder = TextEncoder::new();
    let mut buffer = vec![];
    encoder.encode(&state.metrics.registry.gather(), &mut buffer).unwrap();

    String::from_utf8(buffer).unwrap()
}

pub async fn metrics_middleware(state: State<AppState>, request: Request<Body>, next: Next) -> impl IntoResponse {
    let start = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();

    let response = next.run(request).await;

    let duration = start.elapsed().as_secs_f64();
    state.metrics.http_requests_total.inc();
    state.metrics.http_request_duration_seconds.observe(duration);

    tracing::info!(
        method = %method,
        uri = %uri,
        duration_seconds = duration,
        status = response.status().as_u16(),
        "Request completed"
    );

    response
}
