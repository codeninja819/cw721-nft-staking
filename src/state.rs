use cosmwasm_std::{coin, Addr, Coin, DepsMut, MessageInfo, Timestamp};
use cw721::Expiration;
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Collection {
    pub address: String,      // nft collection ca
    pub reward: Coin,         // rewards coin denom & amount
    pub cycle: u64,           // reward cycle
    pub is_whitelisted: bool, // is whitelisted for staking
}
impl Collection {
    pub fn default() -> Self {
        Collection {
            address: String::from_str("").unwrap(),
            reward: coin(0, "inj"),
            cycle: 604_800, // 1 week = 7 * 24 * 60 * 60 * 1000,
            is_whitelisted: true,
        }
    }
    pub fn new(address: String, reward: Coin, cycle: u64) -> Self {
        Collection {
            address,
            reward,
            cycle,
            is_whitelisted: true,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Staking {
    pub owner: String, // nft collection ca
    pub token_id: String,
    pub start_timestamp: Timestamp,
    pub end_timestamp: Timestamp,
    pub is_paid: bool,
}
impl Staking {
    pub fn default() -> Self {
        Staking {
            owner: String::from_str("").unwrap(),
            token_id: String::from_str("").unwrap(),
            start_timestamp: Timestamp::from_seconds(0),
            end_timestamp: Timestamp::from_seconds(0),
            is_paid: false,
        }
    }
    pub fn new(owner: String, token_id: String, start_timestamp: Timestamp) -> Self {
        Staking {
            owner,
            token_id,
            start_timestamp,
            end_timestamp: Timestamp::from_seconds(0),
            is_paid: false,
        }
    }
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const COLLECTIONS: Map<String, Collection> = Map::new("collections");
pub const STAKINGS: Map<String, Vec<Staking>> = Map::new("stakings");
