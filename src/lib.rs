//! # Polygon.io API Client
//!
//! A comprehensive Rust client library for the Polygon.io REST and WebSocket APIs.
//! This library provides structured access to stock market data following the
//! hierarchical organization of Polygon.io's documentation.
//!
//! ## Features
//!
//! - REST API client for fetching historical and real-time market data
//! - WebSocket client for streaming live market data
//! - Organized by asset classes: Stocks, Options, Futures, Indices, Forex, Crypto
//! - Type-safe API responses with serde serialization
//! - Async/await support with tokio
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use polygon_io::PolygonClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = PolygonClient::new("your-api-key".to_string());
//!     let stocks = client.stocks();
//!     
//!     // Fetch ticker details
//!     let ticker_details = stocks.ticker_details("AAPL").await?;
//!     println!("{:?}", ticker_details);
//!     
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod deserializers;
pub mod error;
pub mod types;

// Asset class modules following Polygon.io's documentation structure
pub mod crypto;
pub mod forex;
pub mod futures;
pub mod indices;
pub mod options;
pub mod stocks;

// Re-export main client and common types
pub use client::PolygonClient;
pub use error::{PolygonError, Result};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = PolygonClient::new("test-key".to_string());
        assert_eq!(client.api_key(), "test-key");
    }
}
