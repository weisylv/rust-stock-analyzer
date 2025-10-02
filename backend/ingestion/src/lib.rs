use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use serde::Deserialize;

// for live mode:
use reqwest;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct StockRecord {
    pub timestamp: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
}

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
    c: Vec<f64>, // close prices
    h: Vec<f64>, // high prices
    l: Vec<f64>, // low prices
    o: Vec<f64>, // open prices
    s: String,   // status
    t: Vec<i64>, // timestamps
    v: Vec<u64>, // volumes
}

pub async fn fetch_live_data(symbol: &str, api_key: &str) -> Result<FinnhubCandle, Box<dyn Error>> {
    let url = format!(
        "https://finnhub.io/api/v1/stock/candle?symbol={}&resolution=5&count=10&token={}",
        symbol, api_key
    );

    let resp = reqwest::get(&url).await?.json::<FinnhubCandle>().await?;
    Ok(resp)
}
