use anyhow::Error;
use cosmwasm_std::{Addr, Timestamp};

use open_edition_factory::state::ParamsExtension;
use sg2::msg::CollectionParams;
use sg_multi_test::StargazeApp;
use vending_factory::msg::VendingMinterInitMsgExtension;

pub struct MinterSetupParams<'a> {
    pub router: &'a mut StargazeApp,
    pub minter_admin: Addr,
    pub num_tokens: u32,
    pub collection_params: CollectionParams,
    pub splits_addr: Option<String>,
    pub start_time: Option<Timestamp>,
    pub minter_code_id: u64,
    pub factory_code_id: u64,
    pub sg721_code_id: u64,
    pub init_msg: Option<VendingMinterInitMsgExtension>,
    pub allowed_burn_collections: Option<Vec<Addr>>,
}
pub struct MinterCollectionResponse {
    pub minter: Option<Addr>,
    pub collection: Option<Addr>,
    pub factory: Option<Addr>,
    pub error: Option<Error>,
}

pub struct MinterInstantiateParams {
    pub num_tokens: u32,
    pub start_time: Option<Timestamp>,
    pub splits_addr: Option<String>,
    pub init_msg: Option<VendingMinterInitMsgExtension>,
    pub allowed_burn_collections: Option<Vec<Addr>>,
}

use cosmwasm_schema::cw_serde;
use open_edition_factory::msg::OpenEditionMinterInitMsgExtension;
use open_edition_factory::types::NftData;

#[cw_serde]
pub struct CodeIds {
    pub minter_code_id: u64,
    pub factory_code_id: u64,
    pub sg721_code_id: u64,
}

pub struct MinterTemplateResponse<T> {
    pub collection_response_vec: Vec<MinterCollectionResponse>,
    pub router: StargazeApp,
    pub accts: T,
}

pub struct MinterTemplateResponseCodeIds<T> {
    pub collection_response_vec: Vec<MinterCollectionResponse>,
    pub router: StargazeApp,
    pub accts: T,
    pub code_ids: CodeIds,
}

pub struct Accounts {
    pub creator: Addr,
    pub buyer: Addr,
}

pub struct OpenEditionMinterSetupParams<'a> {
    pub router: &'a mut StargazeApp,
    pub minter_admin: Addr,
    pub collection_params: CollectionParams,
    pub start_time: Option<Timestamp>,
    pub nft_data: NftData,
    pub per_address_limit: u32,
    pub end_time: Option<Timestamp>,
    pub minter_code_id: u64,
    pub factory_code_id: u64,
    pub sg721_code_id: u64,
    pub init_msg: Option<OpenEditionMinterInitMsgExtension>,
    pub allowed_burn_collections: Option<Vec<Addr>>,
}

pub struct OpenEditionMinterInstantiateParams {
    pub start_time: Option<Timestamp>,
    pub end_time: Option<Timestamp>,
    pub per_address_limit: Option<u32>,
    pub nft_data: Option<NftData>,
    pub init_msg: Option<OpenEditionMinterInitMsgExtension>,
    pub params_extension: Option<ParamsExtension>,
    pub allowed_burn_collections: Option<Vec<Addr>>,
}
