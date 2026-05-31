use std::{collections::HashMap, sync::Arc};

use reqwest::{self, Method, Url};
use serde_json::Value;
use thiserror::Error;

use crate::events::EventsAPI;

#[derive(Debug, PartialEq, Error)]
pub enum BsRsError {
    #[error("Incorrect Params (400): {0}")]
    IncorrectParams(String), // 400
    #[error("Access Denied (403): {0}")]
    AccessDenied(String),    // 403
    #[error("Not Found (404): {0}")]
    NotFound(String),        // 404
    #[error("Rate Limit (429): {0}")]
    RateLimit(String),       // 429
    #[error("Unknown (500): {0}")]
    Unknown(String),         // 500
    #[error("Unavailable (503): {0}")]
    Unavailable(String)      // 503
}

#[derive(Clone)]
pub struct Client {
    key: Arc<str>,
    http: reqwest::Client
}

impl Client {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            key: Arc::from(api_key.into()),
            http: reqwest::Client::new(),
        }
    }

    pub fn events(&self) -> EventsAPI {
        EventsAPI::new(self.clone())
    }

    pub(crate) async fn request(&self, url: &str, method: Method, query: HashMap<&str, &str>) -> Result<Value, Box<dyn std::error::Error>> {
        let params_url = Url::parse_with_params(url, query)?;
        let response = self.http.request(method, params_url)
            .header("Authorization", format!("Bearer {}", self.key))
            .send()
            .await?;

        let status_code = response.status().as_u16();
        let raw_text = response.text().await?;
        let json_value: Value = serde_json::from_str(&raw_text)?;

        Ok(json_value)
    }
}
