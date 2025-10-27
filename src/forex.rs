//! Forex API client and data types
//!
//! This module provides access to forex market data from Polygon.io.

use crate::client::PolygonClient;

/// Client for accessing forex market data from Polygon.io
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ForexClient {
    client: PolygonClient,
}

impl ForexClient {
    #[allow(dead_code)]
    pub(crate) fn new(client: PolygonClient) -> Self {
        Self { client }
    }

    // Forex endpoints will be implemented here
}
