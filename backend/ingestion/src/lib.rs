use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};

// for live mode:
use reqwest;
// use serde::Deserialize;

/// Represents a single stock record (one time interval)
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StockRecord {
    pub timestamp: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
}

/// Loads demo stock data from a CSV file in the local data directory.
///
/// # Arguments
/// * `path` - The relative or absolute path to the CSV file.
///
/// # Example
/// ```
/// let data = load_demo_data("data/sample_aapl.csv").unwrap();
/// println!("Loaded {} rows", data.len());
/// ```
pub fn load_demo_data(path: &str) -> Result<Vec<StockRecord>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let mut records = Vec::new();
    for result in rdr.deserialize() {
        let record: StockRecord = result?;
        records.push(record);
    }

    Ok(records)
}


#[derive(Debug, Deserialize, Serialize)]
pub struct FinnhubCandle {
    pub c: Vec<f64>, // close prices
    pub h: Vec<f64>, // high prices
    pub l: Vec<f64>, // low prices
    pub o: Vec<f64>, // open prices
    pub s: String,   // status
    pub t: Vec<i64>, // timestamps
    pub v: Vec<u64>, // volumes
}

/// Fetch recent stock data from Finnhub (Live Mode)
pub async fn fetch_live_data(
    symbol: &str,
    api_key: &str,
) -> Result<serde_json::Value, Box<dyn Error>> {
    let url = format!(
        "https://finnhub.io/api/v1/stock/candle?symbol={}&resolution=D&count=10&token={}",
        symbol, api_key
    );

    let resp = reqwest::get(&url).await?;
    if resp.status() == reqwest::StatusCode::FORBIDDEN {
        // Fallback to /quote
        let quote_url = format!(
            "https://finnhub.io/api/v1/quote?symbol={}&token={}",
            symbol, api_key
        );
        let quote_resp = reqwest::get(&quote_url).await?;
        let quote_json: serde_json::Value = quote_resp.json().await?;

        return Ok(serde_json::json!({
            "mode": "live (free-tier)",
            "symbol": symbol,
            "quote": quote_json
        }));
    }
    if !resp.status().is_success() {
    return Err(format!("Finnhub API error: {}", resp.status()).into());
    }

    let candle_json: serde_json::Value = resp.json().await?;
    Ok(candle_json)
}
