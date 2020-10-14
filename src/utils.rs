pub mod log {
    #[allow(unused_macros)]
    #[macro_export]
    macro_rules! info {
        () => (
            unsafe {
                XPLMDebugString(
                    CString::new("\n")
                        .expect("CString::new failed")
                        .as_ptr(),
                );
            }
        );
        ($fmt:expr) => {
            unsafe {
                XPLMDebugString(
                    CString::new(format!($fmt))
                        .expect("CString::new failed")
                        .as_ptr(),
                );
            }
        };
        ($fmt:expr, $($args:tt)*) => {
            unsafe {
                XPLMDebugString(
                    CString::new(format!($fmt, $( $args ),*))
                        .expect("CString::new failed")
                        .as_ptr(),
                );
            }
        }
    }
}

use std::ffi::CString;
pub fn write_c_char(string: &str, ptr: *mut ::std::os::raw::c_char) -> anyhow::Result<()> {
    let c_str = CString::new(string)?;
    unsafe {
        std::ptr::copy_nonoverlapping(c_str.as_ptr(), ptr, c_str.as_bytes_with_nul().len());
    }
    Ok(())
}
