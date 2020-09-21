use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdResult, Storage,
};

use crate::msg::{FileAddressResponse, HandleMsg, InitMsg, QueryMsg};
use crate::state::{config, config_read, State};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let mut initial_registry: Vec<String> = Vec::new();
    initial_registry.push(msg.ipfs_address);
    let state = State {
        owner: deps.api.canonical_address(&env.message.sender)?,
        address_registry: initial_registry,
    };
    config(&mut deps.storage).save(&state)?;
    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::AddFileAddress { ipfs_address } => try_add_file(deps, env, ipfs_address),
    }
}

pub fn try_add_file<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    ipfs_address: String,
) -> StdResult<HandleResponse> {
    config(&mut deps.storage).update(|mut state| {
        state.address_registry.push(ipfs_address);
        Ok(state)
    })?;

    Ok(HandleResponse::default())
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::ListAll {} => to_binary(&query_file(deps)?),
    }
}

fn query_file<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
) -> StdResult<FileAddressResponse> {
    let state = config_read(&deps.storage).load()?;
    Ok(FileAddressResponse {
        address_registry: state.address_registry,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        // Set up
        let mut deps = mock_dependencies(20, &[]);
        let env = mock_env("creator", &coins(1000, "earth"));
        let mock_address = "ipfs_address".to_string();
        let msg = InitMsg {
            ipfs_address: mock_address.clone(),
        };

        // Assert that init successful
        let res = init(&mut deps, env.clone(), msg).unwrap();
        assert_eq!(0, res.messages.len());

        // Assert that the mock address has been added to the registry
        let res = query(&deps, QueryMsg::ListAll {}).unwrap();
        let value: FileAddressResponse = from_binary(&res).unwrap();
        assert_eq!(mock_address, value.address_registry[0]);
    }

    #[test]
    fn add_file() {
        // Setup
        let mut deps = mock_dependencies(20, &coins(2, "token"));
        let mock_address = "ipfs_address".to_string();
        let msg = InitMsg {
            ipfs_address: mock_address.clone(),
        };
        let env = mock_env("creator", &coins(2, "token"));
        // Initialize
        let _res = init(&mut deps, env, msg).unwrap();

        // Mock message
        let message = HandleMsg::AddFileAddress {
            ipfs_address: mock_address.clone(),
        };

        // Add the address
        let env = mock_env("anyone", &coins(2, "token"));
        let _res = handle(&mut deps, env, message);

        // Assert the address has been added correctly
        let res = query(&deps, QueryMsg::ListAll {}).unwrap();
        let value: FileAddressResponse = from_binary(&res).unwrap();
        assert_eq!(2, value.address_registry.len());
        assert_eq!("ipfs_address".to_string(), value.address_registry[1]);
    }

    #[test]
    fn list_all() {
        // Setup
        let mut deps = mock_dependencies(20, &[]);
        let env = mock_env("creator", &coins(1000, "earth"));
        let mock_address = "ipfs_address".to_string();
        let msg = InitMsg {
            ipfs_address: mock_address.clone(),
        };
        let _res = init(&mut deps, env, msg).unwrap();

        // Mock message
        let message = HandleMsg::AddFileAddress {
            ipfs_address: mock_address,
        };

        // Add addresses
        let env = mock_env("anyone", &coins(2, "token"));
        let _res = handle(&mut deps, env, message.clone());

        let env = mock_env("anyone", &coins(2, "token"));
        let _res = handle(&mut deps, env, message.clone());

        // Query for all addresses
        let res = query(&deps, QueryMsg::ListAll {}).unwrap();
        let value: FileAddressResponse = from_binary(&res).unwrap();
        assert_eq!(3, value.address_registry.len());
    }
}
