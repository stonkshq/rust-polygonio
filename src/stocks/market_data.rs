//! Real-time market data for stocks
//! 
//! This module provides access to real-time trades and quotes.

use crate::client::PolygonClient;

/// Client for real-time market data
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MarketDataClient {
    client: PolygonClient,
}

impl MarketDataClient {
    #[allow(dead_code)]
    pub(crate) fn new(client: PolygonClient) -> Self {
        Self { client }
    }

    // Real-time market data endpoints will be implemented here
    // This is a placeholder for future WebSocket integration
}