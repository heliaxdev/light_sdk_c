use std::str::FromStr;

use namada_light_sdk::namada_core::types::address::Address;
use namada_light_sdk::reading;
use crate::types::DenominatedAmount;

use crate::utils::{allocate, CResult, CString};

#[no_mangle]
pub extern "C" fn query_native_token(tendermint_addr: CString) -> CResult {
    let tendermint_addr = tendermint_addr.to_string();
    match reading::query_native_token(&tendermint_addr) {
        Ok(address) => CResult {
            is_err: false,
            error_msg: "".to_string().into(),
            value: allocate(CString::from(address.to_string())),
        },
        e => e.into(),
    }
}

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

#[no_mangle]
pub extern "C" fn denominate_amount(tendermint_addr: CString, amount: u64, token: CString) -> CResult {
    match reading::denominate_amount(&tendermint_addr.to_string(), amount, &token.to_string()) {
        Ok(denominated) => CResult {
            is_err: false,
            error_msg: "".to_string().into(),
            value: allocate(DenominatedAmount{inner: allocate(denominated)}),
        },
        e => e.into(),
    }
}
