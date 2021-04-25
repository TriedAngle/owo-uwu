use serde::{Serialize, Deserialize};
use crate::riot::model::Region;

#[derive(Debug, Serialize, Deserialize)]
pub struct Summoner {
    pub id: i32,
    pub summoner_id: String,
    pub account_id: String,
    pub puuid: String,
    pub name: String,
    pub icon_id: String,
    pub region: Region,
}

pub struct NewSummoner {
    pub summoner_id: String,
    pub account_id: String,
    pub puuid: String,
    pub name: String,
    pub icon_id: String,
    pub region: Region,
}

#[derive(Debug, Deserialize)]
pub struct SummonerDTO {
    pub account_id: String,
    pub profile_icon_id: i32,
    pub revision_date: i64,
    pub name: String,
    pub id: String,
    pub puuid: String,
    pub summoner_level: i64,
}

#[derive(Debug, Deserialize)]
pub struct LeagueEntryDTO {
    pub league_id: String,
    pub summoner_id: String,
    pub summoner_name: String,
    pub queue_type: String,
    pub tier: String,
    pub rank: String,
    pub league_points: i32,
    pub wins: i32,
    pub losses: i32,
    pub hot_streak: bool,
    pub veteran: bool,
    pub fresh_blood: bool,
    pub inactive: bool,
}

#[derive(Debug, Deserialize)]
pub struct MiniSeriesDTO {
    pub losses: i32,
    pub progress: String,
    pub target: i32,
    pub wins: i32,
}