use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Timestamp};
use cw_storage_plus::{Item, Map};
use sg4::{MinterConfig, Status};

#[cw_serde]
pub struct ConfigExtension {
    pub admin: Addr,
    pub payment_address: Option<Addr>,
    pub num_tokens: u32,
    pub whitelist: Option<Addr>,
    pub start_time: Timestamp,
    pub per_address_limit: u32,
    pub discount_price: Option<Coin>,
    pub merkle_root: String,
}

pub type Config = MinterConfig<ConfigExtension>;


pub const CONFIG: Item<Config> = Item::new("config");
pub const SG721_ADDRESS: Item<Addr> = Item::new("sg721_address");
// map of index position and token id

pub const MINTED_IDS : Map<&str, bool> = Map::new("minted_ids");
pub const MINTABLE_NUM_TOKENS: Item<u32> = Item::new("mintable_num_tokens");

pub const MINTER_ADDRS: Map<&Addr, u32> = Map::new("ma");

/// Holds the status of the minter. Can be changed with on-chain governance proposals.
pub const STATUS: Item<Status> = Item::new("status");

pub const MERKLE_ROOT: Item<String> = Item::new("merkle_root");
pub const MERKLE_TREE_URI: Item<String> = Item::new("merkle_tree_uri");