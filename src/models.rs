use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

#[derive(Debug, Clone)]
pub enum Client {
    Desktop = 0,
    Mobile = 1,
    Unknown = 99999,
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
    pub asset_type: usize,
    pub content: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChangelogConfig {
    pub min_version: usize,
}
