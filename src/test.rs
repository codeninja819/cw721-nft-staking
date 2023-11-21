#[cfg(test)]
mod tests {
    use cosmwasm_std::{coin, to_json_binary, Addr, CosmosMsg, Empty, Timestamp};
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};
    use schemars::JsonSchema;

    use crate::{
        contract::{execute, instantiate, query},
        msg::{CollectionResponse, ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg},
        state::Staking,
    };

    #[test]
    fn test_staking() {
        let mut app = App::default();
        let owner: Addr = Addr::unchecked("owner");
        let cw721_base_code_id = app.store_code(cw721_base_contract());
        let cw721_base_contract_address = app
            .instantiate_contract(
                cw721_base_code_id,
                owner.clone(),
                &cw721_base::InstantiateMsg {
                    name: "CW721 Base".to_owned(),
                    symbol: "CWB".to_owned(),
                    minter: owner.clone().to_string(),
                },
                &[],
                "deploy cw721_base contract",
                None,
            )
            .unwrap();
        let _ = app.execute_contract(
            owner.clone(),
            cw721_base_contract_address.clone(),
            &cw721_base::ExecuteMsg::<Empty, Empty>::Mint {
                token_id: "0".to_owned(),
                owner: owner.clone().to_string(),
                token_uri: Some("token_uri".to_owned()),
                extension: Empty {},
            },
            &vec![],
        );
        let staking_code_id = app.store_code(staking_contract());
        let staking_contract_address = app
            .instantiate_contract(
                staking_code_id,
                owner.clone(),
                &InstantiateMsg {},
                &[],
                "deploy staking contract",
                None,
            )
            .unwrap();
        let resp: ConfigResponse = app
            .wrap()
            .query_wasm_smart(staking_contract_address.clone(), &QueryMsg::GetConfig {})
            .unwrap();
        assert_eq!(resp.owner, "owner");

        app.execute_contract(
            owner.clone(),
            staking_contract_address.clone(),
            &ExecuteMsg::WhitelistCollection {
                address: cw721_base_contract_address.clone().to_string(),
                reward: coin(10, "inj"),
                cycle: 604_800,
                is_whitelisted: true,
            },
            &vec![],
        )
        .unwrap();

        let resp: Vec<CollectionResponse> = app
            .wrap()
            .query_wasm_smart(
                staking_contract_address.clone(),
                &QueryMsg::GetCollections {},
            )
            .unwrap();
        assert_eq!(
            resp,
            vec![CollectionResponse {
                address: cw721_base_contract_address.clone().to_string(),
                reward: coin(10, "inj"),
                cycle: 604_800,
                is_whitelisted: true,
            }]
        );

        app.execute_contract(
            owner.clone(),
            staking_contract_address.clone(),
            &ExecuteMsg::TransferOwnership {
                address: "new_owner".to_owned(),
            },
            &vec![],
        )
        .unwrap();
        let resp: ConfigResponse = app
            .wrap()
            .query_wasm_smart(staking_contract_address.clone(), &QueryMsg::GetConfig {})
            .unwrap();
        assert_eq!(resp.owner, "new_owner");

        let _ = app
            .execute_contract(
                owner.clone(),
                cw721_base_contract_address.clone(),
                &cw721_base::ExecuteMsg::<Empty, Empty>::SendNft {
                    contract: staking_contract_address.clone().to_string(),
                    token_id: "0".to_owned(),
                    msg: to_json_binary(&"").unwrap(),
                },
                &vec![],
            )
            .unwrap();

        let resp: Vec<Staking> = app
            .wrap()
            .query_wasm_smart(
                staking_contract_address.clone(),
                &QueryMsg::GetStakingsByOwner {
                    owner: owner.clone().to_string(),
                },
            )
            .unwrap();
        assert_eq!(
            resp[0].token_address,
            cw721_base_contract_address.clone().to_string()
        );
        assert_eq!(resp[0].token_id, "0".to_owned());
        assert_ne!(resp[0].start_timestamp, Timestamp::from_seconds(0));
        assert_eq!(resp[0].end_timestamp, Timestamp::from_seconds(0));
        assert_eq!(resp[0].is_paid, false);

        let resp: cw721::OwnerOfResponse = app
            .wrap()
            .query_wasm_smart(
                cw721_base_contract_address.clone(),
                &cw721_base::QueryMsg::<Empty>::OwnerOf {
                    token_id: "0".to_owned(),
                    include_expired: Some(true),
                },
            )
            .unwrap();
        assert_eq!(resp.owner, staking_contract_address.clone().to_string());

        let _ = app
            .execute_contract(
                owner.clone(),
                staking_contract_address.clone(),
                &ExecuteMsg::Unstake { index: 0 },
                &vec![],
            )
            .unwrap();

        let resp: Vec<Staking> = app
            .wrap()
            .query_wasm_smart(
                staking_contract_address.clone(),
                &QueryMsg::GetStakingsByOwner {
                    owner: owner.clone().to_string(),
                },
            )
            .unwrap();
        assert_eq!(
            resp[0].token_address,
            cw721_base_contract_address.clone().to_string()
        );
        assert_eq!(resp[0].token_id, "0".to_owned());
        assert_ne!(resp[0].start_timestamp, Timestamp::from_seconds(0));
        assert_ne!(resp[0].end_timestamp, Timestamp::from_seconds(0));
        assert_eq!(resp[0].is_paid, false);

        let resp: cw721::OwnerOfResponse = app
            .wrap()
            .query_wasm_smart(
                cw721_base_contract_address.clone(),
                &cw721_base::QueryMsg::<Empty>::OwnerOf {
                    token_id: "0".to_owned(),
                    include_expired: Some(true),
                },
            )
            .unwrap();
        assert_eq!(resp.owner, owner.clone().to_string());

        let _ = app
            .execute_contract(
                owner.clone(),
                staking_contract_address.clone(),
                &ExecuteMsg::ClaimReward { index: 0 },
                &vec![],
            )
            .unwrap();

        let resp: Vec<Staking> = app
            .wrap()
            .query_wasm_smart(
                staking_contract_address.clone(),
                &QueryMsg::GetStakingsByOwner {
                    owner: owner.clone().to_string(),
                },
            )
            .unwrap();
        assert_eq!(
            resp[0].token_address,
            cw721_base_contract_address.clone().to_string()
        );
        assert_eq!(resp[0].token_id, "0".to_owned());
        assert_ne!(resp[0].start_timestamp, Timestamp::from_seconds(0));
        assert_ne!(resp[0].end_timestamp, Timestamp::from_seconds(0));
        assert_eq!(resp[0].is_paid, true);
    }

    fn staking_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(execute, instantiate, query);
        Box::new(contract)
    }

    fn cw721_base_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            cw721_base::entry::execute,
            cw721_base::entry::instantiate,
            cw721_base::entry::query,
        );
        Box::new(contract)
    }
}
