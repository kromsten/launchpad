use cosmwasm_std::{Addr, StdError};
pub mod msg;
#[cfg(not(feature = "library"))]
use cosmwasm_std::{to_binary, CosmosMsg, MessageInfo, WasmMsg};

use cw721::Cw721ReceiveMsg;
use sg_std::Response;

pub fn generate_burn_msg(
    info: MessageInfo,
    msg: Cw721ReceiveMsg,
    contract_addr: Addr,
) -> Result<Response, StdError> {
    let res = Response::new();
    let burn_msg = cw721::Cw721ExecuteMsg::Burn {
        token_id: msg.token_id.clone(),
    };
    let cosmos_burn_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: info.sender.to_string(),
        msg: to_binary(&burn_msg)?,
        funds: vec![],
    });
    let res = res.add_attribute("sender", info.sender);
    let res = res.add_attribute("token_id", msg.token_id);
    let res = res.add_attribute("contract_address", contract_addr);
    Ok(res.add_message(cosmos_burn_msg))
}

pub fn check_sender_creator_or_allowed_burn_collection(
    info: MessageInfo,
    creator_addr: Addr,
    allowed_burn_collections: Option<Vec<Addr>>,
) -> Result<bool, StdError> {
    let mut allowed_senders = vec![creator_addr];
    if let Some(mut allowed_burn_collections) = allowed_burn_collections {
        allowed_senders.append(&mut allowed_burn_collections);
    };
    if !allowed_senders.contains(&info.sender) {
        return Err(StdError::GenericErr {
            msg: "Sender is not sg721 creator".to_string(),
        });
    }
    Ok(true)
}

pub fn sender_is_allowed_burn_collection(
    info: MessageInfo,
    allowed_burn_collections: Option<Vec<Addr>>,
) -> bool {
    if let Some(allowed_burn_collections) = allowed_burn_collections {
        return allowed_burn_collections.contains(&info.sender);
    };
    false
}