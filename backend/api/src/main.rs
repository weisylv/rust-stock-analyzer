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
use reqwest::Client;

// --- Struct for /health response ---
#[derive(Serialize)]
struct HealthCheck {
    status: String,
    stock_api_key_set: bool,
    ollama_host_set: bool,
}

// --- /health handler ---
async fn health() -> Json<HealthCheck> {
    Json(HealthCheck {
        status: "OK".to_string(),
        stock_api_key_set: env::var("STOCK_API_KEY").is_ok(),
        ollama_host_set: env::var("OLLAMA_HOST").is_ok(),
    })
}

// --- /health/ollama ---
#[derive(Serialize)]
struct OllamaHealth {
    reachable: bool,
    host: String,
}

async fn health_ollama() -> Json<OllamaHealth> {
    let host = env::var("OLLAMA_HOST").unwrap_or("http://localhost:11434".to_string());
    let url = format!("{}/api/tags", host); // /api/tags lists available models

    let client = Client::new();
    let reachable = client.get(&url).send().await.is_ok();

    Json(OllamaHealth { reachable, host })
}

#[tokio::main]
async fn main() {
    // Load .env (if present). If .env doesn't exist, dotenv().ok() will be fine.
    dotenv().ok();

    // Default Ollama host if not set
    let ollama_host = env::var("OLLAMA_HOST")
        .unwrap_or("http://localhost:11434".to_string());

    println!("✅ Server starting...");
    println!("Ollama host: {}", ollama_host);

    // Build the app router
    let app = Router::new()
        .route("/health", get(health))
        .route("/health/ollama", get(health_ollama));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("✅ Server running on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
