use std::vec;

use cosmwasm_std::{to_json_binary, Deps, Empty, Env, Order, QueryResponse};

use crate::{
    error::ContractError,
    msg::{
        CollectionResponse, CollectionTokensResponse, ConfigResponse, StakingStateResponse,
        TokenResponse, UniversalNftInfoResponse,
    },
    state::{Staking, COLLECTIONS, CONFIG, STAKINGS},
};

pub fn get_config(deps: Deps) -> Result<QueryResponse, ContractError> {
    let config_state = CONFIG.load(deps.storage)?;
    let owner = config_state.owner;
    Ok(to_json_binary(&ConfigResponse { owner }).unwrap())
}

pub fn get_collections(deps: Deps, env: Env) -> Result<QueryResponse, ContractError> {
    let mut collections: Vec<CollectionResponse> = vec![];
    for k in COLLECTIONS.keys(deps.storage, None, None, Order::Ascending) {
        let address = k.unwrap();
        let collection = COLLECTIONS.load(deps.storage, address.clone()).unwrap();
        let contract_info: cw721::ContractInfoResponse = deps
            .querier
            .query_wasm_smart(address.clone(), &cw721::Cw721QueryMsg::ContractInfo {})?;
        let num_tokens: cw721::NumTokensResponse = deps
            .querier
            .query_wasm_smart(address.clone(), &cw721::Cw721QueryMsg::NumTokens {})?;
        let staked: cw721::TokensResponse = deps.querier.query_wasm_smart(
            address.clone(),
            &cw721::Cw721QueryMsg::Tokens {
                owner: env.contract.address.to_string(),
                start_after: None,
                limit: None,
            },
        )?;
        collections.push(CollectionResponse {
            address: address.clone(),
            reward: collection.reward,
            cycle: collection.cycle,
            is_whitelisted: collection.is_whitelisted,
            name: contract_info.clone().name,
            symbol: contract_info.clone().symbol,
            num_tokens: num_tokens.count,
            spots: collection.spots,
            staked: u64::from_str_radix(&staked.tokens.len().to_string(), 10).unwrap(),
        });
    }
    Ok(to_json_binary(&collections).unwrap())
}

pub fn get_stakings_by_owner(deps: Deps, owner: String) -> Result<QueryResponse, ContractError> {
    let stakings: Vec<Staking>;
    let stakings_state = STAKINGS.may_load(deps.storage, owner).unwrap();
    if stakings_state.is_some() {
        stakings = stakings_state.unwrap();
    } else {
        stakings = vec![];
    }
    Ok(to_json_binary(&stakings).unwrap())
}

pub fn get_all_collection_tokens_by_owner(
    deps: Deps,
    owner: String,
) -> Result<QueryResponse, ContractError> {
    let mut all_tokens: Vec<CollectionTokensResponse> = vec![];
    let stakings: Vec<Staking>;
    let stakings_state = STAKINGS.may_load(deps.storage, owner.clone()).unwrap();
    if stakings_state.is_some() {
        stakings = stakings_state.unwrap();
    } else {
        stakings = vec![];
    }
    for k in COLLECTIONS.keys(deps.storage, None, None, Order::Ascending) {
        let address = k.unwrap();
        let mut collection_tokens = CollectionTokensResponse {
            token_address: address.clone(),
            tokens: vec![],
        };
        let unstaked: cw721::TokensResponse = deps.querier.query_wasm_smart(
            address.clone(),
            &cw721::Cw721QueryMsg::Tokens {
                owner: owner.clone(),
                start_after: None,
                limit: None,
            },
        )?;
        for token_id in unstaked.tokens {
            let UniversalNftInfoResponse { token_uri, .. } = deps.querier.query_wasm_smart(
                address.clone(),
                &cw721::Cw721QueryMsg::NftInfo {
                    token_id: token_id.clone().into(),
                },
            )?;
            collection_tokens.tokens.push(TokenResponse {
                token_id,
                token_uri,
                staking_state: None,
            });
        }
        for index in 0..stakings.len() {
            if stakings[index].token_address == address.clone() {
                let UniversalNftInfoResponse { token_uri, .. } = deps.querier.query_wasm_smart(
                    address.clone(),
                    &cw721::Cw721QueryMsg::NftInfo {
                        token_id: stakings[index].clone().token_id,
                    },
                )?;
                collection_tokens.tokens.push(TokenResponse {
                    token_id: stakings[index].clone().token_id,
                    token_uri: token_uri,
                    staking_state: Some(StakingStateResponse {
                        index: u64::from_str_radix(&index.to_string(), 10).unwrap(),
                        start_timestamp: stakings[index].start_timestamp,
                        end_timestamp: stakings[index].end_timestamp,
                        is_paid: stakings[index].is_paid,
                    }),
                });
            }
        }
        all_tokens.push(collection_tokens.to_owned());
    }
    Ok(to_json_binary(&all_tokens).unwrap())
}
