//! Quotes data for stocks
//! 
//! This module provides access to bid/ask quote data.

use crate::{client::PolygonClient, error::Result, types::ApiResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Client for stock quotes data
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct QuotesClient {
    client: PolygonClient,
}

impl QuotesClient {
    #[allow(dead_code)]
    pub(crate) fn new(client: PolygonClient) -> Self {
        Self { client }
    }

    /// Get quotes for a ticker on a given date
    pub async fn quotes(&self, ticker: &str, date: &str, params: Option<QuotesParams>) -> Result<ApiResponse<Vec<Quote>>> {
        let endpoint = format!("v3/quotes/{}", ticker);
        
        let mut query_params = HashMap::new();
        query_params.insert("timestamp".to_string(), date.to_string());
        
        if let Some(p) = params {
            if let Some(timestamp_gte) = p.timestamp_gte {
                query_params.insert("timestamp.gte".to_string(), timestamp_gte);
            }
            if let Some(timestamp_lte) = p.timestamp_lte {
                query_params.insert("timestamp.lte".to_string(), timestamp_lte);
            }
            if let Some(sort) = p.sort {
                query_params.insert("sort".to_string(), sort);
            }
            if let Some(limit) = p.limit {
                query_params.insert("limit".to_string(), limit.to_string());
            }
        }

        self.client.get(&endpoint, Some(query_params)).await
    }
}

#[derive(Debug, Clone, Default)]
pub struct QuotesParams {
    pub timestamp_gte: Option<String>,
    pub timestamp_lte: Option<String>,
    pub sort: Option<String>,
    pub limit: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub ask: Option<f64>,
    pub ask_exchange: Option<i32>,
    pub ask_size: Option<i64>,
    pub bid: Option<f64>,
    pub bid_exchange: Option<i32>,
    pub bid_size: Option<i64>,
    pub sip_timestamp: Option<i64>,
    pub participant_timestamp: Option<i64>,
}