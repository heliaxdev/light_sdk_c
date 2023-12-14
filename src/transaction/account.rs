use std::str::FromStr;

use light_sdk::namada_core::types::key::common;
use light_sdk::transaction::account;

use crate::transaction::GlobalArgs;
use crate::types::Tx;
use crate::utils::{CResult, CString, FormatErr};

#[repr(C)]
pub struct RevealPk(Tx);

#[no_mangle]
pub extern "C" fn new_reveal_pk(
    public_key: CString,
    args: GlobalArgs,
) -> CResult<RevealPk> {
    let pk = public_key.to_string();
    let pk = match common::PublicKey::from_str(&pk).format_err(){
        Ok(pk) => pk,
        Err(e) => return CResult::Err(e)
    };

    let args = match args.try_into().format_err() {
        Ok(args) => args,
        Err(e) => return CResult::Err(e)
    };
    let account::RevealPk(tx) = account::RevealPk::new(pk, args);
    CResult::Ok(RevealPk(Tx::from(tx)))
}
