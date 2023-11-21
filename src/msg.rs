use cosmwasm_std::{Coin, Timestamp};
use cw721::Cw721ReceiveMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    TransferOwnership {
        address: String,
    },
    WhitelistCollection {
        address: String,
        reward: Coin,
        cycle: u64,
        is_whitelisted: bool,
    },
    ReceiveNft(Cw721ReceiveMsg),
    Unstake {
        index: u128,
    },
    ClaimReward {
        index: u128,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetConfig {},
    GetCollections {},
    GetStakingsByOwner { owner: String },
}

// responses
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ConfigResponse {
    pub owner: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CollectionResponse {
    pub address: String,
    pub reward: Coin,
    pub cycle: u64,
    pub is_whitelisted: bool,
}
