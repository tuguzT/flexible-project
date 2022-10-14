#![warn(missing_docs)]
#![warn(clippy::all)]
#![forbid(unsafe_code)]

//! Flexible Project data layer library.

pub use error::{Error, Result};

pub mod data_source;
pub mod interactor;
pub mod repository;

mod error;
