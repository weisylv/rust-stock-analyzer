use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use serde::Deserialize;

// for live mode:
use reqwest;
// use serde::Deserialize;

/// Represents a single stock record (one time interval)
#[derive(Debug, Deserialize, Clone)]
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


#[derive(Debug, Deserialize)]
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
) -> Result<FinnhubCandle, Box<dyn Error>> {
    // Example: 5-minute candles, last 10 bars
    let url = format!(
        "https://finnhub.io/api/v1/stock/candle?symbol={}&resolution=5&count=10&token={}",
        symbol, api_key
    );

    let response = reqwest::get(&url).await?;
    if !response.status().is_success() {
        return Err(format!("Finnhub API error: {}", response.status()).into());
    }

    let candle = response.json::<FinnhubCandle>().await?;
    if candle.s != "ok" {
        return Err("No data returned from Finnhub".into());
    }

    Ok(candle)
}
