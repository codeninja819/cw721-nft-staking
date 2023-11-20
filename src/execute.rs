use crate::error::ContractError;

use crate::state::{Collection, COLLECTIONS, CONFIG};
use cosmwasm_std::{
    Coin, DepsMut, Env, Event, MessageInfo,
    Response, StdResult, Storage,
};


pub fn whitelist(
    mut deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    address: String,
    reward: Coin,
    cycle: u64,
    is_whitelisted: bool,
) -> Result<Response, ContractError> {
    let store = deps.branch().storage;
    check_contract_owner_only(info.clone(), store)?;
    let collection = COLLECTIONS.may_load(store, address.clone())?;
    if collection.is_none() {
        let new_collection = Collection::new(reward.clone(), cycle.clone(), true);
        COLLECTIONS.save(store, address.clone(), &new_collection)?;
    } else {
        COLLECTIONS.update(store, address.clone(), |c| -> StdResult<Collection> {
            Ok(Collection::new(
                c.clone().unwrap().reward,
                c.clone().unwrap().cycle,
                is_whitelisted.clone(),
            ))
        })?;
    }
    Ok(Response::new().add_event(
        Event::new("collection_whitelisted")
            .add_attribute("address", address)
            .add_attribute("reward", reward.to_string())
            .add_attribute("cycle", cycle.to_string())
            .add_attribute("is_whitelisted", is_whitelisted.to_string()),
    ))
}

// pub fn deposit(
//     token_id: String,
//     sender: String,
//     deps: DepsMut,
//     env: Env,
// ) -> Result<Response, ContractError> {
//     let resp = Response::new();

//     let name = encode(format!("{} - {}", sender, token_id));
//     let owner: Vec<String> = vec![token_id.clone()];
//     let owner_obj: Item<Vec<String>> = Item::new(&sender);

//     owner_obj
//         .save(deps.storage, &owner)
//         .expect("Error saving owner to storage");

//     let staked: Item<Token> = Item::new(&name);

//     let t = Token {
//         id: token_id,
//         deposit_time: env.block.time.seconds(),
//         owner: sender,
//     };

//     staked
//         .save(deps.storage, &t)
//         .expect("Error saving stake to storage");

//     Ok(resp)
// }

// check message sender is contract owner.
pub fn check_contract_owner_only(
    info: MessageInfo,
    store: &dyn Storage,
) -> Result<bool, ContractError> {
    let config_state = CONFIG.load(store)?;
    if config_state.owner != info.sender.to_string() {
        return Err(ContractError::Unauthorized {});
    }
    Ok(true)
}
