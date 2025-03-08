mod api;
mod auth;
mod config;
mod entities;
mod error;
mod metrics;
mod utils;

use axum::routing::get;
use axum::{middleware, Router};
use config::CONFIG;

use crate::api::requests::*;
use crate::auth::*;
use crate::utils::*;
use crate::config::AppState;
use crate::error::{Error, Result};
use crate::metrics::{metrics_handler, metrics_middleware, Metrics};
use axum::http;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let log_level = format!("sqlx=warn,{}", CONFIG.log_level);
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(log_level)
        .init();

    let pool = init_db(&CONFIG.db_url).await?;
    let metrics = Metrics::new()?;
    let state = AppState { db: pool, metrics };

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/last-update-time", get(last_update_time))
        .route("/company-list", get(company_list))
        .route("/two-day-diff", get(two_day_diff))
        .route("/stock-sector-breadth", get(stock_sector_breadth))
        .route("/stock-sector-breadth-trend", get(stock_sector_breadth_trend))
        .route("/factor-relative-strength", get(factor_relative_strength))
        .route("/factor-sector-std", get(factor_sector_std))
        .route("/metrics", get(metrics_handler))
        .with_state(state.clone())
        .layer(middleware::from_fn_with_state(state.clone(), metrics_middleware))
        .layer(
            TraceLayer::new_for_http()
                .on_failure(|_error, _latency, _span: &tracing::Span| {
                    // NOTE: Do nothing, trace detail errors in error.rs
                })
                .make_span_with(|request: &http::Request<_>| {
                    tracing::info_span!(
                        "HTTP request",
                        method = %request.method(),
                        uri = %request.uri(),
                        headers = ?request.headers(),
                    )
                }),
        )
        .layer(middleware::from_fn(auth_middleware));

    let listener = TcpListener::bind("0.0.0.0:3001").await?;
    tracing::info!("Starting server on {}", listener.local_addr()?);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("server failed to start");

    Ok(())
}
