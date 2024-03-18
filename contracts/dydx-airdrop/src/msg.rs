use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;
use whitelist_immutable_flex::msg::Member;

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: String,
    pub claim_msg_plaintext: String,
    pub airdrop_amount: u128,
    pub members: Vec<Member>,
    pub whitelist_code_id: u64,
    pub minter_address: String,
    pub name_discount_wl_address: String,
    pub name_collection_address: String,
    pub airdrop_count_limit: u32,
}

#[cw_serde]
pub struct AirdropClaimResponse {
    result: bool,
    amount: u32,
    minter_page: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Register {
        eth_address: String,
        eth_sig: String,
    },
    ClaimAirdrop {
        eth_address: String,
        eth_sig: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(bool)]
    AirdropEligible { eth_address: String },
    #[returns(Addr)]
    GetMinter {},
}
