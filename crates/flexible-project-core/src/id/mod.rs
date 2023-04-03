//! Data model of identifier of the backend.

pub use self::{
    filter::{ErasedIdFilters, IdFilters},
    model::{ErasedId, ErasedOwner, Id},
};

mod filter;
mod model;
