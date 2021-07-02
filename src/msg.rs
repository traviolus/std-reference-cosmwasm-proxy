use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{HumanAddr, Uint128};
use crate::state::State;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub ref_addr: HumanAddr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    SetRef { new_ref: HumanAddr },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetRefs {},
    GetReferenceData { base: String, quote: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ReferenceData {
    pub rate: Uint128,
    pub last_updated_base: Uint128,
    pub last_updated_quote: Uint128,
}

pub type ConfigResponse = State;
