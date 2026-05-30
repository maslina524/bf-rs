use std::sync::Arc;
use reqwest;

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
}
