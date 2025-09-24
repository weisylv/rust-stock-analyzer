use axum::{
    routing::get,
    Router,
    response::Json,
};
use serde::Serialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use std::env;
use dotenvy::dotenv;

// --- Struct for /health response ---
#[derive(Serialize)]
struct HealthCheck {
    status: String,
    // Optional: expose a flag that env loaded (do NOT put keys here)
    env_loaded: bool,
}

// --- /health handler ---
async fn health() -> Json<HealthCheck> {
    Json(HealthCheck {
        status: "OK".to_string(),
        env_loaded: env::var("STOCK_API_KEY").is_ok(),
    })
}

#[tokio::main]
async fn main() {
    // Load .env (if present). If .env doesn't exist, dotenv().ok() will be fine.
    dotenv().ok();

    // Example: read and log whether keys exist (don't print keys!)
    let stock_api_key = env::var("STOCK_API_KEY").ok();
    let openai_api_key = env::var("OPENAI_API_KEY").ok();

    println!("✅ Server starting...");
    match &stock_api_key {
        Some(_) => println!("STOCK_API_KEY is set."),
        None => println!("STOCK_API_KEY is NOT set. See .env or environment variables."),
    }
    match &openai_api_key {
        Some(_) => println!("OPENAI_API_KEY is set."),
        None => println!("OPENAI_API_KEY is NOT set."),
    }

    // Build the app router
    let app = Router::new()
        .route("/health", get(health));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("✅ Server running on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
