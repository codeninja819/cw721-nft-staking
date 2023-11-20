use cosmwasm_std::{to_json_binary, Deps, QueryResponse};

use crate::{error::ContractError, msg::ConfigResponse, state::CONFIG};

// query configuration.
pub fn get_config(deps: Deps) -> Result<QueryResponse, ContractError> {
    let config_state = CONFIG.load(deps.storage)?;
    let owner = config_state.owner;
    Ok(to_json_binary(&ConfigResponse { owner }).unwrap())
}
