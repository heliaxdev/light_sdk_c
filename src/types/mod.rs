use std::ffi::c_void;

#[repr(C)]
pub struct Tx {
    pub inner: *mut c_void,
}

#[repr(C)]
pub struct Hashes {
    pub ptr: *mut [u8; 32],
    pub len: usize,
}
