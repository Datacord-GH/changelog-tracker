use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

#[derive(Debug, Clone)]
pub enum Client {
    Desktop = 0,
    Mobile = 1,
    Unknown = 99999,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssetType {
    YouTube = 0,
    Image = 1,
    Unknown = 9999,
}

#[derive(Debug, Clone)]
pub struct ChangelogDB {
    pub changelog_id: String,
    pub client: usize,
    pub locale: String,
    pub date: String,
    pub asset: String,
    pub asset_type: usize,
    pub content: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Changelog {
    #[serde(deserialize_with = "deserialize_string_from_number")]
    pub changelog_id: String,
    #[serde(deserialize_with = "deserialize_string_from_number")]
    pub entry_id: String,
    pub locale: String,
    pub date: String,
    pub asset: Option<String>,
    pub asset_type: AssetType,
    pub content: String,
}

impl Changelog {
    pub fn convert_from_reqwest(changelog_reqwest: ChangelogReqwest) -> Changelog {
        let asset_type = match changelog_reqwest.asset_type {
            0 => AssetType::YouTube,
            1 => AssetType::Image,
            _ => AssetType::Unknown,
        };

        Changelog {
            asset: changelog_reqwest.asset,
            changelog_id: changelog_reqwest.changelog_id,
            content: changelog_reqwest.content,
            date: changelog_reqwest.date,
            entry_id: changelog_reqwest.entry_id,
            locale: changelog_reqwest.locale,
            asset_type,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangelogReqwest {
    #[serde(deserialize_with = "deserialize_string_from_number")]
    pub changelog_id: String,
    #[serde(deserialize_with = "deserialize_string_from_number")]
    pub entry_id: String,
    pub locale: String,
    pub date: String,
    pub asset: Option<String>,
    pub asset_type: u8,
    pub content: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChangelogConfig {
    pub min_version: usize,
    pub show_on_startup: bool,
}
