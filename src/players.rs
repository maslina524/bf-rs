use std::collections::HashMap;
use std::marker::PhantomData;
use std::str::FromStr;

use reqwest::Method;
use thiserror::Error;
use serde_json::{Value, Map};

use crate::{BsResult, Client};
use crate::{get_object, get_string, get_i64, get_array, get_bool};

const INFO_ENDPOINT: &str = "https://api.brawlstars.com/v1/players";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NameColor {
    White,      // 0xffffffff
    Cyan,       // 0xffa2e3fe
    Emerald,    // 0xff4ddba2
    Orange,     // 0xffff9727
    DarkOrange, // 0xfff9775d
    Red,        // 0xfff05637
    Yellow,     // 0xfff9c908
    DarkYellow, // 0xffffce89
    Green,      // 0xffa8e132
    Blue,       // 0xff1ba5f5
    Pink,       // 0xffff8afb
    Purple      // 0xffcb5aff
}

impl NameColor {
    fn get_hex(&self) -> u32 {
        match self {
            Self::White      => 0xffffff,
            Self::Cyan       => 0xa2e3fe,
            Self::Emerald    => 0x4ddba2,
            Self::Orange     => 0xff9727,
            Self::DarkOrange => 0xf9775d,
            Self::Red        => 0xf05637,
            Self::Yellow     => 0xf9c908,
            Self::DarkYellow => 0xffce89,
            Self::Green      => 0xa8e132,
            Self::Blue       => 0x1ba5f5,
            Self::Pink       => 0xff8afb,
            Self::Purple     => 0xcb5aff
        }
    }
}

impl FromStr for NameColor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("0x") || !is_hex_string(&s[2..]) || s.len() < 8 {
            return Err(());
        }
        let s = &s[(s.len() - 6)..].parse::<u32>().map_err(|_| ())?;
        let ret = match s {
            0xffffff => Self::White,
            0xa2e3fe => Self::Cyan,
            0x4ddba2 => Self::Emerald,
            0xff9727 => Self::Orange,
            0xf9775d => Self::DarkOrange,
            0xf05637 => Self::Red,
            0xf9c908 => Self::Yellow,
            0xffce89 => Self::DarkYellow,
            0xa8e132 => Self::Green,
            0x1ba5f5 => Self::Blue,
            0xff8afb => Self::Pink,
            0xcb5aff => Self::Purple,
            _ => return Err(())
        };

        Ok(ret)
    }
}

fn is_hex_string(s: &str) -> bool {
    for ch in s.chars() {
        if !ch.is_ascii_hexdigit() {
            return false;
        }
    }
    true
}

impl FromStr for NameColor {
    type Err = ();
    fn from_str(_string: &str) -> Result<Self, Self::Err> {
        Ok(NameColor::White)
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
pub struct BrawlerSkinType;

#[derive(Debug, Clone, PartialEq)]
pub struct BrawlerGadgetType;

#[derive(Debug, Clone, PartialEq)]
pub struct BrawlerGearType;

#[derive(Debug, Clone, PartialEq)]
pub struct BrawlerStarType;

#[derive(Debug, Clone, PartialEq)]
pub struct BrawlerHyperType;

#[derive(Debug, Clone, PartialEq)]
struct NamedThing<T> {
    id: u32,
    name: String,
    _marker: std::marker::PhantomData<T>
}

impl<T> NamedThing<T> {
    pub fn new(id: u32, name: String) -> Self {
        Self {
            id,
            name,
            _marker: PhantomData,
        }
    }
    
    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

type BrawlerSkin = NamedThing<BrawlerSkinType>;
type BrawlerGadget = NamedThing<BrawlerGadgetType>;
type BrawlerGear = NamedThing<BrawlerGearType>;
type BrawlerStar = NamedThing<BrawlerStarType>;
type BrawlerHyper = NamedThing<BrawlerHyperType>;

#[derive(Debug, Clone, PartialEq)]
pub struct BrawlerState {
    pub id: u32,
    pub name: String,
    pub power: u8,
    pub rank: u8,
    pub trophies: u16,
    pub h_trophies: u16,
    pub prestige: u8,
    pub win_streak: u16,
    pub max_win_streak: u16,
    pub skin: Option<BrawlerSkin>,
    pub gadgets: Vec<BrawlerGadget>,
    pub gears: Vec<BrawlerGear>,
    pub star_powers: Vec<BrawlerStar>,
    pub hyper_charge: Option<BrawlerHyper>,
    pub gadget_buffie: bool,
    pub star_buffie: bool,
    pub hyper_buffie: bool
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerState {
    pub tag: Tag,
    pub name: String,
    pub name_color: String,
    pub icon_id: u32,
    pub trophies: u32,
    pub h_trophies: u32,
    pub prestige_level: u32,
    pub exp_level: u32,
    pub exp_points: u32,
    pub qualified_for_championship: bool,
    pub trio_victories: u32,
    pub solo_victories: u32,
    pub duo_victories: u32,
    pub robo_rumble_best_time: u16,
    pub big_brawler_best_time: u16,
    pub ranked_season_id: u8,
    pub rank: RankedRank,
    pub elo: u16,
    pub h_season_rank: RankedRank,
    pub h_season_elo: u16,
    pub h_all_time_rank: RankedRank,
    pub h_all_time_elo: u16,
    pub club_tag: Option<Tag>,
    pub club_name: Option<String>,
    pub brawlers: Vec<BrawlerState>
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

        let name_color = get_string!(&obj, "nameColor")?.to_string();
        // let name_color = NameColor::Green;
        
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
            brawlers.push(read_brawler(brawler)?);
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

fn read_brawler(brawler: &Map<String, Value>) -> BsResult<BrawlerState> {
    let id = get_i64!(&brawler, "id")? as u32;
    let name = get_string!(&brawler, "name")?.to_string();
    let power = get_i64!(&brawler, "power")? as u8;
    let rank = get_i64!(&brawler, "rank")? as u8;
    let trophies = get_i64!(&brawler, "trophies")? as u16;
    let h_trophies = get_i64!(&brawler, "highestTrophies")? as u16;
    let prestige = get_i64!(&brawler, "prestigeLevel")? as u8;
    let win_streak = get_i64!(&brawler, "currentWinStreak")? as u16;
    let max_win_streak = get_i64!(&brawler, "maxWinStreak")? as u16;

    let skin_obj = get_object!(&brawler, "skin")?;
    let skin_id = get_i64!(&skin_obj, "id")? as u32;
    let skin_name = get_string!(&skin_obj, "name")?;
    let skin: Option<BrawlerSkin> = if skin_name == &name {
        None
    } else {
        Some(
            BrawlerSkin::new(skin_id, skin_name.to_string())
        )
    };

    let mut gadgets: Vec<BrawlerGadget> = Vec::new();
    let gadgets_vec = get_array!(&brawler, "gadgets")?;
    for cur_gadget in gadgets_vec {
        let gadget_obj = cur_gadget.as_object().ok_or("Strange Response")?;
        let gadget_id = get_i64!(&gadget_obj, "id")? as u32;
        let gadget_name = get_string!(&gadget_obj, "name")?.to_string();
        gadgets.push(BrawlerGadget::new(gadget_id, gadget_name));
    }

    let mut gears: Vec<BrawlerGear> = Vec::new();
    let gears_vec = get_array!(&brawler, "gears")?;
    for cur_gear in gears_vec {
        let gear_obj = cur_gear.as_object().ok_or("Strange Response")?;
        let gear_id = get_i64!(&gear_obj, "id")? as u32;
        let gear_name = get_string!(&gear_obj, "name")?.to_string();
        gears.push(BrawlerGear::new(gear_id, gear_name));
    }

    let mut star_powers: Vec<BrawlerStar> = Vec::new();
    let stars_vec = get_array!(&brawler, "starPowers")?;
    for cur_star in stars_vec {
        let star_obj = cur_star.as_object().ok_or("Strange Response")?;
        let star_id = get_i64!(&star_obj, "id")? as u32;
        let star_name = get_string!(&star_obj, "name")?.to_string();
        star_powers.push(BrawlerStar::new(star_id, star_name));
    }

    let hyper_vec = get_array!(&brawler, "hyperCharges")?;
    let hyper_charge = if let Some(value) = hyper_vec.get(0) {
        let hyper_obj = value.as_object().ok_or("Strange Response")?;
        let hyper_id = get_i64!(&hyper_obj, "id")? as u32;
        let hyper_name = get_string!(&hyper_obj, "name")?.to_string();
        Some(
            BrawlerHyper::new(hyper_id, hyper_name)
        )
    } else { None };

    let buffies_obj = get_object!(&brawler, "buffies")?;
    let gadget_buffie = get_bool!(&buffies_obj, "gadget")?;
    let star_buffie = get_bool!(&buffies_obj, "starPower")?;
    let hyper_buffie = get_bool!(&buffies_obj, "hyperCharge")?;

    Ok(
        BrawlerState { 
            id, name, power, rank, trophies, h_trophies, prestige,
            win_streak, max_win_streak, skin, gadgets, gears, star_powers,
            hyper_charge, gadget_buffie, star_buffie, hyper_buffie
        }
    )
}