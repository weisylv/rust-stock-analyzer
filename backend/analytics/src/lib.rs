#![allow(dead_code)]

#[derive(Debug)]
struct MovingAverageResult {
    symbol: String,
    ma: f64,
}

fn simple_moving_average(prices: &[f64], period: usize) -> Option<f64> {
    if prices.len() < period {
        return None;
    }
    let sum: f64 = prices[prices.len()-period..].iter().sum();
    Some(sum / period as f64)
}

fn main() {
    println!("Analytics service starting...");

    let prices = vec![100.0, 101.5, 102.0, 103.0, 104.5];
    if let Some(ma) = simple_moving_average(&prices, 3) {
        let result = MovingAverageResult {
            symbol: "AAPL".to_string(),
            ma,
        };
        println!("Computed SMA: {:?}", result);
    } else {
        println!("Not enough data for SMA.");
    }
}
