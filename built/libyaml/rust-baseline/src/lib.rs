//! Rust transliteration of libyaml 0.2.5.
//!
//! Every non-static C function in `libyaml/src/*.c` is exposed here as
//! `#[no_mangle] pub extern "C"` with the same symbol name, and every public
//! type from `libyaml/include/yaml.h` is laid out `#[repr(C)]` to be
//! ABI-compatible with the original header.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(path_statements)]
#![allow(clippy::all)]

#[macro_use]
pub mod yaml_private;

pub mod externs;
pub mod yaml;

pub mod api;
pub mod dumper;
pub mod emitter;
pub mod loader;
pub mod parser;
pub mod reader;
pub mod scanner;
pub mod writer;

pub use yaml::*;
