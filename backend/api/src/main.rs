use axum::{
    routing::get,
    response::IntoResponse,
    Json, Router,
};
use dotenvy::dotenv;
use std::env;
use serde_json::json;
use std::net::SocketAddr;

// Import functions from your ingestion crate
use ingestion::{load_demo_data, fetch_live_data};

#[tokio::main]
async fn main() {
    dotenvy::from_filename("../../.env").ok();

    let demo_mode = env::var("DEMO_MODE")
        .unwrap_or_else(|_| "true".to_string()) == "true";
    println!("Running in Demo Mode? {}", demo_mode);

    let app = Router::new()
        .route("/stocks/demo", get(get_demo_data))
        .route("/stocks/live", get(get_live_data))
        .route("/health", get(health_check));

    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    println!("🚀 Server running at http://{}/", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// Health check endpoint showing current mode
async fn health_check() -> impl IntoResponse {
    let demo_mode = env::var("DEMO_MODE")
        .unwrap_or_else(|_| "true".to_string()) == "true";
    let mode = if demo_mode { "demo" } else { "live" };

    Json(json!({
        "status": "ok",
        "mode": mode,
    }))
}

/// Demo Mode endpoint: reads local CSV
async fn get_demo_data() -> impl IntoResponse {
    match load_demo_data("../../data/sample_aapl.csv") {
        Ok(data) => Json(json!({
            "mode": "demo",
            "symbol": "AAPL",
            "records": data
        })),
        Err(e) => Json(json!({
            "error": format!("Failed to load demo data: {}", e)
        })),
    }
}

/// Live Mode endpoint: calls Finnhub API or fallback
async fn get_live_data() -> impl IntoResponse {
    let api_key = env::var("STOCK_API_KEY").unwrap_or_default();
    let symbol = "AAPL";

    if api_key.is_empty() {
        return Json(json!({
            "error": "Missing STOCK_API_KEY in .env"
        }));
    }

    match fetch_live_data(symbol, &api_key).await {
        Ok(data) => Json(data),
        Err(e) => Json(json!({"error": e.to_string()})),
    }
}
