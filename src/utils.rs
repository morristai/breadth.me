use crate::config::CONFIG;
use crate::error::{Error, Result};
use axum::body::{to_bytes, Body, Bytes};
use axum::extract::Request;
use axum::middleware::Next;
use sea_orm::{Database, DatabaseConnection};
use tokio::signal;

pub async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

pub async fn init_db(url: &String) -> Result<DatabaseConnection, Error> {
    tracing::info!("Connecting to MySQL database at {}...", CONFIG.db_url);

    let connect = Database::connect(url).await?;
    Ok(connect)
}

pub async fn log_request(req: Request<Body>, next: Next) -> axum::response::Response {
    tracing::info!(
        "Incoming request: method={}, uri={}, headers={:?}",
        req.method(),
        req.uri(),
        req.headers()
    );

    let (parts, body) = req.into_parts();
    let body_bytes = to_bytes(body, 1024 * 1024).await.unwrap_or_else(|e| {
        tracing::warn!("Failed to read request body: {}", e);
        Bytes::new()
    });
    if !body_bytes.is_empty() {
        tracing::info!("Request body: {:?}", String::from_utf8_lossy(&body_bytes));
    }

    // Reconstruct the request to pass to the next handler
    let req = Request::from_parts(parts, Body::from(body_bytes));

    let response = next.run(req).await;
    tracing::debug!("Request processed, responding...");
    response
}
