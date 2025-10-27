//! Stocks API client and data types
//!
//! This module provides access to all stock market data endpoints from Polygon.io,
//! organized hierarchically as per their documentation:
//! - Market Data (trades, quotes, aggregates, snapshots)
//! - Reference Data (tickers, exchanges, conditions)
//! - Corporate Actions (dividends, splits)

pub mod aggregates;
pub mod market_data;
pub mod quotes;
pub mod reference;
pub mod snapshots;
pub mod trades;

use crate::{
    client::PolygonClient, deserializers::deserialize_volume_as_i64, error::Result,
    types::ApiResponse,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Client for accessing stock market data from Polygon.io
#[derive(Debug, Clone)]
pub struct StocksClient {
    /// Reference to the main Polygon client
    client: PolygonClient,
}

impl StocksClient {
    /// Create a new stocks client
    pub(crate) fn new(client: PolygonClient) -> Self {
        Self { client }
    }

    /// Get detailed information about a ticker
    ///
    /// # Arguments
    /// * `ticker` - The ticker symbol to get details for
    ///
    /// # Example
    /// ```rust,no_run
    /// # use polygon_io::{PolygonClient, Result};
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// let client = PolygonClient::new("your-api-key".to_string());
    /// let stocks = client.stocks();
    ///
    /// let details = stocks.ticker_details("AAPL").await?;
    /// if let Some(ticker) = details.results {
    ///     println!("Company: {}", ticker.name.unwrap_or_default());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn ticker_details(&self, ticker: &str) -> Result<ApiResponse<TickerDetails>> {
        let endpoint = format!("v3/reference/tickers/{}", ticker);
        self.client.get(&endpoint, None).await
    }

    /// Get a list of tickers with optional filtering
    ///
    /// # Arguments
    /// * `params` - Optional parameters for filtering and pagination
    pub async fn list_tickers(
        &self,
        params: Option<ListTickersParams>,
    ) -> Result<ApiResponse<Vec<TickerInfo>>> {
        let mut query_params = HashMap::new();

        if let Some(p) = params {
            if let Some(market) = p.market {
                query_params.insert("market".to_string(), market);
            }
            if let Some(exchange) = p.exchange {
                query_params.insert("exchange".to_string(), exchange);
            }
            if let Some(cusip) = p.cusip {
                query_params.insert("cusip".to_string(), cusip);
            }
            if let Some(cik) = p.cik {
                query_params.insert("cik".to_string(), cik);
            }
            if let Some(date) = p.date {
                query_params.insert("date".to_string(), date.format("%Y-%m-%d").to_string());
            }
            if let Some(search) = p.search {
                query_params.insert("search".to_string(), search);
            }
            if let Some(active) = p.active {
                query_params.insert("active".to_string(), active.to_string());
            }
            if let Some(sort) = p.sort {
                query_params.insert("sort".to_string(), sort);
            }
            if let Some(order) = p.order {
                query_params.insert("order".to_string(), order);
            }
            if let Some(limit) = p.limit {
                query_params.insert("limit".to_string(), limit.to_string());
            }
        }

        let params = if query_params.is_empty() {
            None
        } else {
            Some(query_params)
        };

        self.client.get("v3/reference/tickers", params).await
    }

    /// Get the current market status
    pub async fn market_status(&self) -> Result<MarketStatus> {
        self.client.get("v1/marketstatus/now", None).await
    }

    /// Get market holidays
    pub async fn market_holidays(&self) -> Result<Vec<MarketHoliday>> {
        self.client.get("v1/marketstatus/upcoming", None).await
    }

    /// Get short interest data for stocks
    ///
    /// This endpoint returns bi-monthly aggregated short interest data reported to FINRA
    /// by broker-dealers for the specified ticker. Results include the number of shares
    /// sold short and contextual metrics like days to cover and average daily volume.
    pub async fn short_interest(
        &self,
        params: &ShortInterestParams,
    ) -> Result<ApiResponse<Vec<ShortInterestRecord>>> {
        let mut query_params = HashMap::new();
        query_params.insert("ticker".to_string(), params.ticker.clone());

        if let Some(date) = params.settlement_date {
            query_params.insert(
                "settlement_date".to_string(),
                date.format("%Y-%m-%d").to_string(),
            );
        }
        if let Some(days_to_cover) = params.days_to_cover {
            query_params.insert("days_to_cover".to_string(), days_to_cover.to_string());
        }
        if let Some(avg_daily_volume) = params.avg_daily_volume {
            query_params.insert("avg_daily_volume".to_string(), avg_daily_volume.to_string());
        }
        if let Some(limit) = params.limit {
            query_params.insert("limit".to_string(), limit.to_string());
        }
        if let Some(sort) = &params.sort {
            query_params.insert("sort".to_string(), sort.clone());
        }
        if let Some(cursor) = &params.cursor {
            query_params.insert("cursor".to_string(), cursor.clone());
        }

        self.client
            .get("stocks/v1/short-interest", Some(query_params))
            .await
    }

    /// Get aggregated bars (custom OHLC bars) for a ticker over a given date range
    ///
    /// This method retrieves aggregated historical OHLC (Open, High, Low, Close) and volume data
    /// for a specified stock ticker over a custom date range and time interval in Eastern Time (ET).
    /// Users can tailor their data by adjusting the multiplier and timespan parameters
    /// (e.g., 5-minute bars, 1-hour bars, daily bars, etc.).
    ///
    /// # Arguments
    /// * `ticker` - The stock ticker symbol (e.g., "AAPL")
    /// * `multiplier` - The size of the timespan multiplier (e.g., 1 for 1-minute, 5 for 5-minute)
    /// * `timespan` - The time window: "minute", "hour", "day", "week", "month", "quarter", "year"
    /// * `from` - Start date (YYYY-MM-DD format or millisecond timestamp)
    /// * `to` - End date (YYYY-MM-DD format or millisecond timestamp)
    /// * `params` - Optional parameters (adjusted, sort, limit)
    ///
    /// # Example
    /// ```rust,no_run
    /// # use polygon_io::{PolygonClient, Result};
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// let client = PolygonClient::new("your-api-key".to_string());
    /// let stocks = client.stocks();
    ///
    /// // Get 5-minute bars for AAPL from January 1 to January 31, 2024
    /// let bars = stocks.aggregates("AAPL", 5, "minute", "2024-01-01", "2024-01-31", None).await?;
    ///
    /// if let Some(results) = bars.results {
    ///     for bar in results {
    ///         println!("Open: {:.2}, High: {:.2}, Low: {:.2}, Close: {:.2}",
    ///             bar.open.unwrap_or(0.0), bar.high.unwrap_or(0.0),
    ///             bar.low.unwrap_or(0.0), bar.close.unwrap_or(0.0));
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn aggregates(
        &self,
        ticker: &str,
        multiplier: i32,
        timespan: &str,
        from: &str,
        to: &str,
        params: Option<AggregatesParams>,
    ) -> Result<AggregatesResponse> {
        let endpoint = format!(
            "v2/aggs/ticker/{}/range/{}/{}/{}/{}",
            ticker, multiplier, timespan, from, to
        );

        let mut query_params = HashMap::new();

        if let Some(p) = params {
            if let Some(adjusted) = p.adjusted {
                query_params.insert("adjusted".to_string(), adjusted.to_string());
            }
            if let Some(sort) = p.sort {
                query_params.insert("sort".to_string(), sort);
            }
            if let Some(limit) = p.limit {
                query_params.insert("limit".to_string(), limit.to_string());
            }
        }

        let params = if query_params.is_empty() {
            None
        } else {
            Some(query_params)
        };

        self.client.get(&endpoint, params).await
    }

    /// Get snapshots for all tickers
    pub async fn all_tickers_snapshot(
        &self,
        params: Option<SnapshotParams>,
    ) -> Result<AllTickersSnapshotResponse> {
        let mut query_params = HashMap::new();

        if let Some(p) = params {
            if let Some(tickers) = p.tickers {
                query_params.insert("tickers".to_string(), tickers.join(","));
            }
        }

        let params = if query_params.is_empty() {
            None
        } else {
            Some(query_params)
        };

        self.client
            .get("v2/snapshot/locale/us/markets/stocks/tickers", params)
            .await
    }

    /// Get snapshot for a specific ticker
    pub async fn ticker_snapshot(&self, ticker: &str) -> Result<SnapshotResponse> {
        let endpoint = format!("v2/snapshot/locale/us/markets/stocks/tickers/{}", ticker);
        self.client.get(&endpoint, None).await
    }
}

// Parameter types for various endpoints

/// Parameters for listing tickers
#[derive(Debug, Clone, Default)]
pub struct ListTickersParams {
    pub market: Option<String>,
    pub exchange: Option<String>,
    pub cusip: Option<String>,
    pub cik: Option<String>,
    pub date: Option<chrono::NaiveDate>,
    pub search: Option<String>,
    pub active: Option<bool>,
    pub sort: Option<String>,
    pub order: Option<String>,
    pub limit: Option<i32>,
}

/// Parameters for aggregates requests
#[derive(Debug, Clone, Default)]
pub struct AggregatesParams {
    pub adjusted: Option<bool>,
    pub sort: Option<String>,
    pub limit: Option<i32>,
}

/// Parameters for snapshot requests
#[derive(Debug, Clone, Default)]
pub struct SnapshotParams {
    pub tickers: Option<Vec<String>>,
}

/// Parameters for querying short interest data
#[derive(Debug, Clone)]
pub struct ShortInterestParams {
    pub ticker: String,
    pub settlement_date: Option<chrono::NaiveDate>,
    pub days_to_cover: Option<f64>,
    pub avg_daily_volume: Option<i64>,
    pub limit: Option<i32>,
    pub sort: Option<String>,
    pub cursor: Option<String>,
}

impl ShortInterestParams {
    /// Create a new set of short interest parameters for the provided ticker
    pub fn new<T: Into<String>>(ticker: T) -> Self {
        Self {
            ticker: ticker.into(),
            settlement_date: None,
            days_to_cover: None,
            avg_daily_volume: None,
            limit: None,
            sort: None,
            cursor: None,
        }
    }
}

// Data types for API responses

/// Detailed information about a ticker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickerDetails {
    pub ticker: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub market: Option<String>,
    pub locale: Option<String>,
    pub primary_exchange: Option<String>,
    pub type_field: Option<String>,
    pub active: Option<bool>,
    pub currency_name: Option<String>,
    pub cik: Option<String>,
    pub composite_figi: Option<String>,
    pub share_class_figi: Option<String>,
    pub market_cap: Option<f64>,
    pub phone_number: Option<String>,
    pub address: Option<Address>,
    pub homepage_url: Option<String>,
    pub total_employees: Option<i32>,
    pub list_date: Option<String>,
    pub branding: Option<Branding>,
    pub sic_code: Option<String>,
    pub sic_description: Option<String>,
    pub ticker_root: Option<String>,
}

/// Address information for a company
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub address1: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
}

/// Branding information for a company
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Branding {
    pub logo_url: Option<String>,
    pub icon_url: Option<String>,
}

/// Basic ticker information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickerInfo {
    pub ticker: String,
    pub name: Option<String>,
    pub market: Option<String>,
    pub locale: Option<String>,
    pub primary_exchange: Option<String>,
    pub type_field: Option<String>,
    pub active: Option<bool>,
    pub currency_name: Option<String>,
    pub cik: Option<String>,
    pub composite_figi: Option<String>,
    pub share_class_figi: Option<String>,
    pub last_updated_utc: Option<String>,
}

/// Market status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketStatus {
    /// Overall market status
    pub market: String,
    /// Server time in Eastern Time
    #[serde(rename = "serverTime")]
    pub server_time: String,
    /// Whether it's after hours
    #[serde(rename = "afterHours")]
    pub after_hours: bool,
    /// Whether it's early hours (pre-market)
    #[serde(rename = "earlyHours")]
    pub early_hours: bool,
    /// Status of individual exchanges
    pub exchanges: Option<serde_json::Value>,
    /// Status of currency markets
    pub currencies: Option<serde_json::Value>,
    /// Status of indices groups
    #[serde(rename = "indicesGroups")]
    pub indices_groups: Option<serde_json::Value>,
}

/// Market holiday information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketHoliday {
    pub exchange: Option<String>,
    pub name: Option<String>,
    pub date: Option<String>,
    pub status: Option<String>,
    pub open: Option<String>,
    pub close: Option<String>,
}

/// Response wrapper for aggregates/custom bars data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatesResponse {
    pub ticker: Option<String>,
    pub adjusted: Option<bool>,
    #[serde(rename = "queryCount")]
    pub query_count: Option<i32>,
    pub request_id: Option<String>,
    #[serde(rename = "resultsCount")]
    pub results_count: Option<i32>,
    pub status: String,
    pub results: Option<Vec<AggregateBar>>,
    pub next_url: Option<String>,
}

/// Individual aggregate bar data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregateBar {
    /// The close price for the symbol in the given time period
    #[serde(rename = "c")]
    pub close: Option<f64>,
    /// The highest price for the symbol in the given time period  
    #[serde(rename = "h")]
    pub high: Option<f64>,
    /// The lowest price for the symbol in the given time period
    #[serde(rename = "l")]
    pub low: Option<f64>,
    /// The number of transactions in the aggregate window
    #[serde(rename = "n")]
    pub transactions: Option<i64>,
    /// The open price for the symbol in the given time period
    #[serde(rename = "o")]
    pub open: Option<f64>,
    /// Whether or not this aggregate is for an OTC ticker
    pub otc: Option<bool>,
    /// The Unix Msec timestamp for the start of the aggregate window
    #[serde(rename = "t")]
    pub timestamp: Option<i64>,
    /// The trading volume of the symbol in the given time period
    #[serde(rename = "v")]
    pub volume: Option<f64>,
    /// The volume weighted average price
    #[serde(rename = "vw")]
    pub volume_weighted_average: Option<f64>,
}

/// Snapshot response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotResponse {
    pub request_id: String,
    pub status: String,
    pub ticker: TickerSnapshot,
}

/// Response wrapper for all tickers snapshots
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllTickersSnapshotResponse {
    pub count: Option<i32>,
    pub status: String,
    pub tickers: Option<Vec<TickerSnapshot>>,
}

/// Snapshot data for a ticker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickerSnapshot {
    pub ticker: String,
    pub updated: Option<i64>,
    #[serde(rename = "todaysChange")]
    pub todays_change: Option<f64>,
    #[serde(rename = "todaysChangePerc")]
    pub todays_change_perc: Option<f64>,
    pub day: Option<DayData>,
    #[serde(rename = "lastQuote")]
    pub last_quote: Option<LastQuote>,
    #[serde(rename = "lastTrade")]
    pub last_trade: Option<LastTrade>,
    pub min: Option<MinuteData>,
    #[serde(rename = "prevDay")]
    pub prev_day: Option<PrevDayData>,
}

/// Day trading data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayData {
    /// Close price
    #[serde(rename = "c")]
    pub close: f64,
    /// High price
    #[serde(rename = "h")]
    pub high: f64,
    /// Low price
    #[serde(rename = "l")]
    pub low: f64,
    /// Open price
    #[serde(rename = "o")]
    pub open: f64,
    /// OTC flag (optional field that may be omitted if false)
    #[serde(rename = "otc")]
    pub otc: Option<bool>,
    /// Volume
    #[serde(rename = "v")]
    pub volume: f64,
    /// Volume weighted average price
    #[serde(rename = "vw")]
    pub volume_weighted_average: f64,
}

/// Last quote data (from snapshot)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LastQuote {
    /// Ask price (uppercase P in API)
    #[serde(rename = "P")]
    pub ask_price: f64,
    /// Ask size (uppercase S in API)
    #[serde(rename = "S")]
    pub ask_size: i64,
    /// Bid price (lowercase p in API)
    #[serde(rename = "p")]
    pub bid_price: f64,
    /// Bid size (lowercase s in API)
    #[serde(rename = "s")]
    pub bid_size: i64,
    /// Timestamp
    #[serde(rename = "t")]
    pub timestamp: i64,
}

/// Last trade data (from snapshot)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LastTrade {
    /// Conditions
    #[serde(rename = "c")]
    pub conditions: Option<Vec<i32>>,
    /// Trade ID
    #[serde(rename = "i")]
    pub trade_id: String,
    /// Price
    #[serde(rename = "p")]
    pub price: f64,
    /// Size
    #[serde(rename = "s")]
    pub size: i64,
    /// Timestamp
    #[serde(rename = "t")]
    pub timestamp: i64,
    /// Exchange
    #[serde(rename = "x")]
    pub exchange: i32,
}

/// Minute aggregated data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinuteData {
    /// Accumulated volume
    #[serde(rename = "av", deserialize_with = "deserialize_volume_as_i64")]
    pub accumulated_volume: i64,
    /// Close price
    #[serde(rename = "c")]
    pub close: f64,
    /// High price
    #[serde(rename = "h")]
    pub high: f64,
    /// Low price
    #[serde(rename = "l")]
    pub low: f64,
    /// Number of transactions
    #[serde(rename = "n")]
    pub transactions: i64,
    /// Open price
    #[serde(rename = "o")]
    pub open: f64,
    /// OTC flag (optional field that may be omitted if false)
    #[serde(rename = "otc")]
    pub otc: Option<bool>,
    /// Timestamp
    #[serde(rename = "t")]
    pub timestamp: i64,
    /// Volume
    #[serde(rename = "v")]
    pub volume: f64,
    /// Volume weighted average price
    #[serde(rename = "vw")]
    pub volume_weighted_average: f64,
}

/// Previous day data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrevDayData {
    /// Close price
    #[serde(rename = "c")]
    pub close: f64,
    /// High price
    #[serde(rename = "h")]
    pub high: f64,
    /// Low price
    #[serde(rename = "l")]
    pub low: f64,
    /// Open price
    #[serde(rename = "o")]
    pub open: f64,
    /// OTC flag (optional field that may be omitted if false)
    #[serde(rename = "otc")]
    pub otc: Option<bool>,
    /// Volume
    #[serde(rename = "v")]
    pub volume: f64,
    /// Volume weighted average price
    #[serde(rename = "vw")]
    pub volume_weighted_average: f64,
}

/// Short interest data as reported to FINRA
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortInterestRecord {
    /// The average daily trading volume for the reporting period
    #[serde(rename = "avg_daily_volume")]
    pub avg_daily_volume: Option<i64>,
    /// Calculated as short interest divided by average daily volume
    #[serde(rename = "days_to_cover")]
    pub days_to_cover: Option<f64>,
    /// Settlement date for the reported short interest
    #[serde(rename = "settlement_date")]
    pub settlement_date: Option<String>,
    /// Total number of shares sold short but not yet covered
    #[serde(rename = "short_interest")]
    pub short_interest: Option<i64>,
    /// The ticker symbol for the security
    pub ticker: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::PolygonClient;

    #[tokio::test]
    async fn test_stocks_client_creation() {
        let client = PolygonClient::new("test-key".to_string());
        let _stocks = client.stocks();

        // Just verify the client was created successfully
        assert!(true);
    }
}
