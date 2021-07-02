use cosmwasm_std::{to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdResult, Storage, HumanAddr, WasmQuery};

use crate::msg::{HandleMsg, InitMsg, QueryMsg, ReferenceData, ConfigResponse};
use crate::state::{config, config_read, State};
use crate::msg::QueryMsg::GetReferenceData;

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State {
        ref_addr: msg.ref_addr,
    };
    config(&mut deps.storage).save(&state)?;
    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::SetRef { new_ref } => try_set_ref(deps, new_ref),
    }
}

pub fn try_set_ref<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    new_ref: HumanAddr,
) -> StdResult<HandleResponse> {
    config(&mut deps.storage).update(|mut state| {
        state.ref_addr = new_ref;
        Ok(state)
    })?;

    Ok(HandleResponse::default())
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetRefs {} => to_binary(&query_refs(deps)?),
        QueryMsg::GetReferenceData { base, quote } => to_binary(&query_reference_data(deps, base, quote)),
    }
}

fn query_refs<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<ConfigResponse> {
    let state = config_read(&deps.storage).load()?;
    Ok(state)
}

fn query_reference_data<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    base: String,
    quote: String,
) -> StdResult<ReferenceData> {
    let state = config_read(&deps.storage).load()?;
    let ref_query = WasmQuery::Smart {
        contract_addr: state.ref_addr,
        msg: to_binary(&GetReferenceData{base, quote})?
    };
    Ok(deps.querier.custom_query::<QueryMsg, ReferenceData>(&ref_query.into())?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::from_binary;

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(20, &[]);

        let msg = InitMsg { ref_addr: HumanAddr::from(String::from("first_addr")) };
        let env = mock_env("sender", &[]);

        let res = init(&mut deps, env, msg).unwrap();
        assert_eq!(0, res.messages.len());

        let res = query(&deps, QueryMsg::GetRefs {}).unwrap();
        let value: ConfigResponse = from_binary(&res).unwrap();
        assert_eq!(ConfigResponse{ref_addr: HumanAddr::from(String::from("first_addr"))}, value);
    }

    #[test]
    fn set_new_ref() {
        let mut deps = mock_dependencies(20, &[]);
        let env = mock_env("sender", &[]);

        let msg = InitMsg { ref_addr: HumanAddr::from(String::from("first_addr")) };
        let _res = init(&mut deps, env, msg).unwrap();

        let res = query(&deps, QueryMsg::GetRefs {}).unwrap();
        let value: ConfigResponse = from_binary(&res).unwrap();
        assert_eq!(ConfigResponse{ref_addr: HumanAddr::from(String::from("first_addr"))}, value);

        let msg = HandleMsg::SetRef { new_ref: HumanAddr::from(String::from("second_addr")) };
        let env = mock_env("sender", &[]);
        let _res = handle(&mut deps, env, msg).unwrap();

        let res = query(&deps, QueryMsg::GetRefs {}).unwrap();
        let value: ConfigResponse = from_binary(&res).unwrap();
        assert_eq!(ConfigResponse{ref_addr: HumanAddr::from(String::from("second_addr"))}, value);
    }
}
