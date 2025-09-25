use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main(){
    dotenv().ok();

    //Example: load stock API key
    let api_key = env::var("STOCK_API_KEY").unwrap_or("missing".to_string());

    println!("Ingestiong service starting...");
    println!("Using STOCK_API_KEY: {}", if api_key == "missing" {"NOT SET"} else {"SET"});

    // TODO: implement fetch from stock API
}