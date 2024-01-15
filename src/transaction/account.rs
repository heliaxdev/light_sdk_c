use std::ffi::c_void;
use std::str::FromStr;

use namada_light_sdk::namada_core::types::address;
use namada_light_sdk::namada_core::types::key::common;
use namada_light_sdk::transaction::account;

use crate::transaction::GlobalArgs;
use crate::types::{DenominatedAmount, Hashes, Tx};
use crate::utils::{allocate, CResult, CString, FormatErr};

#[repr(C)]
pub struct RevealPk(*mut c_void);

impl RevealPk {
    fn to_native(self) -> account::RevealPk {
        let tx = self.0 as *mut account::RevealPk;
        unsafe { tx.read() }
    }

    fn to_native_ref(&self) -> &account::RevealPk {
        let tx = self.0 as *mut account::RevealPk;
        unsafe {
            tx.as_ref()
                .expect("Expected initialized RevealPk, found null pointer")
        }
    }
}

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
    let tx = reveal_pk_tx.to_native_ref();
    let hash = tx
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
    let tx = reveal_pk_tx.to_native();
    let signer = match common::PublicKey::from_str(&public_key.to_string()) {
        Ok(pk) => pk,
        e => return e.into(),
    };
    let signature = match common::Signature::from_str(&signature.to_string()) {
        Ok(sig) => sig,
        e => return e.into(),
    };

    let signed_tx = tx.attach_signatures(signer, signature);

    CResult {
        is_err: false,
        error_msg: "".to_string().into(),
        value: allocate(RevealPk(allocate(signed_tx))),
    }
}

#[no_mangle]
pub extern "C" fn reveal_pk_attach_fee(
    reveal_pk_tx: RevealPk,
    fee: DenominatedAmount,
    token: CString,
    fee_payer: CString,
    epoch: u64,
    gas_limit: u64,
) -> CResult {
    let tx = reveal_pk_tx.to_native();
    let fee = fee.to_native();
    let token = match address::Address::from_str(&token.to_string()) {
        Ok(token) => token,
        e => return e.into(),
    };
    let fee_payer = match common::PublicKey::from_str(&fee_payer.to_string()) {
        Ok(token) => token,
        e => return e.into(),
    };
    CResult {
        is_err: false,
        error_msg: "".to_string().into(),
        value: allocate(RevealPk(allocate(tx.attach_fee(
            fee,
            token,
            fee_payer,
            epoch.into(),
            gas_limit.into(),
        )))),
    }
}

#[no_mangle]
pub extern "C" fn reveal_pk_get_fee_sign_bytes(reveal_pk_tx: &RevealPk) -> Hashes {
    let tx = reveal_pk_tx.to_native_ref();
    let hash = vec![tx.get_fee_sig_bytes()]
        .into_iter()
        .map(|hash| hash.0)
        .collect::<Vec<[u8; 32]>>()
        .into_boxed_slice();
    let len = hash.len();
    let ptr = Box::into_raw(hash) as *mut [u8; 32];
    Hashes { len, ptr }
}

#[no_mangle]
pub extern "C" fn attach_fee_signature(
    reveal_pk_tx: RevealPk,
    public_key: CString,
    signature: CString,
) -> CResult {
    let tx = reveal_pk_tx.to_native();
    let signer = match common::PublicKey::from_str(&public_key.to_string()) {
        Ok(pk) => pk,
        e => return e.into(),
    };
    let signature = match common::Signature::from_str(&signature.to_string()) {
        Ok(sig) => sig,
        e => return e.into(),
    };

    let signed_tx = tx.attach_fee_signature(signer, signature);

    CResult {
        is_err: false,
        error_msg: "".to_string().into(),
        value: allocate(RevealPk(allocate(signed_tx))),
    }
}

#[no_mangle]
pub extern "C" fn reveal_pk_validate_tx(reveal_pk: &RevealPk) -> CResult {
    let tx = reveal_pk.to_native_ref();
    match tx.validate_tx() {
        Ok(None) => CResult {
            is_err: true,
            error_msg: "This tx is not of correct type and will not rejected by the mempool"
                .to_string()
                .into(),
            value: std::ptr::null_mut(),
        },
        Ok(Some(_)) => CResult {
            is_err: false,
            error_msg: "".to_string().into(),
            value: std::ptr::null_mut(),
        },
        e => e.into(),
    }
}

#[no_mangle]
pub extern "C" fn reveal_pk_payload(reveal_pk_tx: RevealPk) -> Tx {
    let tx = reveal_pk_tx.0 as *mut account::RevealPk;
    let casted_tx = unsafe { tx.read() };
    Tx {
        inner: allocate(casted_tx.payload()),
    }
}
