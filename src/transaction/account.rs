use std::ffi::c_void;
use std::str::FromStr;

use light_sdk::namada_core::proto::Tx as NamadaTx;
use light_sdk::namada_core::types::key::common;
use light_sdk::transaction::account;

use crate::transaction::GlobalArgs;
use crate::types::{Hashes, Tx};
use crate::utils::{allocate, CResult, CString, FormatErr};

#[repr(C)]
pub struct RevealPk(*mut c_void);

#[no_mangle]
pub extern "C" fn new_reveal_pk(public_key: CString, args: GlobalArgs) -> CResult {
    let pk = public_key.to_string();
    let pk = match common::PublicKey::from_str(&pk) {
        Ok(pk) => pk,
        e => return e.into(),
    };

    let args = match args.try_into().wrap_err("Parse global args failed") {
        Ok(args) => args,
        e => return e.into(),
    };
    let tx = account::RevealPk::new(pk, args);
    CResult {
        is_err: false,
        error_msg: "".to_string().into(),
        value: allocate(RevealPk(allocate(tx))),
    }
}

#[no_mangle]
pub extern "C" fn get_sign_bytes_reveal_pk(reveal_pk_tx: &RevealPk) -> Hashes {
    let tx = reveal_pk_tx.0 as *mut account::RevealPk;
    let casted_tx = unsafe {
        tx.as_ref()
            .expect("Expected initialized RevealPk, found null pointer")
    };

    let hash = casted_tx
        .get_sign_bytes()
        .into_iter()
        .map(|hash| hash.0)
        .collect::<Vec<[u8; 32]>>()
        .into_boxed_slice();
    let len = hash.len();
    let ptr = Box::into_raw(hash) as *mut [u8; 32];

    Hashes { ptr, len }
}

#[no_mangle]
pub extern "C" fn attach_raw_signatures_reveal_pk(
    reveal_pk_tx: RevealPk,
    public_key: CString,
    signature: CString,
) -> CResult {
    let tx = reveal_pk_tx.0 as *mut account::RevealPk;
    let casted_tx = unsafe { tx.read() };
    let signer = match common::PublicKey::from_str(&public_key.to_string()) {
        Ok(pk) => pk,
        e => return e.into(),
    };
    let signature = match common::Signature::from_str(&signature.to_string()) {
        Ok(sig) => sig,
        e => return e.into(),
    };

    let signed_tx = casted_tx.attach_signatures(signer, signature);

    CResult {
        is_err: false,
        error_msg: "".to_string().into(),
        value: allocate(RevealPk(allocate(signed_tx))),
    }
}
