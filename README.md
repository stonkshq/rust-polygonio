# Polygon.io Rust Client

A comprehensive, type-safe Rust client library for the [Polygon.io](https://polygon.io) REST and WebSocket APIs. This library provides structured access to real-time and historical market data for stocks, options, futures, indices, forex, and cryptocurrencies.

## Features

- 🦀 **Fully async/await** - Built with tokio for high-performance async operations
- 🔒 **Type-safe** - Comprehensive type definitions with serde serialization
- 📊 **Complete API coverage** - Support for all major Polygon.io endpoints
- 🏗️ **Hierarchical organization** - Structured like Polygon.io's documentation
- 🔑 **Multiple auth methods** - API key via query parameter or Authorization header
- ⚡ **Built-in error handling** - Detailed error types for different failure scenarios
- 📈 **Asset class support** - Stocks, Options, Futures, Indices, Forex, and Crypto
- 🌐 **WebSocket ready** - Architecture prepared for real-time data streaming

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
polygon-io = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

### Basic Usage

```rust
use polygon_io::{PolygonClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Create client with your API key
    let client = PolygonClient::new("your-api-key".to_string());

    // Access different asset classes
    let stocks = client.stocks();
    let options = client.options();
    let crypto = client.crypto();

    // Get ticker details
    let details = stocks.ticker_details("AAPL").await?;
    println!("Company: {}", details.results.name.unwrap_or_default());

    // Get market aggregates (OHLCV data)
    let aggs = stocks.aggregates(
        "AAPL",           // ticker
        1,                // multiplier
        "day",            // timespan
        "2024-01-01",     // from date
        "2024-01-31",     // to date
        None              // optional parameters
    ).await?;

    if let Some(results) = aggs.results.and_then(|r| r.results) {
        for bar in results {
            println!("OHLC: ${:.2} ${:.2} ${:.2} ${:.2}",
                bar.o.unwrap_or(0.0), bar.h.unwrap_or(0.0),
                bar.l.unwrap_or(0.0), bar.c.unwrap_or(0.0));
        }
    }

    Ok(())
}
```

## API Organization

The library is organized hierarchically following Polygon.io's documentation structure:

### 📈 Stocks (`client.stocks()`)

- **Reference Data**: Ticker details, company information, market holidays
- **Market Data**: Real-time trades and quotes
- **Aggregates**: OHLCV bars (minute, hour, day, week, month, etc.)
- **Snapshots**: Current market state for tickers
- **Fundamentals**: FINRA short interest metrics (days to cover, average volume)
- **Corporate Actions**: Dividends, stock splits

```rust
let stocks = client.stocks();

// Get company details
let details = stocks.ticker_details("AAPL").await?;

// Get daily bars for date range
let bars = stocks.aggregates("AAPL", 1, "day", "2024-01-01", "2024-01-31", None).await?;

// Get current snapshot
let snapshot = stocks.ticker_snapshot("AAPL").await?;

// Get market status
let status = stocks.market_status().await?;

// Get short interest fundamentals (sorted by most recent)
let mut short_interest_params = polygon_io::stocks::ShortInterestParams::new("AAPL");
short_interest_params.sort = Some("settlement_date.desc".to_string());
short_interest_params.limit = Some(5);

let short_interest = stocks.short_interest(&short_interest_params).await?;
if let Some(records) = short_interest.results {
    for entry in records {
        println!("Short interest on {}: {:?}", entry.settlement_date.unwrap_or_default(), entry.short_interest);
    }
}
```

### 🎯 Options (`client.options()`)

- **Contracts**: Options contract details and chains
- **Market Data**: Options trades and quotes
- **Aggregates**: OHLCV data for options

### 📦 Futures (`client.futures()`)

- **Contracts**: Futures contract specifications
- **Market Data**: Futures trades and quotes
- **Aggregates**: OHLCV data for futures

### 📊 Indices (`client.indices()`)

- **Values**: Index values and calculations
- **Aggregates**: Historical index data

### 💱 Forex (`client.forex()`)

- **Real-time Rates**: Live currency exchange rates
- **Historical Data**: Historical forex data
- **Aggregates**: OHLCV data for currency pairs

### ₿ Crypto (`client.crypto()`)

- **Market Data**: Cryptocurrency trades and quotes
- **Aggregates**: OHLCV data for crypto pairs
- **Snapshots**: Current crypto market state

## Authentication

The client supports multiple authentication methods:

### API Key in Constructor

```rust
let client = PolygonClient::new("your-api-key".to_string());
```

### Environment Variable

```rust
let api_key = std::env::var("POLYGON_API_KEY").expect("API key not found");
let client = PolygonClient::new(api_key);
```

The client automatically handles authentication using both query parameters and Authorization headers as fallbacks.

## Error Handling

The library provides comprehensive error handling with detailed error types:

```rust
use polygon_io::{PolygonClient, PolygonError};

match stocks.ticker_details("INVALID").await {
    Ok(response) => println!("Success: {:?}", response),
    Err(PolygonError::Authentication { message }) => {
        println!("Auth error: {}", message);
    },
    Err(PolygonError::RateLimit { message }) => {
        println!("Rate limited: {}", message);
    },
    Err(PolygonError::ApiError { status, message }) => {
        println!("API error {}: {}", status, message);
    },
    Err(e) => println!("Other error: {}", e),
}
```

## Response Types

All API responses are wrapped in a standard `ApiResponse<T>` structure:

```rust
#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub status: String,           // "OK" for success
    pub request_id: String,       // Unique request identifier
    pub count: Option<i32>,       // Number of results
    pub results: Option<T>,       // The actual data
    pub next_url: Option<String>, // For pagination
}
```

## Advanced Usage

### Custom Parameters

Many endpoints support optional parameters for filtering and pagination:

```rust
use polygon_io::stocks::{AggregatesParams, ListTickersParams};

// Aggregates with custom parameters
let params = AggregatesParams {
    adjusted: Some(true),
    sort: Some("asc".to_string()),
    limit: Some(1000),
};

let aggs = stocks.aggregates("AAPL", 1, "day", "2024-01-01", "2024-01-31", Some(params)).await?;

// List tickers with filtering
let ticker_params = ListTickersParams {
    market: Some("stocks".to_string()),
    active: Some(true),
    limit: Some(100),
    ..Default::default()
};

let tickers = stocks.list_tickers(Some(ticker_params)).await?;
```

### Custom Base URL

For testing or custom deployments:

```rust
let client = PolygonClient::with_base_url(
    "your-api-key".to_string(),
    "https://custom-api.polygon.io".to_string()
);
```

## Examples

Run the included examples:

```bash
# Set your API key
export POLYGON_API_KEY=your_api_key_here

# Run basic example
cargo run --example basic_usage

# Run with custom ticker
TICKER=MSFT cargo run --example basic_usage

# Inspect FINRA short interest fundamentals
cargo run --example short_interest
```

## Roadmap

- [x] REST API client foundation
- [x] Stocks endpoints (reference data, aggregates, snapshots)
- [x] Comprehensive error handling
- [x] Type-safe response deserialization
- [ ] WebSocket client for real-time data
- [ ] Options endpoints implementation
- [ ] Futures endpoints implementation
- [ ] Forex endpoints implementation
- [ ] Crypto endpoints implementation
- [ ] Indices endpoints implementation
- [ ] Corporate actions endpoints
- [ ] News and sentiment data
- [ ] Technical indicators
- [ ] Rate limiting and retry logic
- [ ] Async streams for large datasets

## Requirements

- Rust 1.70+ (2021 edition)
- A [Polygon.io API key](https://polygon.io/dashboard)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Disclaimer

This library is not officially affiliated with Polygon.io. It is a community-driven project to provide Rust developers with access to Polygon.io's market data APIs.

Please ensure you comply with Polygon.io's [Terms of Service](https://polygon.io/terms) and [API documentation](https://polygon.io/docs) when using this library.
