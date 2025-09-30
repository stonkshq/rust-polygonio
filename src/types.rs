//! Common types used across the Polygon.io API

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Standard API response wrapper used by most Polygon.io endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// The status of the response
    pub status: String,
    
    /// Unique request identifier
    pub request_id: String,
    
    /// Number of results returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    
    /// The actual data results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<T>,
    
    /// Next page URL for paginated responses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_url: Option<String>,
}

/// Pagination parameters for API requests
#[derive(Debug, Clone, Default)]
pub struct PaginationParams {
    /// Number of results to return per page
    pub limit: Option<i32>,
    
    /// Sort order for results
    pub sort: Option<String>,
    
    /// Cursor for pagination
    pub cursor: Option<String>,
}

/// Date range parameters for time-based queries
#[derive(Debug, Clone)]
pub struct DateRange {
    /// Start date (inclusive)
    pub from: Option<chrono::NaiveDate>,
    
    /// End date (inclusive) 
    pub to: Option<chrono::NaiveDate>,
}

/// Timestamp with nanosecond precision as used by Polygon.io
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolygonTimestamp(pub i64);

impl PolygonTimestamp {
    /// Create a new timestamp from nanoseconds since epoch
    pub fn from_nanos(nanos: i64) -> Self {
        Self(nanos)
    }
    
    /// Convert to DateTime<Utc>
    pub fn to_datetime(&self) -> Option<DateTime<Utc>> {
        DateTime::from_timestamp(self.0 / 1_000_000_000, (self.0 % 1_000_000_000) as u32)
    }
    
    /// Get the raw nanoseconds value
    pub fn nanos(&self) -> i64 {
        self.0
    }
}

/// Market status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketStatus {
    /// Whether the market is currently open
    pub market: String,
    
    /// Server time
    #[serde(rename = "serverTime")]
    pub server_time: DateTime<Utc>,
    
    /// Individual exchange statuses
    pub exchanges: Option<serde_json::Value>,
    
    /// Currency markets status
    pub currencies: Option<serde_json::Value>,
}

/// Exchange information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exchange {
    /// Exchange ID
    pub id: i32,
    
    /// Exchange type
    #[serde(rename = "type")]
    pub exchange_type: String,
    
    /// Market identifier code
    pub mic: Option<String>,
    
    /// Exchange name
    pub name: String,
    
    /// Exchange tape
    pub tape: Option<String>,
}

/// Condition codes used in trades and quotes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionCode {
    /// Condition ID
    pub id: String,
    
    /// Condition type
    #[serde(rename = "type")]
    pub condition_type: Option<String>,
    
    /// Human readable name
    pub name: Option<String>,
    
    /// Condition description
    pub description: Option<String>,
    
    /// Legacy condition flag
    pub legacy: Option<bool>,
    
    /// SIP mapping information
    pub sip_mapping: Option<serde_json::Value>,
    
    /// Data types this condition applies to
    pub data_types: Option<Vec<String>>,
}

/// Common ticker information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickerInfo {
    /// The ticker symbol
    pub ticker: String,
    
    /// Company name
    pub name: Option<String>,
    
    /// Primary exchange
    pub primary_exchange: Option<String>,
    
    /// Market type
    pub market: Option<String>,
    
    /// Locale
    pub locale: Option<String>,
    
    /// Currency code
    pub currency_name: Option<String>,
    
    /// Whether the ticker is active
    pub active: Option<bool>,
    
    /// Composite FIGI
    pub cik: Option<String>,
    
    /// Central Index Key
    pub composite_figi: Option<String>,
    
    /// Share Class FIGI
    pub share_class_figi: Option<String>,
    
    /// Last updated timestamp
    pub last_updated_utc: Option<DateTime<Utc>>,
}

/// Trade data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    /// Conditions applied to this trade
    pub conditions: Option<Vec<i32>>,
    
    /// Exchange ID
    pub exchange: Option<i32>,
    
    /// Trade price
    pub price: Option<f64>,
    
    /// SIP timestamp
    pub sip_timestamp: Option<i64>,
    
    /// Trade size
    pub size: Option<i64>,
    
    /// Timeframe (for aggregated data)
    pub timeframe: Option<String>,
    
    /// Participant timestamp
    pub participant_timestamp: Option<i64>,
    
    /// Trade ID
    pub id: Option<String>,
}

/// Quote data structure  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    /// Ask price
    pub ask: Option<f64>,
    
    /// Ask exchange
    pub ask_exchange: Option<i32>,
    
    /// Ask size
    pub ask_size: Option<i64>,
    
    /// Bid price  
    pub bid: Option<f64>,
    
    /// Bid exchange
    pub bid_exchange: Option<i32>,
    
    /// Bid size
    pub bid_size: Option<i64>,
    
    /// SIP timestamp
    pub sip_timestamp: Option<i64>,
    
    /// Participant timestamp
    pub participant_timestamp: Option<i64>,
    
    /// Timeframe (for aggregated data)
    pub timeframe: Option<String>,
}