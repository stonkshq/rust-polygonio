//! Reference data for stocks
//!
//! This module provides access to reference data including ticker details,
//! exchanges, market holidays, and other metadata.

use crate::client::PolygonClient;

/// Client for stock reference data
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReferenceClient {
    client: PolygonClient,
}

impl ReferenceClient {
    #[allow(dead_code)]
    pub(crate) fn new(client: PolygonClient) -> Self {
        Self { client }
    }

    // Reference data endpoints are already implemented in the main stocks module
    // This module can be expanded for more specific reference data functionality
}
