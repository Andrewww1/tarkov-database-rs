use serde::Deserialize;
use super::{kind::Kind, item::{Item, Grid}};
use chrono::{serde::ts_seconds, DateTime, Utc};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonItem {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    pub short_name: String,
    pub description: String,
    pub price: i64,
    pub weight: f64,
    pub max_stack: i64,
    pub rarity: String,
    pub grid: Grid,
    #[serde(with = "ts_seconds", rename = "_modified")]
    pub modified: DateTime<Utc>,
    #[serde(rename = "_kind")]
    pub kind: Kind,
}

impl Item for CommonItem {}