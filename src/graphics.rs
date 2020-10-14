use crate::bindings;

pub fn XPLMDrawString(
    mut color: [f32; 3],
    xOffset: i32,
    yOffset: i32,
    string: &str,
    wordWrapWidth: Option<i32>,
    fontID: bindings::XPLMFontID,
) {
    let bytes = String::from(format!("{}\0", string)).into_bytes();
    let mut cchars: Vec<::std::os::raw::c_char> =
        bytes.iter().map(|&b| b as ::std::os::raw::c_char).collect();
    let text: *mut ::std::os::raw::c_char = cchars.as_mut_ptr();
    let wordWrapWidthl = match wordWrapWidth {
        Some(mut v) => &mut v,
        None => std::ptr::null_mut(),
    };
    unsafe {
        bindings::XPLMDrawString(
            color.as_mut_ptr(),
            xOffset,
            yOffset,
            text,
            wordWrapWidthl,
            fontID,
        )
    };
}
