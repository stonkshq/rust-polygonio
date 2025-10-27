//! Example demonstrating the short interest fundamentals endpoint
//!
//! To run this example:
//! 1. Set your API key: export POLYGON_API_KEY=your_api_key_here
//! 2. Run: cargo run --example short_interest

use polygonio::{stocks::ShortInterestParams, PolygonClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Retrieve the API key from the environment for authentication
    let api_key =
        std::env::var("POLYGON_API_KEY").expect("Please set POLYGON_API_KEY environment variable");

    let client = PolygonClient::new(api_key);
    let stocks = client.stocks();

    println!("🔎 Checking short interest fundamentals for AAPL");

    // Configure the request to fetch the most recent short interest observations
    let mut params = ShortInterestParams::new("AAPL");
    params.limit = Some(5);
    params.sort = Some("settlement_date.desc".to_string());

    match stocks.short_interest(&params).await {
        Ok(response) => {
            println!("Request ID: {}", response.request_id);
            if let Some(results) = response.results {
                if results.is_empty() {
                    println!("No short interest records returned.");
                } else {
                    for record in results {
                        let date = record.settlement_date.as_deref().unwrap_or("unknown date");
                        let short_interest = record
                            .short_interest
                            .map(|value| value.to_string())
                            .unwrap_or_else(|| "n/a".to_string());
                        let days_to_cover = record
                            .days_to_cover
                            .map(|value| format!("{value:.2}"))
                            .unwrap_or_else(|| "n/a".to_string());
                        let avg_volume = record
                            .avg_daily_volume
                            .map(|value| value.to_string())
                            .unwrap_or_else(|| "n/a".to_string());

                        println!(
                            "• {date}: short_interest={short_interest} days_to_cover={days_to_cover} avg_daily_volume={avg_volume}"
                        );
                    }
                }
                if let Some(next_url) = response.next_url {
                    println!("More data available: {}", next_url);
                }
            } else {
                println!("Response contained no results field.");
            }
        }
        Err(err) => {
            eprintln!("Failed to retrieve short interest data: {err}");
        }
    }

    Ok(())
}
