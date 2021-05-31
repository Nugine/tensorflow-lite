#![allow(clippy::missing_safety_doc)] // FIXME

#[allow(dead_code)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
mod bindings;

#[macro_use]
mod utils;

pub mod error;
pub mod interpreter;
pub mod model;
pub mod tensor;
