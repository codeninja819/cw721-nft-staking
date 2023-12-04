use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, Coin, Empty, Timestamp};
use cw721::Cw721ReceiveMsg;
use serde::{Deserialize, Serialize};

#[cw_serde]
pub struct InstantiateMsg {
    pub unstake_fee: Coin,
}

#[cw_serde]
pub enum ExecuteMsg {
    TransferOwnership {
        address: String,
    },
    WhitelistCollection {
        address: String,
        reward: Coin,
        cycle: u64,
        is_whitelisted: bool,
        spots: u64,
    },
    DepositCollectionReward {
        address: String,
    },
    ReceiveNft(UniversalNftReceiveMsg),
    Unstake {
        index: u64,
    },
    ClaimReward {
        index: u64,
    },
    WithdrawFee {
        fee: Coin,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    GetConfig {},
    #[returns(Vec<CollectionResponse>)]
    GetCollections {},
    #[returns(Vec<StakingResponse>)]
    GetStakingsByOwner { owner: String },
}

// responses
#[cw_serde]
pub struct ConfigResponse {
    pub owner: String,
    pub unstake_fee: Coin,
}

#[cw_serde]
pub struct CollectionResponse {
    pub address: String,
    pub reward: Coin,
    pub cycle: u64,
    pub is_whitelisted: bool,
    pub spots: u64,
}

#[cw_serde]
pub struct StakingResponse {
    pub token_address: String, // nft collection ca
    pub token_id: String,
    pub start_timestamp: Timestamp,
    pub end_timestamp: Timestamp,
    pub is_paid: bool,
}

#[derive(Deserialize)]
pub struct UniversalNftInfoResponse {
    pub token_uri: Option<String>,

    #[serde(skip_deserializing)]
    #[allow(dead_code)]
    extension: Empty,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct UniversalNftReceiveMsg {
    pub sender: String,
    pub token_id: String,
    pub msg: Binary,

    #[serde(skip_deserializing)]
    #[allow(dead_code)]
    pub edition: Option<u64>,
}

#[derive(Serialize, Deserialize)]
pub struct TalisCw721ReceiveMsg {
    pub sender: String,
    pub token_id: String,
    pub msg: Binary,
    pub edition: Option<u64>,
}
