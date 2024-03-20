use crate::{msg::QueryMsg, state::CONFIG, ContractError};
use cosmwasm_std::{entry_point, to_json_binary, Binary};
use cosmwasm_std::{Addr, Env};
use cosmwasm_std::{Deps, DepsMut, StdResult};
use vending_minter::helpers::MinterContract;
use whitelist_immutable_flex::helpers::WhitelistImmutableFlexContract;
use whitelist_immutable_flex::msg::MemberResponse;
use crate::state::{AIRDROP_COUNT, HAS_CLAIMED, IS_ADDRESS_REGISTERED};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::AirdropEligible { eth_address } => {
            to_json_binary(&query_airdrop_is_eligible(deps, eth_address)?)
        }
        QueryMsg::GetMinter {} => to_json_binary(&query_minter(deps)?),
        QueryMsg::IsRegistered { eth_address } => to_json_binary(&query_airdrop_is_registered(deps, eth_address)?),
        QueryMsg::HasClaimed { eth_address } => to_json_binary(&query_airdrop_has_claimed(deps, eth_address)?),
        QueryMsg::GetAirdropCount {} => to_json_binary(&query_airdrop_count(deps)?),
    }
}

fn query_minter(deps: Deps) -> StdResult<Addr> {
    let config = CONFIG.load(deps.storage)?;
    Ok(config.minter_address)
}

pub fn query_airdrop_is_eligible(deps: Deps, eth_address: String) -> StdResult<bool> {
    let config = CONFIG.load(deps.storage)?;
    match config.whitelist_address {
        Some(address) => WhitelistImmutableFlexContract(deps.api.addr_validate(&address)?)
            .includes(&deps.querier, eth_address),
        None => Err(cosmwasm_std::StdError::NotFound {
            kind: "Whitelist Contract".to_string(),
        }),
    }
}

pub fn query_airdrop_count(deps: Deps) -> StdResult<u32> {
    let airdrop_count = AIRDROP_COUNT.load(deps.storage)?;
    Ok(airdrop_count)
}

pub fn query_airdrop_is_registered(deps: Deps, eth_address: String) -> StdResult<bool> {
   let is_registered = IS_ADDRESS_REGISTERED.may_load(deps.storage, &eth_address)?;
    Ok(is_registered.unwrap_or_default())
}

pub fn query_airdrop_has_claimed(deps: Deps, eth_address: String) -> StdResult<bool> {
    let has_claimed = HAS_CLAIMED.may_load(deps.storage, &eth_address)?;
    Ok(has_claimed.unwrap_or_default())
}


pub fn query_collection_whitelist(deps: &DepsMut) -> Result<String, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let minter_addr = config.minter_address;
    let config = MinterContract(minter_addr).config(&deps.querier)?;
    match config.whitelist {
        Some(whitelist) => Ok(whitelist),
        None => Err(ContractError::CollectionWhitelistMinterNotSet {}),
    }
}

pub fn query_collection_address(deps: &DepsMut) -> StdResult<String> {
    let config = CONFIG.load(deps.storage)?;
    let minter_addr = config.minter_address;
    let config = MinterContract(minter_addr).config(&deps.querier)?;
    Ok(config.sg721_address)
}

pub fn query_mint_count(deps: &DepsMut, eth_address: String) -> StdResult<u32> {
    let config = CONFIG.load(deps.storage)?;
    let whitelist_address =
        config
            .whitelist_address
            .ok_or_else(|| cosmwasm_std::StdError::NotFound {
                kind: "Whitelist Contract".to_string(),
            })?;
    let member_response: MemberResponse = deps.querier.query(&cosmwasm_std::QueryRequest::Wasm(
        cosmwasm_std::WasmQuery::Smart {
            contract_addr: whitelist_address,
            msg: to_json_binary(&whitelist_immutable_flex::msg::QueryMsg::Member {
                address: eth_address,
            })?,
        },
    ))?;
    Ok(member_response.member.mint_count)
}
