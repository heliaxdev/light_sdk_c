use std::ffi::c_void;

#[repr(C)]
pub struct Tx {
    pub(crate) inner: *mut c_void,
}

impl Tx {
    pub(crate) fn to_native(self) -> namada_light_sdk::namada_core::proto::Tx {
        let native = self.inner as *mut namada_light_sdk::namada_core::proto::Tx;
        unsafe {
            native.read()
        }
    }

}

#[repr(C)]
pub struct DenominatedAmount {
    pub(crate) inner: *mut c_void,
}

impl DenominatedAmount {
    pub fn to_native(self) -> namada_light_sdk::namada_core::types::token::DenominatedAmount {
        let native = self.inner as *mut namada_light_sdk::namada_core::types::token::DenominatedAmount;
        unsafe {
            native.read()
        }
    }
}

#[repr(C)]
pub struct Hashes {
    pub ptr: *mut [u8; 32],
    pub len: usize,
}
