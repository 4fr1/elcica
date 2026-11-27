use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use serde_json::{json, Value};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;

/// Endpoint ringan untuk keep-alive (tidak butuh DB).
async fn root() -> Json<Value> {
    Json(json!({
        "app": env!("CARGO_PKG_NAME"),
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

/// Endpoint yang diping Cloudflare Cron tiap 10 menit.
/// Query SELECT 1 juga menjaga Neon tetap aktif (scale-to-zero 5 menit).
async fn health(State(pool): State<Option<sqlx::PgPool>>) -> (StatusCode, Json<Value>) {
    let db_ok = match &pool {
        Some(p) => sqlx::query("SELECT 1").fetch_one(p).await.is_ok(),
        None => false,
    };

    let code = if db_ok {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    (
        code,
        Json(json!({
            "status": if db_ok { "ok" } else { "degraded" },
            "db": db_ok,
        })),
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    // DB opsional supaya `cargo run` tetap jalan tanpa Neon.
    let pool = match std::env::var("DATABASE_URL") {
        Ok(url) => match PgPoolOptions::new().max_connections(5).connect(&url).await {
            Ok(p) => Some(p),
            Err(e) => {
                tracing::warn!(%e, "gagal konek Neon, /health akan degraded");
                None
            }
        },
        Err(_) => {
            tracing::warn!("DATABASE_URL tidak diset, /health akan degraded");
            None
        }
    };

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .with_state(pool);

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    tracing::info!("listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
