use std::collections::HashMap;

use reqwest::Method;

use crate::{Client, Result};

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
        ).await.unwrap();

        let mut ret = HashMap::new();

        let obj_vec = value
            .as_object()
            .ok_or("Strange Response")?
            .get("items")
            .ok_or("Strange Response")?
            .as_array()
            .ok_or("Strange Response")?;

        for item in obj_vec {
            let obj = item
                .as_object()
                .ok_or("Strange Response")?;

            let id = obj
                .get("id")
                .ok_or("Strange Response")?
                .as_number()
                .ok_or("Strange Response")?
                .as_i64()
                .ok_or("Strange Response")?;

            let name = obj
                .get("name")
                .ok_or("Strange Response")?
                .as_str()
                .map(|s| s.to_string());

            ret.insert(id, name);
        }

        Ok(ret)
    }
}