//! Crypto API client and data types
//!
//! This module provides access to cryptocurrency market data from Polygon.io.

use crate::client::PolygonClient;

/// Client for accessing cryptocurrency market data from Polygon.io
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CryptoClient {
    client: PolygonClient,
}

impl CryptoClient {
    #[allow(dead_code)]
    pub(crate) fn new(client: PolygonClient) -> Self {
        Self { client }
    }

    // Crypto endpoints will be implemented here
}
