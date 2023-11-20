use crate::error::ContractError;
use crate::execute::whitelist;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::get_config;
use crate::state::{Config, CONFIG};
use cosmwasm_std::{
    entry_point, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response,
};
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
        ExecuteMsg::WhitelistCollection {
            address,
            reward,
            cycle,
            is_whitelisted,
        } => whitelist(deps, env, info, address, reward, cycle, is_whitelisted),
        // ExecuteMsg::ReceiveNft(msg) => stake_nft(deps, env, info, config, msg),
        _ => Err(ContractError::Unknown {}),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    match msg {
        QueryMsg::GetConfig {} => get_config(deps),
        _ => Err(ContractError::Unknown {}),
    }
}
