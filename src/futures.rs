//! Futures API client and data types
//!
//! This module provides access to futures market data from Polygon.io.

use crate::client::PolygonClient;

/// Client for accessing futures market data from Polygon.io
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FuturesClient {
    client: PolygonClient,
}

impl FuturesClient {
    #[allow(dead_code)]
    pub(crate) fn new(client: PolygonClient) -> Self {
        Self { client }
    }

    // Futures endpoints will be implemented here
}
