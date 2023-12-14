use std::ffi::c_void;
use crate::utils::allocate;
use light_sdk::namada_core::proto::Tx as NamadaTx;

#[repr(C)]
pub struct Tx {
    inner: *mut c_void,
}

impl From<NamadaTx> for Tx {
    fn from(value: NamadaTx) -> Self {
        Self {
            inner: allocate(value),
        }
    }
}
