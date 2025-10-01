//! Example: Fetching custom OHLC bars for stock analysis
//! 
//! This example demonstrates how to fetch custom aggregated    match stocks.aggregates("AAPL", 15, "minute", "2024-09-30", "2024-09-30", Some(minute_params)).await {bars (OHLC data) 
//! for different time intervals and use them for basic technical analysis.

use polygon_io::{PolygonClient, Result, stocks::AggregatesParams};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Get API key from environment variable
    let api_key = env::var("POLYGON_API_KEY")
        .map_err(|_| polygon_io::PolygonError::authentication("Please set POLYGON_API_KEY environment variable"))?;

    // Create the client
    let client = PolygonClient::new(api_key);
    let stocks = client.stocks();

    println!("🔗 Connected to Polygon.io API");
    println!();

    // Example 1: Get 5-minute bars for AAPL
    println!("📊 Example 1: 5-minute bars for AAPL (last 5 trading days)");
    println!("{}", "=".repeat(60));
    
    let params = AggregatesParams {
        adjusted: Some(true),
        sort: Some("asc".to_string()),
        limit: Some(50), // Limit to first 50 bars
    };

    match stocks.aggregates("AAPL", 5, "minute", "2024-09-26", "2024-09-30", Some(params)).await {
        Ok(response) => {
            println!("✅ Status: {}", response.status);
            if let Some(ticker) = &response.ticker {
                println!("📈 Ticker: {}", ticker);
            }
            if let Some(adjusted) = response.adjusted {
                println!("🔧 Adjusted for splits: {}", adjusted);
            }
            if let Some(count) = response.results_count {
                println!("📊 Total bars returned: {}", count);
            }

            if let Some(bars) = response.results {
                println!("\n🕐 First 10 bars (5-minute intervals):");
                for (i, bar) in bars.iter().take(10).enumerate() {
                    if let (Some(timestamp), Some(open), Some(high), Some(low), Some(close), Some(volume)) = 
                        (bar.timestamp, bar.open, bar.high, bar.low, bar.close, bar.volume) {
                        
                        let datetime = chrono::DateTime::from_timestamp_millis(timestamp)
                            .unwrap_or_default()
                            .format("%Y-%m-%d %H:%M:%S");
                        
                        println!("{}. {} | O: ${:.2} H: ${:.2} L: ${:.2} C: ${:.2} | Vol: {:.0}", 
                            i + 1, datetime, open, high, low, close, volume);
                    }
                }

                // Calculate some basic statistics
                calculate_basic_stats(&bars, "5-minute");
            }
        }
        Err(e) => {
            println!("❌ Error fetching 5-minute bars: {}", e);
        }
    }

    println!("\n{}", "=".repeat(60));

    // Example 2: Get daily bars for broader analysis
    println!("📊 Example 2: Daily bars for TSLA (last 30 days)");
    println!("{}", "=".repeat(60));
    
    let daily_params = AggregatesParams {
        adjusted: Some(true),
        sort: Some("desc".to_string()), // Most recent first
        limit: Some(30),
    };

    match stocks.aggregates("TSLA", 1, "day", "2024-08-01", "2024-09-30", Some(daily_params)).await {
        Ok(response) => {
            println!("✅ Status: {}", response.status);
            
            if let Some(bars) = response.results {
                println!("📈 Last 10 trading days for TSLA:");
                for (i, bar) in bars.iter().take(10).enumerate() {
                    if let (Some(timestamp), Some(open), Some(high), Some(low), Some(close), Some(volume)) = 
                        (bar.timestamp, bar.open, bar.high, bar.low, bar.close, bar.volume) {
                        
                        let date = chrono::DateTime::from_timestamp_millis(timestamp)
                            .unwrap_or_default()
                            .format("%Y-%m-%d");
                        
                        let daily_change = close - open;
                        let daily_change_pct = (daily_change / open) * 100.0;
                        let direction = if daily_change >= 0.0 { "📈" } else { "📉" };
                        
                        println!("{}. {} | ${:.2} → ${:.2} {} {:.2}% | Range: ${:.2}-${:.2} | Vol: {:.0}M", 
                            i + 1, date, open, close, direction, daily_change_pct, low, high, volume / 1_000_000.0);
                    }
                }

                calculate_basic_stats(&bars, "daily");
            }
        }
        Err(e) => {
            println!("❌ Error fetching daily bars: {}", e);
        }
    }

    println!("\n{}", "=".repeat(60));

    // Example 3: Get 15-minute bars for intraday analysis  
    println!("📊 Example 3: 15-minute bars for AAPL (recent trading session)");
    println!("{}", "=".repeat(60));
    
    let minute_params = AggregatesParams {
        adjusted: Some(true),
        sort: Some("asc".to_string()), // Chronological order
        limit: Some(20), // Ask for more bars
    };

    match stocks.aggregates("AAPL", 15, "minute", "2024-09-30", "2024-10-01", Some(minute_params)).await {
        Ok(response) => {
            println!("✅ Status: {}", response.status);
            if let Some(count) = response.results_count {
                println!("📊 Total bars returned: {}", count);
            }
            
            if let Some(bars) = response.results {
                if bars.is_empty() {
                    println!("⚠️  No hourly data available for the requested time period");
                } else {
                    println!("📈 15-minute bars for AAPL:");
                    for (i, bar) in bars.iter().enumerate() {
                        if let (Some(timestamp), Some(open), Some(close), Some(volume)) = 
                            (bar.timestamp, bar.open, bar.close, bar.volume) {
                            
                            let datetime = chrono::DateTime::from_timestamp_millis(timestamp)
                                .unwrap_or_default()
                                .format("%m/%d %H:%M");
                            
                            let change_15min = ((close - open) / open) * 100.0;
                            let direction = if change_15min >= 0.0 { "📈" } else { "📉" };
                            
                            println!("{}. {} | ${:.2} → ${:.2} {} {:.2}% | Vol: {:.0}", 
                                i + 1, datetime, open, close, direction, change_15min, volume);
                        }
                    }
                }
            } else {
                println!("⚠️  No results field in response");
            }
        }
        Err(e) => {
            println!("❌ Error fetching hourly bars: {}", e);
        }
    }

    println!();
    println!("✅ Custom bars examples completed successfully!");
    println!();
    println!("💡 Key insights from custom bars:");
    println!("   • Use different timeframes (minute, hour, day) for different analysis needs");
    println!("   • Adjust the multiplier to get custom intervals (5-minute, 15-minute, etc.)");
    println!("   • Set adjusted=true to account for stock splits and dividends");
    println!("   • Use the limit parameter to control the number of bars returned");
    println!("   • Sort by 'asc' for chronological order, 'desc' for most recent first");
    
    Ok(())
}

/// Calculate and display basic statistics for a set of bars
fn calculate_basic_stats(bars: &[polygon_io::stocks::AggregateBar], timeframe: &str) {
    if bars.is_empty() {
        return;
    }

    let closes: Vec<f64> = bars.iter()
        .filter_map(|bar| bar.close)
        .collect();
    
    if closes.is_empty() {
        return;
    }

    let high_prices: Vec<f64> = bars.iter()
        .filter_map(|bar| bar.high)
        .collect();
    
    let low_prices: Vec<f64> = bars.iter()
        .filter_map(|bar| bar.low)
        .collect();

    let volumes: Vec<f64> = bars.iter()
        .filter_map(|bar| bar.volume)
        .collect();

    if let (Some(&first_close), Some(&last_close)) = (closes.first(), closes.last()) {
        let total_change = last_close - first_close;
        let total_change_pct = (total_change / first_close) * 100.0;
        
        let max_high = high_prices.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let min_low = low_prices.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let avg_volume = volumes.iter().sum::<f64>() / volumes.len() as f64;

        println!("\n📊 {} Statistics:", timeframe);
        println!("   💰 Period change: ${:.2} ({:.2}%)", total_change, total_change_pct);
        println!("   📏 High/Low range: ${:.2} - ${:.2}", min_low, max_high);
        println!("   📊 Average volume: {:.0}", avg_volume);
        println!("   📈 Total bars analyzed: {}", bars.len());
    }
}