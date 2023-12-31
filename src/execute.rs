use std::str::FromStr;
use std::sync::Arc;
use std::vec;

use crate::error::ContractError;

use crate::msg::UniversalNftReceiveMsg;
use crate::state::{Collection, Staking, COLLECTIONS, CONFIG, STAKINGS};
use cosmwasm_std::{
    coin, to_json_binary, BankMsg, Coin, CosmosMsg, DepsMut, Env, Event, MessageInfo, Response,
    StdResult, Storage, Timestamp, Uint128, WasmMsg,
};
use cw721::{Cw721ExecuteMsg, Cw721ReceiveMsg};

pub fn transfer_ownership(
    mut deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    new_owner: String,
) -> Result<Response, ContractError> {
    let store = deps.branch().storage;
    check_contract_owner_only(info.clone(), store)?;
    let mut config_state = CONFIG.load(store).unwrap();
    let old_owner = config_state.owner;
    config_state.owner = new_owner.clone();
    CONFIG.save(deps.storage, &config_state)?;
    Ok(Response::new().add_event(
        Event::new("ownership_transferred")
            .add_attribute("old_owner", old_owner)
            .add_attribute("new_owner", new_owner),
    ))
}

pub fn change_fee(
    mut deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    new_fee: Coin,
) -> Result<Response, ContractError> {
    let store = deps.branch().storage;
    check_contract_owner_only(info.clone(), store)?;
    let mut config_state = CONFIG.load(store).unwrap();
    let old_fee = config_state.unstake_fee;
    config_state.unstake_fee = new_fee.clone();
    CONFIG.save(deps.storage, &config_state)?;
    Ok(Response::new().add_event(
        Event::new("fee_changed")
            .add_attribute("old_fee", old_fee.to_string())
            .add_attribute("new_fee", new_fee.to_string()),
    ))
}

pub fn whitelist(
    mut deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    address: String,
    reward: Coin,
    cycle: u64,
    is_whitelisted: bool,
    spots: u64,
) -> Result<Response, ContractError> {
    let store = deps.branch().storage;
    check_contract_owner_only(info.clone(), store)?;
    let collection = COLLECTIONS.may_load(store, address.clone())?;
    if collection.is_none() {
        let new_collection = Collection::new(reward.clone(), cycle.clone(), true, spots, 0);
        COLLECTIONS.save(store, address.clone(), &new_collection)?;
    } else {
        COLLECTIONS.update(store, address.clone(), |c| -> StdResult<Collection> {
            Ok(Collection::new(
                reward.clone(),
                cycle,
                is_whitelisted,
                spots,
                c.clone().unwrap().pool_amount,
            ))
        })?;
    }
    Ok(Response::new().add_event(
        Event::new("collection_whitelisted")
            .add_attribute("address", address)
            .add_attribute("reward", reward.to_string())
            .add_attribute("cycle", cycle.to_string())
            .add_attribute("is_whitelisted", is_whitelisted.to_string())
            .add_attribute("spots", spots.to_string()),
    ))
}

pub fn deposit_collection_reward(
    mut deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    address: String,
) -> Result<Response, ContractError> {
    let store = deps.branch().storage;
    let collection = COLLECTIONS.may_load(store, address.clone())?;
    let index = info.funds.iter().position(|r| r.denom == "inj").unwrap();
    let amount: u128 = info.funds[index].amount.into();
    if collection.is_none() {
        return Err(ContractError::Unknown {});
    } else {
        COLLECTIONS.update(store, address.clone(), |c| -> StdResult<Collection> {
            let col = c.clone().unwrap();
            Ok(Collection::new(
                col.reward,
                col.cycle,
                col.is_whitelisted,
                col.spots,
                col.pool_amount + amount,
            ))
        })?;
    }
    Ok(Response::new().add_event(
        Event::new("collection_reward_deposited")
            .add_attribute("address", address)
            .add_attribute("amount", amount.to_string()),
    ))
}
pub fn withdraw_fee(
    mut deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    fee: Coin,
) -> Result<Response, ContractError> {
    let store = deps.branch().storage;
    check_contract_owner_only(info.clone(), store)?;
    let mut config_state = CONFIG.load(store).unwrap();
    let fee_amount = u128::from_str(&fee.clone().amount.to_string()).unwrap();
    if fee_amount > config_state.fee_collected.clone() {
        return Err(ContractError::NotEnoughFeeCollected {});
    }
    config_state.fee_collected = config_state.fee_collected.clone() - fee_amount;
    CONFIG.save(deps.storage, &config_state)?;

    let transfer_msg = BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![fee.clone()],
    };
    Ok(Response::new()
        .add_event(
            Event::new("fee_withdrawn")
                .add_attribute("address", info.sender.to_string())
                .add_attribute("fee", fee.to_string()),
        )
        .add_message(transfer_msg))
}

pub fn stake(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: UniversalNftReceiveMsg,
) -> Result<Response, ContractError> {
    let token_address = info.sender.to_string();
    let owner = msg.clone().sender;
    let store = deps.branch().storage;
    let collection = COLLECTIONS.may_load(store, token_address.clone())?;
    if collection.is_none() {
        return Err(ContractError::NotWhitelisted {});
    }
    let stakings = STAKINGS.may_load(store, owner.clone())?;
    let mut stakings_state: Vec<Staking>;
    if stakings.is_none() {
        stakings_state = vec![]
    } else {
        stakings_state = stakings.unwrap()
    };
    stakings_state.push(Staking::new(
        token_address.clone(),
        msg.clone().token_id,
        env.block.time,
        false,
    ));
    STAKINGS.save(store, owner.clone(), &stakings_state)?;
    Ok(Response::new().add_event(
        Event::new("staked")
            .add_attribute("token_address", token_address)
            .add_attribute("token_id", msg.token_id)
            .add_attribute("owner", owner)
            .add_attribute("start_timestamp", env.block.time.to_string())
            .add_attribute("index", (stakings_state.len() - 1).to_string()),
    ))
}

pub fn unstake(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    index: u64,
) -> Result<Response, ContractError> {
    let owner = info.sender.to_string();
    let store = deps.branch().storage;
    let mut config_state = CONFIG.load(store)?;
    let mut stakings_state = STAKINGS.may_load(store, owner.clone())?.unwrap();
    let staking_info = stakings_state[index as usize].clone();
    let mut staking = &mut stakings_state[index as usize];
    let collection = COLLECTIONS.load(store, staking_info.token_address.clone())?;
    if staking.end_timestamp != Timestamp::from_nanos(0) {
        return Err(ContractError::AlreadyUnstaked {});
    }
    if env.block.time.seconds() - staking_info.start_timestamp.clone().seconds() < collection.cycle
    {
        if info.funds == vec![config_state.unstake_fee.clone()] {
            config_state.fee_collected = config_state.fee_collected
                + u128::from_str(&config_state.unstake_fee.clone().amount.to_string()).unwrap();
            CONFIG.save(store, &config_state)?;
        } else {
            return Err(ContractError::NotEnoughUnstakeFee {});
        }
    }
    staking.end_timestamp = env.block.time;
    let transfer_msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: staking.token_address.clone(),
        msg: to_json_binary(&Cw721ExecuteMsg::TransferNft {
            recipient: owner.clone(),
            token_id: staking.token_id.clone(),
        })?,
        funds: vec![],
    });
    let _ = STAKINGS.save(store, owner.clone(), &stakings_state);
    Ok(Response::new()
        .add_event(
            Event::new("unstaked")
                .add_attribute("token_address", staking_info.token_address.clone())
                .add_attribute("token_id", staking_info.token_id.clone())
                .add_attribute("owner", owner)
                .add_attribute(
                    "start_timestamp",
                    staking_info.start_timestamp.seconds().to_string(),
                )
                .add_attribute("end_timestamp", env.block.time.seconds().to_string())
                .add_attribute("index", index.to_string()),
        )
        .add_message(transfer_msg))
}

pub fn claim(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    index: u64,
) -> Result<Response, ContractError> {
    let owner = info.sender.to_string();
    let store = deps.branch().storage;
    let mut stakings_state = STAKINGS.may_load(store, owner.clone())?.unwrap();
    let staking_info = stakings_state[index as usize].clone();
    let collection = COLLECTIONS
        .may_load(store, staking_info.token_address.clone())?
        .unwrap();
    let mut staking = &mut stakings_state[index as usize];
    if staking.end_timestamp == Timestamp::from_nanos(0) {
        return Err(ContractError::NotUnstaked {});
    }
    if staking.is_paid == true {
        return Err(ContractError::RewardAlreadyClaimed {});
    }
    staking.is_paid = true;
    let mut reward_amount = &u128::from(collection.reward.amount)
        * &u128::from(
            staking_info.end_timestamp.seconds() - staking_info.start_timestamp.seconds(),
        )
        / &u128::from(collection.cycle);
    if reward_amount > u128::from(collection.reward.amount) {
        reward_amount = u128::from(collection.reward.amount);
    }
    let _ = STAKINGS.save(store, owner.clone(), &stakings_state);
    if reward_amount > 0 {
        if collection.pool_amount < reward_amount {
            return Err(ContractError::NotEnoughRewardPool {});
        }
        let transfer_msg = BankMsg::Send {
            to_address: owner.clone(),
            amount: vec![coin(reward_amount, collection.clone().reward.denom)],
        };
        COLLECTIONS.update(
            store,
            staking_info.token_address.clone(),
            |c| -> StdResult<Collection> {
                let col = c.clone().unwrap();
                Ok(Collection::new(
                    col.reward,
                    col.cycle,
                    col.is_whitelisted,
                    col.spots,
                    col.pool_amount - reward_amount,
                ))
            },
        )?;
        Ok(Response::new()
            .add_event(
                Event::new("claimed")
                    .add_attribute("token_address", staking_info.token_address.clone())
                    .add_attribute("token_id", staking_info.token_id.clone())
                    .add_attribute("owner", owner)
                    .add_attribute(
                        "start_timestamp",
                        staking_info.start_timestamp.seconds().to_string(),
                    )
                    .add_attribute(
                        "end_timestamp",
                        staking_info.end_timestamp.seconds().to_string(),
                    )
                    .add_attribute(
                        "reward",
                        coin(reward_amount, collection.reward.denom).to_string(),
                    )
                    .add_attribute("index", index.to_string()),
            )
            .add_message(transfer_msg))
    } else {
        Ok(Response::new().add_event(
            Event::new("unstaked")
                .add_attribute("token_address", staking_info.token_address.clone())
                .add_attribute("token_id", staking_info.token_id.clone())
                .add_attribute("owner", owner)
                .add_attribute(
                    "start_timestamp",
                    staking_info.start_timestamp.seconds().to_string(),
                )
                .add_attribute(
                    "end_timestamp",
                    staking_info.end_timestamp.seconds().to_string(),
                )
                .add_attribute(
                    "reward",
                    coin(reward_amount, collection.reward.denom).to_string(),
                )
                .add_attribute("index", index.to_string()),
        ))
    }
}

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
