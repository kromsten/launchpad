use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;
use cw_ownable::{cw_ownable_execute, cw_ownable_query};

use crate::state::{WhitelistConfig, WhitelistData};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
}

#[cw_ownable_execute]
#[cw_serde]
pub enum ExecuteMsg {
    AddWhitelist {
        contract: String,
        config: WhitelistConfig,
    },
    RemoveWhitelist {
        contract: String,
    },
    UpdateWhitelist {
        contract: String,
        config: WhitelistConfig,
    },
    /// Called after minting to process the address
    PostMint {
        whitelist: String,
        address: String,
    },
    Purge {},
}

#[cw_ownable_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Called before minting to check if the address is whitelisted
    #[returns(Coin)]
    PreMint {
        whitelist: String,
        address: String,
        count: u32,
    },
    #[returns(Vec<String>)]
    Wave {},
    #[returns(WhitelistData)]
    WhitelistData { contract: String },
}