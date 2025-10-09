use ingestion::fetch_live_data;
use reqwest;
use serde::Deserialize;

/// Fallback structure for the /quote endpoint (always available on free tier)
#[derive(Debug, Deserialize)]
pub struct Quote {
    pub c: f64, // current price
    pub h: f64, // high price of the day
    pub l: f64, // low price of the day
    pub o: f64, // open price of the day
    pub pc: f64, // previous close price
}

#[tokio::main]
async fn main() {
    let api_key = std::env::var("STOCK_API_KEY")
        .expect("Please set STOCK_API_KEY in your .env or environment");
    let symbol = "AAPL";

    println!("Fetching {} data from Finnhub...", symbol);

    // Try candle endpoint first (daily data works on free tier)
    match fetch_live_data(symbol, &api_key).await {
        Ok(candle) => {
            println!("✅ Candle data fetched successfully!");
            println!("Fetched {} candles", candle.c.len());
            if !candle.c.is_empty() {
                println!("First few closes: {:?}", &candle.c[..3.min(candle.c.len())]);
            }
        }
        Err(e) => {
            println!("⚠️ Candle fetch failed: {}", e);
            println!("Attempting fallback to /quote endpoint...");

            // Fallback to /quote (always allowed)
            let quote_url = format!(
                "https://finnhub.io/api/v1/quote?symbol={}&token={}",
                symbol, api_key
            );

            match reqwest::get(&quote_url).await {
                Ok(resp) => {
                    if resp.status().is_success() {
                        let quote: Quote = resp.json().await.unwrap();
                        println!("✅ Quote data fetched successfully!");
                        println!("Current price: {}", quote.c);
                        println!("Open: {}, High: {}, Low: {}, Prev Close: {}", quote.o, quote.h, quote.l, quote.pc);
                    } else {
                        println!("❌ Fallback /quote request failed with status: {}", resp.status());
                    }
                }
                Err(err) => println!("❌ Failed to call /quote endpoint: {}", err),
            }
        }
    }
}
