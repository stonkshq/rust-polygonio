//! Main client implementation for the Polygon.io API

use crate::{
    error::{PolygonError, Result},
    stocks::StocksClient,
    options::OptionsClient,
    futures::FuturesClient,
    indices::IndicesClient,
    forex::ForexClient,
    crypto::CryptoClient,
    types::ApiResponse,
};
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use url::Url;

/// Base URL for the Polygon.io REST API
pub const POLYGON_BASE_URL: &str = "https://api.polygon.io";

/// Main client for interacting with the Polygon.io API
#[derive(Debug, Clone)]
pub struct PolygonClient {
    /// HTTP client for making requests
    http_client: Client,
    
    /// API key for authentication
    api_key: String,
    
    /// Base URL for the API
    base_url: String,
}

impl PolygonClient {
    /// Create a new Polygon.io client with the provided API key
    ///
    /// # Arguments
    /// * `api_key` - Your Polygon.io API key
    ///
    /// # Example
    /// ```rust
    /// use polygon_io::PolygonClient;
    /// 
    /// let client = PolygonClient::new("your-api-key".to_string());
    /// ```
    pub fn new(api_key: String) -> Self {
        Self {
            http_client: Client::new(),
            api_key,
            base_url: POLYGON_BASE_URL.to_string(),
        }
    }

    /// Create a new client with a custom base URL (useful for testing)
    ///
    /// # Arguments  
    /// * `api_key` - Your Polygon.io API key
    /// * `base_url` - Custom base URL for the API
    pub fn with_base_url(api_key: String, base_url: String) -> Self {
        Self {
            http_client: Client::new(),
            api_key,
            base_url,
        }
    }

    /// Get the API key being used by this client
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    /// Get a stocks client for accessing stock market data
    pub fn stocks(&self) -> StocksClient {
        StocksClient::new(self.clone())
    }

    /// Get an options client for accessing options market data
    pub fn options(&self) -> OptionsClient {
        OptionsClient::new(self.clone())
    }

    /// Get a futures client for accessing futures market data
    pub fn futures(&self) -> FuturesClient {
        FuturesClient::new(self.clone())
    }

    /// Get an indices client for accessing indices market data
    pub fn indices(&self) -> IndicesClient {
        IndicesClient::new(self.clone())
    }

    /// Get a forex client for accessing forex market data
    pub fn forex(&self) -> ForexClient {
        ForexClient::new(self.clone())
    }

    /// Get a crypto client for accessing cryptocurrency market data
    pub fn crypto(&self) -> CryptoClient {
        CryptoClient::new(self.clone())
    }

    /// Make a GET request to the Polygon.io API
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint (without the base URL)
    /// * `params` - Query parameters to include in the request
    pub async fn get<T>(&self, endpoint: &str, params: Option<HashMap<String, String>>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = self.build_url(endpoint, params)?;
        
        let response = self
            .http_client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Make a GET request and return the raw response
    pub async fn get_raw(&self, endpoint: &str, params: Option<HashMap<String, String>>) -> Result<Response> {
        let url = self.build_url(endpoint, params)?;
        
        let response = self
            .http_client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        Ok(response)
    }

    /// Build a complete URL for an API endpoint with parameters
    fn build_url(&self, endpoint: &str, params: Option<HashMap<String, String>>) -> Result<String> {
        let mut url = Url::parse(&format!("{}/{}", self.base_url, endpoint.trim_start_matches('/')))?;

        // Add API key as query parameter (fallback authentication method)
        url.query_pairs_mut()
            .append_pair("apikey", &self.api_key);

        // Add additional parameters
        if let Some(params) = params {
            for (key, value) in params {
                url.query_pairs_mut().append_pair(&key, &value);
            }
        }

        Ok(url.to_string())
    }

    /// Handle an HTTP response and deserialize the JSON
    async fn handle_response<T>(&self, response: Response) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let status = response.status();
        
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            
            return Err(match status.as_u16() {
                401 => PolygonError::authentication("Invalid API key or unauthorized access"),
                403 => PolygonError::authentication("Forbidden - check your API key permissions"),
                429 => PolygonError::rate_limit("Rate limit exceeded - please slow down your requests"),
                402 => PolygonError::quota_exceeded("Quota exceeded - upgrade your plan or wait for reset"),
                _ => PolygonError::api_error(status.as_u16(), error_text),
            });
        }

        let json_text = response.text().await?;
        
        // Try to deserialize as the expected type
        match serde_json::from_str::<T>(&json_text) {
            Ok(data) => Ok(data),
            Err(e) => {
                // If deserialization fails, try to parse as a standard API response to get error details
                if let Ok(api_response) = serde_json::from_str::<ApiResponse<serde_json::Value>>(&json_text) {
                    if api_response.status != "OK" {
                        return Err(PolygonError::api_error(
                            status.as_u16(),
                            format!("API returned status: {}", api_response.status),
                        ));
                    }
                }
                
                // Truncate the JSON text for error messages to avoid massive output
                let truncated_json = if json_text.len() > 500 {
                    format!("{}... (truncated, {} total chars)", &json_text[..500], json_text.len())
                } else {
                    json_text.clone()
                };
                
                Err(PolygonError::invalid_response(format!(
                    "Failed to deserialize response: {}. Response was: {}",
                    e, truncated_json
                )))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = PolygonClient::new("test-key".to_string());
        assert_eq!(client.api_key(), "test-key");
        assert_eq!(client.base_url, POLYGON_BASE_URL);
    }

    #[test]
    fn test_client_with_custom_url() {
        let custom_url = "https://custom.api.com";
        let client = PolygonClient::with_base_url("test-key".to_string(), custom_url.to_string());
        assert_eq!(client.api_key(), "test-key");
        assert_eq!(client.base_url, custom_url);
    }

    #[test]
    fn test_url_building() {
        let client = PolygonClient::new("test-key".to_string());
        
        // Test basic endpoint
        let url = client.build_url("/v2/aggs/ticker/AAPL/range/1/day/2023-01-01/2023-01-31", None).unwrap();
        assert!(url.contains("apikey=test-key"));
        assert!(url.contains("/v2/aggs/ticker/AAPL/range/1/day/2023-01-01/2023-01-31"));
        
        // Test with parameters
        let mut params = HashMap::new();
        params.insert("adjusted".to_string(), "true".to_string());
        params.insert("sort".to_string(), "asc".to_string());
        
        let url = client.build_url("/v2/aggs/ticker/AAPL/range/1/day/2023-01-01/2023-01-31", Some(params)).unwrap();
        assert!(url.contains("apikey=test-key"));
        assert!(url.contains("adjusted=true"));
        assert!(url.contains("sort=asc"));
    }
}