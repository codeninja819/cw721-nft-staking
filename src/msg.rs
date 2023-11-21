use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Timestamp};
use cw721::Cw721ReceiveMsg;

#[cw_serde]
pub struct InstantiateMsg {}

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
    },
    ReceiveNft(Cw721ReceiveMsg),
    Unstake {
        index: u128,
    },
    ClaimReward {
        index: u128,
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
}

#[cw_serde]
pub struct CollectionResponse {
    pub address: String,
    pub reward: Coin,
    pub cycle: u64,
    pub is_whitelisted: bool,
}

#[cw_serde]
pub struct StakingResponse {
    pub token_address: String, // nft collection ca
    pub token_id: String,
    pub start_timestamp: Timestamp,
    pub end_timestamp: Timestamp,
    pub is_paid: bool,
}
