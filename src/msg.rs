use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Empty, Timestamp};
use cw721::Cw721ReceiveMsg;
use serde::Deserialize;

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
        lockup_period: u64,
    },
    DepositCollectionReward {
        address: String,
    },
    ReceiveNft(Cw721ReceiveMsg),
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
    #[returns(Vec<CollectionTokensResponse>)]
    GetAllCollectionTokensByOwner { owner: String },
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
    pub name: String,
    pub symbol: String,
    pub num_tokens: u64,
    pub staked: u64,
    pub lockup_period: u64,
}

#[cw_serde]
pub struct CollectionTokensResponse {
    pub token_address: String, // nft collection ca
    pub tokens: Vec<TokenResponse>,
}

#[cw_serde]
pub struct TokenResponse {
    pub token_id: String,
    pub token_uri: Option<String>,
    pub staking_state: Option<StakingStateResponse>,
}

#[cw_serde]
pub struct StakingStateResponse {
    pub index: u64,
    pub start_timestamp: Timestamp,
    pub end_timestamp: Timestamp,
    pub is_paid: bool,
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
