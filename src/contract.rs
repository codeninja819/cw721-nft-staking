use crate::error::ContractError;
use crate::execute::{
    change_fee, claim, deposit_collection_reward, stake, transfer_ownership, unstake, whitelist,
    withdraw_fee,
};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::{get_collections, get_config, get_stakings_by_owner};
use crate::state::{Config, CONFIG};
use cosmwasm_std::{coin, entry_point, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response};
use cw2::set_contract_version;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config_state = Config {
        owner: info.clone().sender.to_string(),
        unstake_fee: msg.unstake_fee,
    };
    CONFIG.save(deps.storage, &config_state)?;
    set_contract_version(deps.storage, "Injective CW721 Staking", "0.0.1")?;
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("contract_owner", config_state.owner)
        .add_attribute("unstake_fee", config_state.unstake_fee.to_string()))
}

#[allow(unreachable_patterns)]
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::TransferOwnership { address } => transfer_ownership(deps, env, info, address),
        ExecuteMsg::ChangeFee { fee } => change_fee(deps, env, info, fee),
        ExecuteMsg::WhitelistCollection {
            address,
            reward,
            cycle,
            is_whitelisted,
            spots,
        } => whitelist(
            deps,
            env,
            info,
            address,
            reward,
            cycle,
            is_whitelisted,
            spots,
        ),
        ExecuteMsg::DepositCollectionReward { address } => {
            deposit_collection_reward(deps, env, info, address)
        }
        ExecuteMsg::WithdrawFee { fee } => withdraw_fee(deps, env, info, fee),
        ExecuteMsg::ReceiveNft(msg) => stake(deps, env, info, msg),
        ExecuteMsg::Unstake { index } => unstake(deps, env, info, index),
        ExecuteMsg::ClaimReward { index } => claim(deps, env, info, index),
        _ => Err(ContractError::Unknown {}),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    match msg {
        QueryMsg::GetConfig {} => get_config(deps),
        QueryMsg::GetCollections {} => get_collections(deps, _env),
        QueryMsg::GetStakingsByOwner { owner } => get_stakings_by_owner(deps, owner),
        _ => Err(ContractError::Unknown {}),
    }
}
