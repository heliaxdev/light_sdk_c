use std::str::FromStr;

use namada_light_sdk::namada_core::types::address::Address;
use namada_light_sdk::reading;

use crate::utils::{allocate, CResult, CString};

#[no_mangle]
pub extern "C" fn is_public_key_revealed(tendermint_addr: CString, owner: CString) -> CResult {
    let owner = match Address::from_str(&owner.to_string()) {
        Ok(address) => address,
        e => return e.into(),
    };
    let tendermint_addr = tendermint_addr.to_string();

    match reading::account::is_public_key_revealed(&tendermint_addr, &owner) {
        Ok(is_revealed) => CResult {
            is_err: false,
            error_msg: "".to_string().into(),
            value: allocate(is_revealed),
        },
        e => e.into(),
    }
}
