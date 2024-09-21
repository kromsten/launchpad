use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Timestamp};
use sg2::msg::{CreateMinterMsg, Sg2ExecuteMsg, UpdateMinterParamsMsg};

use crate::state::MetadataMinterParams;

#[cw_serde]
pub struct InstantiateMsg {
    pub params: MetadataMinterParams,
}

#[cw_serde]
pub struct MetadataMinterInitMsgExtension {
    pub merkle_root: String,
    pub merkle_tree_uri: Option<String>,
    pub payment_address: Option<String>,
    pub start_time: Timestamp,
    pub num_tokens: u32,
    pub mint_price: Coin,
    pub per_address_limit: u32,
    pub whitelist: Option<String>,
}

pub type MetadataMinterCreateMsg = CreateMinterMsg<MetadataMinterInitMsgExtension>;

pub type ExecuteMsg = Sg2ExecuteMsg<MetadataMinterInitMsgExtension>;

#[cw_serde]
pub enum SudoMsg {
    UpdateParams(Box<MetadataUpdateParamsMsg>),
}

/// Message for params so they can be updated individually by governance
pub type MetadataUpdateParamsMsg = UpdateMinterParamsMsg<MetadataUpdateParamsExtension>;

#[cw_serde]
pub struct ParamsResponse {
    pub params: MetadataMinterParams,
}

#[cw_serde]
pub struct MetadataUpdateParamsExtension {
    pub max_token_limit: Option<u32>,
    pub max_per_address_limit: Option<u32>,
    pub airdrop_mint_price: Option<Coin>,
    pub airdrop_mint_fee_bps: Option<u64>,
}
