use crate::error::ContractError;
use crate::execute::{claim, stake, transfer_ownership, unstake, whitelist};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::{
    get_all_collection_tokens_by_owner, get_collections, get_config, get_stakings_by_owner,
};
use crate::state::{Config, CONFIG};
use cosmwasm_std::{entry_point, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response};
use cw2::set_contract_version;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config_state = Config {
        owner: info.clone().sender.to_string(),
    };
    CONFIG.save(deps.storage, &config_state)?;
    set_contract_version(deps.storage, "Injective CW721 Staking", "0.0.1")?;
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("contract_owner", config_state.owner))
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
        QueryMsg::GetAllCollectionTokensByOwner { owner } => {
            get_all_collection_tokens_by_owner(deps, owner)
        }
        _ => Err(ContractError::Unknown {}),
    }
}
