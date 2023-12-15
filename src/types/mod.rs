use crate::utils::allocate;
use light_sdk::namada_core::proto::Tx as NamadaTx;
use std::{ffi::c_void, ops::DerefMut};

#[repr(C)]
pub struct Tx {
    pub inner: *mut c_void,
}

#[repr(C)]
pub struct Hashes {
    pub ptr: *mut [u8; 32],
    pub len: usize,
}
