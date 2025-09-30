//! Error types for the Polygon.io API client

use thiserror::Error;

/// Result type alias for Polygon.io API operations
pub type Result<T> = std::result::Result<T, PolygonError>;

/// Errors that can occur when using the Polygon.io API
#[derive(Error, Debug)]
pub enum PolygonError {
    /// HTTP request errors
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON serialization/deserialization errors
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// URL parsing errors
    #[error("URL parsing error: {0}")]
    Url(#[from] url::ParseError),

    /// API authentication errors
    #[error("Authentication failed: {message}")]
    Authentication { message: String },

    /// API rate limiting errors
    #[error("Rate limit exceeded: {message}")]
    RateLimit { message: String },

    /// API quota exceeded errors
    #[error("Quota exceeded: {message}")]
    QuotaExceeded { message: String },

    /// Invalid API response
    #[error("Invalid API response: {message}")]
    InvalidResponse { message: String },

    /// API returned an error status
    #[error("API error (status: {status}): {message}")]
    ApiError { status: u16, message: String },

    /// Invalid input parameters
    #[error("Invalid parameter: {message}")]
    InvalidParameter { message: String },

    /// WebSocket connection errors
    #[error("WebSocket error: {message}")]
    WebSocket { message: String },

    /// General client errors
    #[error("Client error: {message}")]
    Client { message: String },
}

impl PolygonError {
    /// Create a new authentication error
    pub fn authentication<S: Into<String>>(message: S) -> Self {
        Self::Authentication {
            message: message.into(),
        }
    }

    /// Create a new rate limit error
    pub fn rate_limit<S: Into<String>>(message: S) -> Self {
        Self::RateLimit {
            message: message.into(),
        }
    }

    /// Create a new quota exceeded error
    pub fn quota_exceeded<S: Into<String>>(message: S) -> Self {
        Self::QuotaExceeded {
            message: message.into(),
        }
    }

    /// Create a new invalid response error
    pub fn invalid_response<S: Into<String>>(message: S) -> Self {
        Self::InvalidResponse {
            message: message.into(),
        }
    }

    /// Create a new API error
    pub fn api_error<S: Into<String>>(status: u16, message: S) -> Self {
        Self::ApiError {
            status,
            message: message.into(),
        }
    }

    /// Create a new invalid parameter error
    pub fn invalid_parameter<S: Into<String>>(message: S) -> Self {
        Self::InvalidParameter {
            message: message.into(),
        }
    }

    /// Create a new WebSocket error
    pub fn websocket<S: Into<String>>(message: S) -> Self {
        Self::WebSocket {
            message: message.into(),
        }
    }

    /// Create a new client error
    pub fn client<S: Into<String>>(message: S) -> Self {
        Self::Client {
            message: message.into(),
        }
    }
}