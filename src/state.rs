use cosmwasm_std::{coin, Coin, Timestamp};

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
    pub reward: Coin,         // rewards coin denom & amount
    pub cycle: u64,           // reward cycle
    pub is_whitelisted: bool, // is whitelisted for staking
    pub spots: u64,           // available spots
    pub pool_amount: u128,    // Reward pool INJ amount
    pub lockup_period: u64,
}
impl Collection {
    pub fn default() -> Self {
        Collection {
            reward: coin(0, "inj"),
            cycle: 604_800, // 1 week = 7 * 24 * 60 * 60
            is_whitelisted: true,
            spots: 0,
            lockup_period: 2_592_000, // 30 days = 30 * 24 * 60 * 60
            pool_amount: 0,
        }
    }
    pub fn new(
        reward: Coin,
        cycle: u64,
        is_whitelisted: bool,
        spots: u64,
        lockup_period: u64,
        pool_amount: u128,
    ) -> Self {
        Collection {
            reward,
            cycle,
            is_whitelisted,
            spots,
            lockup_period,
            pool_amount,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Staking {
    pub token_address: String, // nft collection ca
    pub token_id: String,
    pub start_timestamp: Timestamp,
    pub end_timestamp: Timestamp,
    pub is_paid: bool,
}
impl Staking {
    pub fn default() -> Self {
        Staking {
            token_address: String::from_str("").unwrap(),
            token_id: String::from_str("").unwrap(),
            start_timestamp: Timestamp::from_seconds(0),
            end_timestamp: Timestamp::from_seconds(0),
            is_paid: false,
        }
    }
    pub fn new(
        token_address: String,
        token_id: String,
        start_timestamp: Timestamp,
        is_paid: bool,
    ) -> Self {
        Staking {
            token_address,
            token_id,
            start_timestamp,
            end_timestamp: Timestamp::from_seconds(0),
            is_paid,
        }
    }
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const COLLECTIONS: Map<String, Collection> = Map::new("collections");
pub const STAKINGS: Map<String, Vec<Staking>> = Map::new("stakings");
