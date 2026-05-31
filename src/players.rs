use std::collections::HashMap;

use reqwest::Method;
use thiserror::Error;

use crate::{Client, BsResult};
use crate::{get_object, get_string, get_i64, get_array};

const INFO_ENDPOINT: &str = "https://api.brawlstars.com/v1/players/";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RankedRank {
    Bronze(RankTier),
    Silver(RankTier),
    Gold(RankTier),
    Mythic(RankTier),
    Legendary(RankTier),
    Masters(RankTier),
    Pro
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RankTier {
    I, II, III
}

#[derive(Debug, Clone, PartialEq)]
pub struct BrawlerState {}

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerState {
    tag: String,
    name: String,
    icon_id: u32,
    trophies: u32,
    h_trophies: u32,
    prestige_level: u32,
    exp_level: u32,
    exp_points: u32,
    qualified_for_championship: bool,
    trio_victories: u32,
    solo_victories: u32,
    duo_victories: u32,
    robo_rumble_best_time: u16,
    big_brawler_best_time: u16,
    ranked_season_id: u8,
    rank: RankedRank,
    elo: u16,
    h_season_rank: RankedRank,
    h_season_elo: u16,
    h_all_time_rank: RankedRank,
    h_all_time_elo: u16,
    club_tag: String,
    club_name: String,
    brawlers: Vec<BrawlerState>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum TagError {
    #[error("A Hash Symbol Is Required")]
    HashSymbRequired,
    #[error("Tag Is Empty")]
    TagIsEmpty,
    #[error("Tag Body Is Incorrect")]
    TagBodyIsIncorrect,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tag {
    value: String
}

fn tag_body_is_correct(s: &String) -> bool {
    s.chars().all(|c| c.is_ascii_alphabetic() || c.is_ascii_digit())
}

impl Tag {
    pub fn new(value: impl Into<String>) -> Result<Self, TagError> {
        let mut string = value.into();
        
        if !string.starts_with('#') {
            return Err(TagError::HashSymbRequired);
        }
        
        string = string.chars().skip(1).collect();
        
        if string.is_empty() {
            return Err(TagError::TagIsEmpty);
        }
        
        if !tag_body_is_correct(&string) {
            return Err(TagError::TagBodyIsIncorrect);
        }
        
        Ok(Self { value: string.to_uppercase() })
    }
 
    pub fn get(&self) -> String {
        format!("#{}", self.value)
    }

    pub fn get_http(&self) -> String {
        format!("%23{}", self.value)
    }
}

pub struct PlayersAPI {
    inner: Client
}

impl PlayersAPI {
    pub(crate) fn new(client: Client) -> Self {
        Self { inner: client }
    }
}