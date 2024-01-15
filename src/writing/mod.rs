use crate::types::Tx;
use crate::utils::{allocate, CResult, CString};
use namada_light_sdk::writing::blocking;

#[no_mangle]
pub extern "C" fn broadcast_tx(tendermint_addr: CString, tx: Tx) -> CResult {
    let to_broadcast = tx.to_native();
    match blocking::broadcast_tx(&tendermint_addr.to_string(), to_broadcast) {
        Ok(resp) => CResult {
            is_err: false,
            error_msg: "".to_string().into(),
            value: allocate(resp.code.is_ok()),
        },
        e => e.into(),
    }
}
