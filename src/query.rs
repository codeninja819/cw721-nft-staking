use cosmwasm_std::{to_json_binary, Deps, Order, QueryResponse, Response};

use crate::{
    error::ContractError,
    msg::{CollectionResponse, ConfigResponse},
    state::{Staking, COLLECTIONS, CONFIG, STAKINGS},
};

pub fn get_config(deps: Deps) -> Result<QueryResponse, ContractError> {
    let config_state = CONFIG.load(deps.storage)?;
    let owner = config_state.owner;
    Ok(to_json_binary(&ConfigResponse { owner }).unwrap())
}

pub fn get_collections(deps: Deps) -> Result<QueryResponse, ContractError> {
    let mut collections: Vec<CollectionResponse> = vec![];
    for k in COLLECTIONS.keys(deps.storage, None, None, Order::Ascending) {
        let address = k.unwrap();
        let collection = COLLECTIONS.load(deps.storage, address.clone()).unwrap();
        collections.push(CollectionResponse {
            address: address,
            reward: collection.reward,
            cycle: collection.cycle,
            is_whitelisted: collection.is_whitelisted,
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
