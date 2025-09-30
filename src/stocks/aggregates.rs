//! Aggregates (bars) data for stocks
//! 
//! This module provides access to aggregate/OHLCV data for stocks.

use crate::{client::PolygonClient, error::Result, types::ApiResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Client for stock aggregates data
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AggregatesClient {
    client: PolygonClient,
}

impl AggregatesClient {
    #[allow(dead_code)]
    pub(crate) fn new(client: PolygonClient) -> Self {
        Self { client }
    }

    /// Get grouped daily aggregates for the entire market
    pub async fn grouped_daily(&self, date: &str, params: Option<GroupedDailyParams>) -> Result<ApiResponse<Vec<GroupedDaily>>> {
        let endpoint = format!("v2/aggs/grouped/locale/us/market/stocks/{}", date);
        
        let mut query_params = HashMap::new();
        if let Some(p) = params {
            if let Some(adjusted) = p.adjusted {
                query_params.insert("adjusted".to_string(), adjusted.to_string());
            }
        }

        let params = if query_params.is_empty() {
            None
        } else {
            Some(query_params)
        };

        self.client.get(&endpoint, params).await
    }

    /// Get previous close data for a ticker
    pub async fn previous_close(&self, ticker: &str, params: Option<PreviousCloseParams>) -> Result<ApiResponse<Vec<PreviousClose>>> {
        let endpoint = format!("v2/aggs/ticker/{}/prev", ticker);
        
        let mut query_params = HashMap::new();
        if let Some(p) = params {
            if let Some(adjusted) = p.adjusted {
                query_params.insert("adjusted".to_string(), adjusted.to_string());
            }
        }

        let params = if query_params.is_empty() {
            None
        } else {
            Some(query_params)
        };

        self.client.get(&endpoint, params).await
    }
}

#[derive(Debug, Clone, Default)]
pub struct GroupedDailyParams {
    pub adjusted: Option<bool>,
}

#[derive(Debug, Clone, Default)]
pub struct PreviousCloseParams {
    pub adjusted: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupedDaily {
    #[serde(rename = "T")]
    pub ticker: Option<String>,
    pub c: Option<f64>,
    pub h: Option<f64>,
    pub l: Option<f64>,
    pub o: Option<f64>,
    pub t: Option<i64>,
    pub v: Option<f64>,
    pub vw: Option<f64>,
    pub n: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviousClose {
    #[serde(rename = "T")]
    pub ticker: Option<String>,
    pub c: Option<f64>,
    pub h: Option<f64>,
    pub l: Option<f64>,
    pub o: Option<f64>,
    pub t: Option<i64>,
    pub v: Option<f64>,
    pub vw: Option<f64>,
}