use std::collections::HashMap;
use chrono::{DateTime, Utc, NaiveDateTime};

use serde_json::{Value, Map};
use reqwest::Method;

use crate::{Client, BsResult};
use crate::{get_object, get_string, get_i64, get_array};

const GAMEMODES_ENDPOINT: &str = "https://api.brawlstars.com/v1/gamemodes";
const ROTATION_ENDPOINT: &str = "https://api.brawlstars.com/v1/events/rotation";

#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    id: u32,
    map: String,
    mode_str: String,
    mode_id: i64
}

#[derive(Debug, Clone, PartialEq)]
pub struct EventSlot {
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    slot_id: u32,
    event: Event
}

pub struct EventsAPI {
    inner: Client
}

impl EventsAPI {
    pub(crate) fn new(client: Client) -> Self {
        Self { inner: client }
    }

    pub async fn gamemodes(&self) -> BsResult<HashMap<i64, Option<String>>> {
        let value = self.inner.request(
            GAMEMODES_ENDPOINT, 
            Method::GET, 
            HashMap::new()
        ).await?;

        let mut ret = HashMap::new();

        let obj = value
            .as_object()
            .ok_or("Strange Response")?;
        
        let obj_vec = get_array!(&obj, "items")?;

        for item in obj_vec {
            let obj = item
                .as_object()
                .ok_or("Strange Response")?;

            let id = get_i64!(&obj, "id")?;
            let name = get_string!(&obj, "name").map(|s| s.to_string()).ok();

            ret.insert(id, name);
        }

        Ok(ret)
    }

    pub async fn rotation(&self) -> BsResult<Vec<EventSlot>> {
        let value = self.inner.request(
            ROTATION_ENDPOINT, 
            Method::GET, 
            HashMap::new()
        ).await?;
        // println!("{value:#?}");
        let mut ret = Vec::new();
        let vec = value
            .as_array()
            .ok_or("Strange Response")?;

        for value in vec {
            let obj = value
                .as_object()
                .ok_or("Strange Response")?;

            let start_time_str = get_string!(&obj, "startTime")?;
            let start_time = date_time_from_str(start_time_str);

            let end_time_str = get_string!(&obj, "endTime")?;
            let end_time = date_time_from_str(end_time_str);

            let slot_id = get_i64!(&obj, "slotId")? as u32;

            let event_obj = get_object!(&obj, "event")?;

            let id = get_i64!(&event_obj, "id")? as u32;
            let map = get_string!(&event_obj, "map")?.to_string();
            let mode_str = get_string!(&event_obj, "mode")?.to_string();
            let mode_id = get_i64!(&event_obj, "modeId")?;

            let event = Event { id, map, mode_str, mode_id };
            let event_slot = EventSlot { start_time, end_time, slot_id, event };

            ret.push(event_slot);
        }

        Ok(ret)
    }
}

fn date_time_from_str(string: &str) -> DateTime<Utc> {
    DateTime::<Utc>::from_naive_utc_and_offset(
        NaiveDateTime::parse_from_str(string, "%Y%m%dT%H%M%S%.3fZ").unwrap(),
        Utc,
    )
}