//! Indices API client and data types
//!
//! This module provides access to indices market data from Polygon.io.

use crate::client::PolygonClient;

/// Client for accessing indices market data from Polygon.io
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct IndicesClient {
    client: PolygonClient,
}

impl IndicesClient {
    #[allow(dead_code)]
    pub(crate) fn new(client: PolygonClient) -> Self {
        Self { client }
    }

    // Indices endpoints will be implemented here
}
