use std::collections::HashMap;
use std::str::FromStr;

use reqwest::Method;
use thiserror::Error;
use serde_json::{Value, Map};

use crate::{BsResult, Client};
use crate::{get_object, get_string, get_i64, get_array, get_bool};

const INFO_ENDPOINT: &str = "https://api.brawlstars.com/v1/players";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NameColor {
    Red, Green
}

impl FromStr for NameColor {
    type Err = ();
    fn from_str(_string: &str) -> Result<Self, Self::Err> {
        Ok(NameColor::Red)
    }
}

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
    tag: Tag,
    name: String,
    name_color: NameColor,
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
    club_tag: Tag,
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

    pub async fn info(&self, tag: Tag) -> BsResult<PlayerState> {
        let value = self.inner.request(
            &format!("{}/{}", INFO_ENDPOINT, tag.get_http()), 
            Method::GET, 
            HashMap::new()
        ).await?;

        let obj = value
            .as_object()
            .ok_or("Strange Response")?;

        let name = get_string!(&obj, "name")?.to_string();

        let name_color_str = get_string!(&obj, "nameColor")?;
        let name_color = NameColor::Green;
        
        let icon = get_object!(&obj, "icon")?;
        let icon_id = get_i64!(&icon, "id")? as u32;

        let trophies = get_i64!(&obj, "trophies")? as u32;
        let h_trophies = get_i64!(&obj, "highestTrophies")? as u32;

        let prestige_level = get_i64!(&obj, "totalPrestigeLevel")? as u32;
        let exp_level = get_i64!(&obj, "expLevel")? as u32;
        let exp_points = get_i64!(&obj, "expPoints")? as u32;

        let qualified_for_championship = get_bool!(&obj, "isQualifiedFromChampionshipChallenge")?;

        let trio_victories = get_i64!(&obj, "3vs3Victories")? as u32;
        let solo_victories = get_i64!(&obj, "soloVictories")? as u32;
        let duo_victories = get_i64!(&obj, "duoVictories")? as u32;

        let robo_rumble_best_time = get_i64!(&obj, "bestRoboRumbleTime")? as u16;
        let big_brawler_best_time = get_i64!(&obj, "bestTimeAsBigBrawler")? as u16;

        let ranked_season_id = get_i64!(&obj, "rankedSeasonId")? as u8;
        let rank_int = get_i64!(&obj, "rankedSeasonId")? as u8;
        let rank = RankedRank::Pro;
        let elo = get_i64!(&obj, "rankedElo")? as u16;
        
        let h_season_rank_int = get_i64!(&obj, "highestSeasonRankedRank")? as u8;
        let h_season_rank = RankedRank::Pro;
        let h_season_elo = get_i64!(&obj, "highestSeasonRankedElo")? as u16;

        let h_all_time_rank_int = get_i64!(&obj, "highestAllTimeRankedRank")? as u8;
        let h_all_time_rank = RankedRank::Pro;
        let h_all_time_elo = get_i64!(&obj, "highestAllTimeRankedElo")? as u16;

        let club = get_object!(&obj, "club")?;
        let club_tag_str = get_string!(&club, "tag")?.to_string();
        let club_tag = Tag::new(club_tag_str).unwrap();
        let club_name = get_string!(&club, "name")?.to_string();

        Ok(
            PlayerState {
                tag, name, name_color, icon_id,
                trophies, h_trophies, prestige_level,
                exp_level, exp_points, qualified_for_championship,
                trio_victories, solo_victories, duo_victories,
                robo_rumble_best_time, big_brawler_best_time,
                ranked_season_id, rank, elo, h_season_rank,
                h_season_elo, h_all_time_rank, h_all_time_elo,
                club_tag, club_name, brawlers: Vec::new()
            }
        )
    }
}