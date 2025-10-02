//! Example: Fetching market snapshots for multiple tickers
//!
//! This example demonstrates how to fetch market snapshots for multiple tickers
//! and display comprehensive market data including prices, changes, and spreads.

use polygonio::{PolygonClient, Result};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Get API key from environment variable
    let api_key = env::var("POLYGON_API_KEY").map_err(|_| {
        polygonio::PolygonError::authentication("Please set POLYGON_API_KEY environment variable")
    })?;

    // Create the client
    let client = PolygonClient::new(api_key);
    let stocks = client.stocks();

    println!("🔗 Connected to Polygon.io API");
    println!();

    // Fetch snapshots for all tickers
    println!("📸 Fetching snapshots for all tickers...");
    match stocks.all_tickers_snapshot(None).await {
        Ok(response) => {
            println!("✅ Response status: {}", response.status);
            if let Some(count) = response.count {
                println!("✅ Found {} total snapshots", count);
            }

            if let Some(snapshots) = response.tickers {
                println!();
                println!("📊 First 10 ticker snapshots:");
                println!("{}", "=".repeat(80));

                // Display first 10 snapshots
                for (index, snapshot) in snapshots.iter().take(10).enumerate() {
                    println!(
                        "{}. {} - {}",
                        index + 1,
                        snapshot.ticker,
                        format_snapshot_summary(&snapshot)
                    );

                    // Show today's performance
                    if let (Some(change), Some(change_perc)) =
                        (snapshot.todays_change, snapshot.todays_change_perc)
                    {
                        let direction = if change >= 0.0 { "📈" } else { "📉" };
                        println!(
                            "   {} Change: ${:.2} ({:.2}%)",
                            direction, change, change_perc
                        );
                    }

                    // Show last trade info if available
                    if let Some(last_trade) = &snapshot.last_trade {
                        println!(
                            "   💰 Last Trade: ${:.2} (size: {})",
                            last_trade.price, last_trade.size
                        );
                    }

                    // Show bid/ask spread if available
                    if let Some(last_quote) = &snapshot.last_quote {
                        let spread = last_quote.ask_price - last_quote.bid_price;
                        println!(
                            "   📋 Bid/Ask: ${:.2}/${:.2} (spread: ${:.2})",
                            last_quote.bid_price, last_quote.ask_price, spread
                        );
                    }

                    // Show day range if available
                    if let Some(day) = &snapshot.day {
                        println!(
                            "   📏 Day Range: ${:.2} - ${:.2} (Vol: {:.0})",
                            day.low, day.high, day.volume
                        );
                    }

                    println!();
                }

                println!("💡 Total snapshots returned: {}", snapshots.len());
                println!("💡 Use the `ticker_snapshot()` method to get detailed data for a specific ticker");
            } else {
                println!("❌ No ticker data returned");
            }
        }
        Err(e) => {
            println!("❌ Error fetching snapshots: {}", e);
        }
    }

    println!();
    println!("✅ Example completed successfully!");
    println!();
    println!("💡 Key insights from this example:");
    println!("   • The all_tickers_snapshot() endpoint provides bulk access to market data");
    println!(
        "   • Snapshot data includes: price changes, bid/ask spreads, volume, and daily ranges"
    );
    println!("   • This is perfect for building market dashboards or screening tools");
    println!("   • Use individual ticker_snapshot() calls when you need data for specific symbols");

    Ok(())
}

/// Format a summary line for a ticker snapshot
fn format_snapshot_summary(snapshot: &polygonio::stocks::TickerSnapshot) -> String {
    // Try to get the most recent price from various sources
    let current_price = snapshot
        .last_trade
        .as_ref()
        .map(|trade| trade.price)
        .or_else(|| snapshot.day.as_ref().map(|day| day.close))
        .or_else(|| snapshot.prev_day.as_ref().map(|prev| prev.close));

    match current_price {
        Some(price) => format!("${:.2}", price),
        None => "Price N/A".to_string(),
    }
}
