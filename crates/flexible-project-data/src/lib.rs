#![warn(missing_docs)]
#![warn(clippy::all)]
#![forbid(unsafe_code)]

//! Flexible Project data layer library.

pub use error::Error;

pub mod data_source;
pub mod interactor;
pub mod model;
pub mod repository;

mod error;
