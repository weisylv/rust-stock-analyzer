📈 Rust Stock Market Data Analyzer

A high-performance, Rust-powered platform for ingesting, analyzing, and visualizing real-time stock market data.
Built with Rust, Kafka, TimescaleDB, React, and Kubernetes to showcase modern, production-grade system design.

🚀 Features

Real-Time Data Ingestion: Pulls live stock data from APIs (Polygon.io, Alpha Vantage, Yahoo Finance).

Streaming Architecture: Uses Kafka/Redpanda to buffer and stream tick-level data.

Time-Series Storage: Persists stock data in TimescaleDB for fast, scalable queries.

Analytics Engine (Rust): Calculates technical indicators (moving averages, RSI, Bollinger Bands).

REST + gRPC API: Query processed stock analytics programmatically.

Interactive Dashboard: React + D3.js frontend for visualizing market trends.

Cloud Native: Containerized with Docker and deployed on Kubernetes for scalability.

Optional AI Integration: Summarize daily stock trends in plain English via the OpenAI API.

🛠️ Tech Stack

Backend (Rust)

Tokio
 — Async runtime

Reqwest
 — API client

Actix-Web
 or Axum
 — Web framework

Kafka / Redpanda
 — Streaming pipeline

TimescaleDB
 — Time-series database

Frontend

React
 + TailwindCSS

D3.js
 — Interactive charts

Infrastructure

Docker
 — Containerization

Kubernetes
 — Orchestration

Grafana
 — Monitoring


## ⚡ Running the App: Demo Mode vs Live Mode

This project supports two modes of operation:

### 🟢 Demo Mode (default, public-safe)
- Uses sample stock data included in the repository (`data/sample_aapl.csv`).  
- Works **out-of-the-box** — no API key required.  
- Safe for public deployments (Kubernetes, Docker Compose, GitHub demos).  
- Great for quickly testing the ingestion → analytics → dashboard pipeline without external dependencies.  

### 🔵 Live Mode (optional, requires API key)
- Fetches **real-time stock data** from a provider (e.g., Finnhub, Alpha Vantage).  
- Requires a valid `STOCK_API_KEY` set in your `.env` file:  
  ```bash
  STOCK_API_KEY=your_api_key_here
  DEMO_MODE=false

## Note on Licensing
- This project is for personal and educational purposes. Live Mode is provided as an optional feature; if you choose to use it, ensure compliance with your chosen API provider’s terms of service.

📌 Demo Screenshots (to add later)

📈 Real-time stock chart (React + D3.js)

🧮 Indicator overlay (RSI, MA, Bollinger)

📊 Dashboard with portfolio simulator

☸️ Deploying with Kubernetes
Demo Mode Deployment (safe for public use)

The provided Kubernetes manifests default to Demo Mode, which means:

No external stock API is used.

The backend serves from bundled CSV demo data.

Safe to run on any cluster without licensing issues.

Example ConfigMap snippet:

apiVersion: v1
kind: ConfigMap
metadata:
  name: stock-analyzer-config
data:
  DEMO_MODE: "true"
  OLLAMA_HOST: "http://ollama:11434"

Live Mode Deployment (private use only)

If you want to use Live Mode in Kubernetes:

Set DEMO_MODE=false in your ConfigMap.

Store your stock API key as a Kubernetes Secret:

apiVersion: v1
kind: Secret
metadata:
  name: stock-analyzer-secrets
type: Opaque
stringData:
  STOCK_API_KEY: "your_api_key_here"


Mount this secret into your deployment as an environment variable.

⚠️ Important: Live Mode should only be used in private clusters (e.g., your local dev environment, private cloud). Do not expose Live Mode publicly, as redistributing stock data from free APIs may violate their terms of service.

🤝 Contributing

Contributions welcome! Open an issue or PR.

📜 License

MIT License.
