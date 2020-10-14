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
