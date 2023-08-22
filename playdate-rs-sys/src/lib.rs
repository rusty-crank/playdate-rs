#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(all(target_arch = "arm", target_os = "none"))]
mod thumbv7em_bindings;

#[cfg(all(target_arch = "arm", target_os = "none"))]
pub use thumbv7em_bindings::*;
