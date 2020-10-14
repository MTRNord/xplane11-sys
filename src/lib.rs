#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod display;
pub mod graphics;

pub mod utils;
pub mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

// TODO enum from XPLMWindowPositioningMode
