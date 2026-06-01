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
    Diamond(RankTier),
    Mythic(RankTier),
    Legendary(RankTier),
    Masters(RankTier),
    Pro
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RankTier {
    I, II, III
}

impl RankedRank {
    pub fn from(i: u8) -> Self {
        let i = i - 1;
        let tier_int = i % 3;
        let tier = match tier_int {
            0 => RankTier::I,
            1 => RankTier::II,
            2 => RankTier::III,
            _ => unreachable!("Incorrect int value for Rank")
        };

        let rank_int = i / 3;
        let rank = match rank_int {
            0 => RankedRank::Bronze(tier),
            1 => RankedRank::Silver(tier),
            2 => RankedRank::Gold(tier),
            3 => RankedRank::Diamond(tier),
            4 => RankedRank::Mythic(tier),
            5 => RankedRank::Legendary(tier),
            6 => RankedRank::Masters(tier),
            7 => RankedRank::Pro,
            _ => unreachable!("Incorrect int value for Rank")
        };

        rank
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BrawlerState {
    id: u32,
    name: String,
    power: u8,
    rank: u8,
    trophies: u16,
    h_trophies: u16,
    prestige: u8,
    win_streak: u16,
    max_win_streak: u16,
    skin: BrawlerSkin,
    gadgets: Vec<BrawlerGadget>,
    gears: Vec<BrawlerGear>,
    star_powers: Vec<BrawlerStar>,
    hyper_charge: BrawlerHyper,
    gadget_buffie: bool,
    star_buffie: bool,
    hyper_buffie: bool
}

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
    club_tag: Option<Tag>,
    club_name: Option<String>,
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

        let qualified_for_championship = get_bool!(&obj, "isQualifiedFromChampionshipChallenge").unwrap_or_default();

        let trio_victories = get_i64!(&obj, "3vs3Victories")? as u32;
        let solo_victories = get_i64!(&obj, "soloVictories")? as u32;
        let duo_victories = get_i64!(&obj, "duoVictories")? as u32;

        let robo_rumble_best_time = get_i64!(&obj, "bestRoboRumbleTime")? as u16;
        let big_brawler_best_time = get_i64!(&obj, "bestTimeAsBigBrawler")? as u16;

        let ranked_season_id = get_i64!(&obj, "rankedSeasonId")? as u8;
        let rank_int = get_i64!(&obj, "rankedRank")? as u8;
        let rank = RankedRank::from(rank_int);
        let elo = get_i64!(&obj, "rankedElo")? as u16;
        
        let h_season_rank_int = get_i64!(&obj, "highestSeasonRankedRank")? as u8;
        let h_season_rank = RankedRank::from(h_season_rank_int);
        let h_season_elo = get_i64!(&obj, "highestSeasonRankedElo")? as u16;

        let h_all_time_rank_int = get_i64!(&obj, "highestAllTimeRankedRank")? as u8;
        let h_all_time_rank = RankedRank::from(h_all_time_rank_int);
        let h_all_time_elo = get_i64!(&obj, "highestAllTimeRankedElo")? as u16;

        let mut club_tag: Option<Tag> = None;
        let mut club_name: Option<String> = None;
        if let Ok(club) = get_object!(&obj, "club") {
            if let Ok(tag_str) = get_string!(&club, "tag") {
                club_tag = Tag::new(tag_str).ok();
            };

            club_name = get_string!(&club, "name").ok().map(|s| s.to_string());
        };

        let mut brawlers = Vec::new();
        let brawlers_obj = get_array!(&obj, "brawlers")?;
        for brawler in brawlers_obj {
            let brawler = brawler.as_object().ok_or("Strange Response")?;
            brawlers.push(read_brawler(brawler));
        }

        Ok(
            PlayerState {
                tag, name, name_color, icon_id,
                trophies, h_trophies, prestige_level,
                exp_level, exp_points, qualified_for_championship,
                trio_victories, solo_victories, duo_victories,
                robo_rumble_best_time, big_brawler_best_time,
                ranked_season_id, rank, elo, h_season_rank,
                h_season_elo, h_all_time_rank, h_all_time_elo,
                club_tag, club_name, brawlers
            }
        )
    }
}

fn read_brawler(brawler: &Map<String, Value>) -> BrawlerState {

}