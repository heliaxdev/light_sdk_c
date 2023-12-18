use std::error::Error;
use std::ffi::c_void;
use std::ffi::{c_char, CStr};
use std::fmt::{Debug, Display, Formatter};

#[repr(transparent)]
#[derive(Debug)]
pub struct CString(*const c_char);

impl From<String> for CString {
    fn from(value: String) -> Self {
        let bytes = value.into_bytes();
        let mut c_chars: Vec<i8> = bytes.iter().map(|c| *c as i8).collect::<Vec<i8>>();
        c_chars.push(0);
        let reference = Box::leak(Box::new(c_chars));
        Self(reference.as_ptr())
    }
}

impl From<CString> for String {
    fn from(value: CString) -> Self {
        let slice = unsafe { CStr::from_ptr(value.0) };
        slice.to_string_lossy().to_string()
    }
}

impl Display for CString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let slice = unsafe { CStr::from_ptr(self.0) };
        f.write_str(slice.to_string_lossy().as_ref())
    }
}

impl Error for CString {}

pub trait FormatErr {
    type Result;
    fn format_err(self) -> Result<Self::Result, CString>;
    fn wrap_err(self, msg: &str) -> Result<Self::Result, CString>;
}

impl<R, E: Error> FormatErr for Result<R, E> {
    type Result = R;
    fn format_err(self) -> Result<R, CString> {
        self.map_err(|e| e.to_string().into())
    }

    fn wrap_err(self, msg: &str) -> Result<R, CString> {
        self.map_err(|e| format!("{}\n{}", msg, e).into())
    }
}

impl<T, E: Error> From<Result<T, E>> for CResult {
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(ty) => CResult {
                is_err: false,
                error_msg: "".to_string().into(),
                value: allocate(ty),
            },
            Err(e) => CResult {
                is_err: true,
                error_msg: e.to_string().into(),
                value: std::ptr::null_mut(),
            },
        }
    }
}

#[repr(C)]
pub struct CResult {
    pub is_err: bool,
    pub error_msg: CString,
    pub value: *mut c_void,
}

/// Put a rust object into the heap and return
/// a pointer to it. Rust will forget this object
/// and it will be the responsibility of the user code
/// to free it.
pub fn allocate<T>(obj: T) -> *mut c_void {
    let boxed = Box::new(obj);
    Box::leak(boxed) as *mut T as *mut c_void
}
