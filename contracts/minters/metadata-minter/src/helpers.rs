use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    to_json_binary, Addr, Coin, ContractInfoResponse, CustomQuery, Querier, QuerierWrapper, StdError, StdResult, Storage, WasmMsg, WasmQuery
};
use sg_std::CosmosMsg;

use crate::{msg::{ConfigResponse, ExecuteMsg, MintData, QueryMsg}, state::MERKLE_ROOT};
use rs_merkle::{algorithms::Sha256, Hasher};


/// MinterContract is a wrapper around Addr that provides a lot of helpers
/// for working with this.
#[cw_serde]
pub struct MinterContract(pub Addr);

impl MinterContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
        let msg = to_json_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }

    pub fn call_with_funds<T: Into<ExecuteMsg>>(
        &self,
        msg: T,
        funds: Coin,
    ) -> StdResult<CosmosMsg> {
        let msg = to_json_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![funds],
        }
        .into())
    }

    pub fn contract_info<Q, T, CQ>(&self, querier: &Q) -> StdResult<ContractInfoResponse>
    where
        Q: Querier,
        T: Into<String>,
        CQ: CustomQuery,
    {
        let query = WasmQuery::ContractInfo {
            contract_addr: self.addr().into(),
        }
        .into();
        let res: ContractInfoResponse = QuerierWrapper::<CQ>::new(querier).query(&query)?;
        Ok(res)
    }

    pub fn config(&self, querier: &QuerierWrapper) -> StdResult<ConfigResponse> {
        let res: ConfigResponse = querier.query_wasm_smart(self.addr(), &QueryMsg::Config {})?;
        Ok(res)
    }
}


pub fn check_mint_data_inclusion(
    storage: &dyn Storage,
    mint_data: &MintData,
) -> StdResult<()> {

    let merkle_root = MERKLE_ROOT.load(storage)?;
    
    let init_data_hash = Sha256::hash(&to_json_binary(&mint_data)?);

    let final_hash = mint_data.proof_hashes.iter().try_fold(
        init_data_hash,
        |accum_hash_slice, new_proof_hashstring| {
            let mut hashe_slices = [
                accum_hash_slice,
                string_to_byte_slice(&new_proof_hashstring)?,
            ];
            hashe_slices.sort_unstable();
            Result::<[u8; 32], StdError>::Ok(Sha256::hash(&hashe_slices.concat()))
        },
    );

    if final_hash.is_err() || merkle_root != hex::encode(final_hash.unwrap()) {
        return Err(cosmwasm_std::StdError::GenericErr {
            msg: "Invalid Merkle Proof".to_string(),
        });
    }

    Ok(())
}


pub fn string_to_byte_slice(string: &String) -> StdResult<[u8; 32]> {
    let mut byte_slice = [0; 32];
    hex::decode_to_slice(string, &mut byte_slice).map_err(|_| StdError::GenericErr {
        msg: "Couldn't decode hash string".to_string(),
    })?;
    Ok(byte_slice)
}
