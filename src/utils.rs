use std::error::Error;
use std::ffi::{c_char, CStr};
use std::fmt::{Debug, Display, Formatter};
use std::ffi::c_void;

#[repr(transparent)]
#[derive(Debug)]
pub struct CString(*const c_char);


impl From<String> for CString {
    fn from(value: String) -> Self {
        let bytes = value.into_bytes();
        let mut c_chars: Vec<i8> = bytes.iter().map(| c | *c as i8).collect::<Vec<i8>>();
        c_chars.push(0);
        Self(c_chars.as_ptr())
    }
}

impl From<CString> for String {
    fn from(value: CString) -> Self {
        let slice = unsafe {
            CStr::from_ptr(value.0)
        };
        slice.to_str().unwrap().to_string()
    }
}

impl Display for CString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let slice = unsafe {
            CStr::from_ptr(self.0)
        };
        let msg = match slice.to_str() {
            Err(e) => {
                println!("Could not parse {} as a CString", e.to_string());
                return Err(std::fmt::Error::default());
            }
            Ok(s) => s,
        };
        f.write_str(msg)
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
        self.map_err(|e| {
            format!("{}\n{}", msg, e.to_string()).into()
        })
    }
}

#[repr(C)]
pub enum CResult<T> {
    Ok(T),
    Err(CString)
}

/// Put a rust object into the heap and return
/// a pointer to it. Rust will forget this object
/// and it will be the responsibility of the user code
/// to free it.
pub fn allocate<T>(obj: T) -> *mut c_void {
    let boxed = Box::new(obj);
    Box::leak(boxed) as *mut T as *mut c_void
}
