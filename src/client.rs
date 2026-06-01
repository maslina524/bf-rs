use std::{collections::HashMap, sync::Arc};

use reqwest::{self, Method, Url};
use serde_json::{Value, Map};
use thiserror::Error;
use url;

use crate::events::EventsAPI;
use crate::get_string;
use crate::players::PlayersAPI;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ApiError {
    #[error("Incorrect Params (400): {0}")]
    IncorrectParams(String),
    #[error("Access Denied (403): {0}")]
    AccessDenied(String),
    #[error("Not Found (404): {0}")]
    NotFound(String),
    #[error("Rate Limit (429): {0}")]
    RateLimit(String),
    #[error("Unknown (500): {0}")]
    Unknown(String),
    #[error("Unavailable (503): {0}")]
    Unavailable(String)
}

#[derive(Debug, Error)]
pub enum CrateError {
    #[error("Api Error: {0}")]
    Api(#[from] ApiError),
    #[error("Json Error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("Http Error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("URL parsing error: {0}")]
    UrlParse(#[from] url::ParseError),
    #[error("Crate error: {0}")]
    Custom(String),
}

impl From<&str> for CrateError {
    fn from(msg: &str) -> Self {
        CrateError::Custom(msg.to_string())
    }
}

pub type BsResult<T> = std::result::Result<T, CrateError>;

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

    pub fn players(&self) -> PlayersAPI {
        PlayersAPI::new(self.clone())
    }

    pub(crate) async fn request(&self, url: &str, method: Method, query: HashMap<&str, &str>) -> BsResult<Value> {
        let params_url = Url::parse_with_params(url, query)?;
        let response = self.http.request(method, params_url)
            .header("Authorization", format!("Bearer {}", self.key))
            .send()
            .await?;

        let status = response.status();
        let raw_text = response.text().await?;

        if status.is_success() {
            let json_value = serde_json::from_str::<Value>(&raw_text)?;
            Ok(json_value)
        } else {
            let binding = serde_json::from_str::<Value>(&raw_text)?;
            let obj = binding
                .as_object()
                .ok_or("Strange Response")?;
            let error_msg = get_string!(&obj, "message").map(|s| s.to_string())?;

            let api_error = match status.as_u16() {
                400 => ApiError::IncorrectParams(error_msg),
                403 => ApiError::AccessDenied(error_msg),
                404 => ApiError::NotFound(error_msg),
                429 => ApiError::RateLimit(error_msg),
                503 => ApiError::Unavailable(error_msg),
                _ if status.is_server_error() => ApiError::Unknown(error_msg),
                _ => ApiError::Unknown(format!("Http {}: {}", status, error_msg)),
            };
            Err(CrateError::Api(api_error))
        }
    }
}
