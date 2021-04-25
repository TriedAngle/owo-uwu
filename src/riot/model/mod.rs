use serde::{Deserialize, Serialize};

pub mod summoner;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Region {
    EUW,
    KR,
    EUNE,
    NA,
    OCE,
    JP,
    RU,
    TR,
    LA1,
    LA2,
}

impl From<Region> for String {
    fn from(rhs: Region) -> Self {
        match rhs {
            Region::EUW => "EUW1",
            Region::KR => "KR",
            Region::EUNE => "EUN1",
            Region::NA => "NA1",
            Region::JP => "JP1",
            Region::OCE => "OC1",
            Region::RU => "RU",
            Region::TR => "TR1",
            Region::LA1 => "LA1",
            Region::LA2 => "LA2",
        }
            .into()
    }
}

impl From<String> for Region {
    fn from(rhs: String) -> Self {
        match rhs.as_str() {
            "EUW1" => Region::EUW,
            "KR" => Region::KR,
            "EUN1" => Region::EUNE,
            "NA1" => Region::NA,
            "JP1" => Region::JP,
            "OC1" => Region::OCE,
            "RU" => Region::RU,
            "TR1" => Region::TR,
            "LA1" => Region::LA1,
            "LA2" => Region::LA2,
            _ => panic!("invalid")
        }
    }
}