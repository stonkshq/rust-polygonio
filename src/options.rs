//! Options API client and data types
//!
//! This module provides access to options market data from Polygon.io.
//! Structure follows the same pattern as stocks with contracts, trades, etc.

use crate::client::PolygonClient;

/// Client for accessing options market data from Polygon.io
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct OptionsClient {
    client: PolygonClient,
}

impl OptionsClient {
    #[allow(dead_code)]
    pub(crate) fn new(client: PolygonClient) -> Self {
        Self { client }
    }

    // Options endpoints will be implemented here following the same pattern as stocks
}
