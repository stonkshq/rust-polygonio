//! Example usage of the Polygon.io Rust client
//!
//! To run this example:
//! 1. Set your API key: export POLYGON_API_KEY=your_api_key_here
//! 2. Run: cargo run --example basic_usage

use polygonio::{PolygonClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Get API key from environment variable
    let api_key =
        std::env::var("POLYGON_API_KEY").expect("Please set POLYGON_API_KEY environment variable");

    // Create the client
    let client = PolygonClient::new(api_key);
    let stocks = client.stocks();

    println!("🔗 Connected to Polygon.io API");

    // Example 1: Get ticker details
    println!("\n📈 Fetching ticker details for AAPL...");
    match stocks.ticker_details("AAPL").await {
        Ok(response) => {
            if let Some(ticker) = response.results {
                println!("  • Company: {}", ticker.name.unwrap_or("N/A".to_string()));
                println!("  • Market: {}", ticker.market.unwrap_or("N/A".to_string()));
                println!("  • Active: {}", ticker.active.unwrap_or(false));
                if let Some(market_cap) = ticker.market_cap {
                    println!("  • Market Cap: ${:.2}B", market_cap / 1_000_000_000.0);
                }
            }
        }
        Err(e) => println!("  ❌ Error fetching ticker details: {}", e),
    }

    // Example 2: Get market status
    println!("\n🏢 Fetching current market status...");
    match stocks.market_status().await {
        Ok(status) => {
            println!("  • Market Status: {}", status.market);
            println!("  • Server Time: {}", status.server_time);
            println!("  • After Hours: {}", status.after_hours);
            println!("  • Early Hours: {}", status.early_hours);
            if let Some(exchanges) = &status.exchanges {
                println!(
                    "  • Exchanges: {}",
                    serde_json::to_string_pretty(exchanges).unwrap_or_default()
                );
            }
        }
        Err(e) => println!("  ❌ Error fetching market status: {}", e),
    }

    // Example 3: Get aggregates (OHLCV data)
    println!("\n📊 Fetching daily aggregates for AAPL (last 5 days)...");
    let from_date = chrono::Utc::now().date_naive() - chrono::Duration::days(10);
    let to_date = chrono::Utc::now().date_naive() - chrono::Duration::days(1);

    match stocks
        .aggregates(
            "AAPL",
            1,
            "day",
            &from_date.format("%Y-%m-%d").to_string(),
            &to_date.format("%Y-%m-%d").to_string(),
            None,
        )
        .await
    {
        Ok(response) => {
            if let Some(results) = response.results {
                println!("  • Found {} trading days", results.len());
                for (i, bar) in results.iter().take(5).enumerate() {
                    if let (Some(open), Some(high), Some(low), Some(close), Some(volume)) =
                        (bar.open, bar.high, bar.low, bar.close, bar.volume)
                    {
                        println!(
                            "  • Day {}: O=${:.2} H=${:.2} L=${:.2} C=${:.2} V={:.0}",
                            i + 1,
                            open,
                            high,
                            low,
                            close,
                            volume
                        );
                    }
                }
            }
        }
        Err(e) => println!("  ❌ Error fetching aggregates: {}", e),
    }

    // Example 4: Get snapshot data
    println!("\n📸 Fetching snapshot for AAPL...");
    match stocks.ticker_snapshot("AAPL").await {
        Ok(response) => {
            let snapshot = &response.ticker;
            println!("  • Ticker: {}", snapshot.ticker);

            if let Some(change) = snapshot.todays_change {
                println!("  • Today's Change: ${:.2}", change);
            }
            if let Some(change_perc) = snapshot.todays_change_perc {
                println!("  • Today's Change %: {:.2}%", change_perc);
            }

            if let Some(last_trade) = &snapshot.last_trade {
                println!("  • Last Trade Price: ${:.2}", last_trade.price);
                println!("  • Last Trade Size: {}", last_trade.size);
            }

            if let Some(last_quote) = &snapshot.last_quote {
                println!(
                    "  • Bid: ${:.2} (size: {})",
                    last_quote.bid_price, last_quote.bid_size
                );
                println!(
                    "  • Ask: ${:.2} (size: {})",
                    last_quote.ask_price, last_quote.ask_size
                );
            }

            if let Some(day) = &snapshot.day {
                println!(
                    "  • Day Range: ${:.2} - ${:.2} (Open: ${:.2}, Close: ${:.2})",
                    day.low, day.high, day.open, day.close
                );
            }
        }
        Err(e) => println!("  ❌ Error fetching snapshot: {}", e),
    }

    println!("\n✅ Examples completed successfully!");
    println!("💡 Tip: Check out the other asset classes too:");
    println!("   • client.options() - for options data");
    println!("   • client.futures() - for futures data");
    println!("   • client.forex() - for forex data");
    println!("   • client.crypto() - for crypto data");
    println!("   • client.indices() - for indices data");

    Ok(())
}
