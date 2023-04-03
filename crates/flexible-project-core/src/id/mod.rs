//! Data model of identifier of the backend.

pub use self::{
    filter::IdFilters,
    model::{ErasedId, Id},
};

mod filter;
mod model;
