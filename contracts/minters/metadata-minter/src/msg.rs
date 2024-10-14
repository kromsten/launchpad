use sg_metadata::Metadata;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, StdResult, Storage, Timestamp};
use metadata_factory::{msg::MetadataMinterCreateMsg, state::MetadataMinterParams};

use crate::helpers;

#[cw_serde]
pub struct InstantiateMsg {
    pub create_msg: MetadataMinterCreateMsg,
    pub params: MetadataMinterParams,
}



#[cw_serde]
pub struct MintWhitelist {
    pub allowance:    Option<u32>,
    pub proof_hashes: Option<Vec<String>>,
}



#[cw_serde]
pub enum ExecuteMsg {
    Mint {
        mint_data  :  MintData,
        whitelist  :  Option<MintWhitelist>,
    },
    SetWhitelist {
        whitelist: String,
    },
    Purge {},
    UpdateMintPrice {
        price: u128,
    },
    UpdateStartTime(Timestamp),
    /// Runs custom checks against TradingStartTime on VendingMinter, then updates by calling sg721-base
    UpdateStartTradingTime(Option<Timestamp>),
    UpdatePerAddressLimit {
        per_address_limit: u32,
    },
    MintTo {
        mint_data  :  MintData,
        recipient  :  String,
    },
    MintFor {
        mint_data  :  MintData,
        recipient  :  String,
    },
    BurnRemaining {},
    UpdateDiscountPrice {
        price: u128,
    },
    RemoveDiscountPrice {},
}

#[cw_serde]
pub enum QueryMsg {
    Config {},
    MintableNumTokens {},
    StartTime {},
    MintPrice {},
    MintCount { address: String },
    Status {},
}


#[cw_serde]
pub struct ConfigResponse {
    pub admin: String,
    pub num_tokens: u32,
    pub per_address_limit: u32,
    pub sg721_address: String,
    pub sg721_code_id: u64,
    pub start_time: Timestamp,
    pub mint_price: Coin,
    pub whitelist: Option<String>,
    pub factory: String,
    pub discount_price: Option<Coin>,

    pub merkle_root: String,
    pub merkle_tree_uri : String,
}

#[cw_serde]
pub struct MintableNumTokensResponse {
    pub count: u32,
}

#[cw_serde]
pub struct StartTimeResponse {
    pub start_time: String,
}

#[cw_serde]
pub struct MintPriceResponse {
    pub public_price: Coin,
    pub airdrop_price: Coin,
    pub whitelist_price: Option<Coin>,
    pub current_price: Coin,
    pub discount_price: Option<Coin>,
}

#[cw_serde]
pub struct MintCountResponse {
    pub address: String,
    pub count: u32,
}



#[cw_serde]
pub struct MintData {
    
    pub metadata: Metadata,

    pub token_id: String,

    #[serde(skip_serializing)]
    pub proof_hashes: Vec<String>,
}



impl MintData {
    pub fn check_inclusion(&self, storage: &dyn Storage) -> StdResult<()> {
        helpers::check_mint_data_inclusion(storage, self)
    }  
}
