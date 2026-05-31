use std::collections::HashMap;

use serde_json::{Value, Map};
use reqwest::Method;

use crate::{Client, Result};
use crate::{get_string, get_i64, get_array};

const GAMEMODES_ENDPOINT: &str = "https://api.brawlstars.com/v1/gamemodes";

pub struct EventsAPI {
    inner: Client
}

impl EventsAPI {
    pub(crate) fn new(client: Client) -> Self {
        Self { inner: client }
    }

    pub async fn get_gamemodes(&self) -> Result<HashMap<i64, Option<String>>> {
        let value = self.inner.request(
            GAMEMODES_ENDPOINT, 
            Method::GET, 
            HashMap::new()
        ).await?;

        let mut ret = HashMap::new();

        let obj = value
            .as_object()
            .ok_or("Strange Response")?;
        
        let obj_vec = get_array!(&obj, "items").ok_or("Strange Response")?;

        for item in obj_vec {
            let obj = item
                .as_object()
                .ok_or("Strange Response")?;

            let id = get_i64!(&obj, "id").ok_or("Strange Response")?;
            let name = get_string!(&obj, "name").map(|s| s.to_string());

            ret.insert(id, name);
        }

        Ok(ret)
    }
}