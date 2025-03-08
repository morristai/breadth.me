use crate::metrics::Metrics;
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use sea_orm::DatabaseConnection;
use std::env;
use std::sync::Arc;

pub struct Config {
    pub log_level: String,
    pub db_url: String,
    pub jwt_secret: String,
}

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub metrics: Metrics,
}

pub static CONFIG: Lazy<Arc<Config>> = Lazy::new(|| {
    if dotenv().is_err() {
        tracing::info!("No .env file found");
    }

    let log_level = env::var("log_level").unwrap_or_else(|_| "info".to_string());
    let db_host = env::var("db_host").expect("db_host must be set");
    let db_port = env::var("db_port").expect("db_port must be set");
    let db_port = db_port.parse::<u16>().expect("db_port must be a number");
    let db_user = env::var("db_user").expect("db_user must be set");
    let db_password = env::var("db_password").expect("db_password must be set");
    let db_name = env::var("db_name").expect("db_name must be set");
    let db_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        db_user, db_password, db_host, db_port, db_name
    );
    let jwt_secret = env::var("jwt_secret_key").expect("jwt_secret must be set");

    Arc::new(Config {
        log_level,
        db_url,
        jwt_secret,
    })
});
