use axum::{
    routing::get,
    Router,
    response::Json,
};
use serde::Serialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[derive(Serialize)]
struct HealthCheck {
    status: String,
}

async fn health() -> Json<HealthCheck> {
    Json(HealthCheck {
        status: "OK".to_string(),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("✅ Server running on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
