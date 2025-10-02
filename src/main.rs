// This file is only used for examples and testing
// The main library code is in lib.rs

use polygonio::{PolygonClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Example usage of the Polygon.io client
    let api_key =
        std::env::var("POLYGON_API_KEY").unwrap_or_else(|_| "your-api-key-here".to_string());

    let client = PolygonClient::new(api_key);
    let _stocks = client.stocks();

    println!("Polygon.io Rust client initialized successfully!");
    println!("Try setting POLYGON_API_KEY environment variable and use the stocks client.");

    Ok(())
}
