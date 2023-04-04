//! Data model of identifier of the backend.

pub use self::{
    filter::{ErasedIdFilters, IdFilters},
    gen::GenerateId,
    model::{ErasedId, ErasedOwner, Id},
};

mod filter;
mod gen;
mod model;
