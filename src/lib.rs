#![allow(dead_code, unused)]

pub mod builder;
pub mod convert;
pub mod error;
pub mod metadata;
pub mod object_type;
mod utils;
pub mod validator;

pub type Result<T> = std::result::Result<T, error::Error>;
