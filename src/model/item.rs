use super::{item_common::CommonItem, kind::Kind};
use crate::{client::Client, Result};

use std::collections::HashMap;

use awc::http::PathAndQuery;
use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{de::DeserializeOwned, Deserialize};

const ENDPOINT_ITEM: &str = "/item";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TokenResponse {
    pub token: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemIndex {
    pub total: i64,
    #[serde(with = "ts_seconds")]
    pub modified: DateTime<Utc>,
    pub kinds: HashMap<String, KindProperties>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KindProperties {
    pub count: i64,
    #[serde(with = "ts_seconds")]
    pub modified: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemResult<T: Item> {
    pub total: i64,
    pub items: Vec<T>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Grid {
    pub color: RGBA,
    pub height: i64,
    pub width: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RGBA {
    pub r: u64,
    pub g: u64,
    pub b: u64,
    pub a: u64,
}

pub trait Item {}

impl Client {
    pub async fn get_item_index(&self) -> Result<ItemIndex> {
        let resp = self.get_json(ENDPOINT_ITEM.parse().unwrap()).await?;

        Ok(resp)
    }

    pub async fn get_items_by_kind<T: Item + DeserializeOwned>(
        &self,
        kind: &str,
        limit: i64,
        offset: i64,
    ) -> Result<ItemResult<T>> {
        let path: PathAndQuery = format!(
            "{}/{}?limit={}&offset={}",
            ENDPOINT_ITEM,
            kind.to_path(),
            limit,
            offset
        )
        .parse()
        .unwrap();

        let resp = self.get_json(path).await?;

        Ok(resp)
    }

    pub async fn get_all_items(&self) -> Result<Vec<CommonItem>> {
        let index = self.get_item_index().await?;

        let limit = Self::MAX_PAGE_LIMIT;

        let mut items: Vec<CommonItem> = Vec::with_capacity(index.total as usize);
        for (k, p) in index.kinds.into_iter() {
            let pages = if (p.count % limit) != 0 {
                (p.count / limit) + 1
            } else {
                p.count / limit
            };
            for i in 0..=(pages - 1) {
                let offset = i * limit;
                items.append(&mut self.get_items_by_kind(&k, limit, offset).await?.items);
            }
        }

        Ok(items)
    }
}
