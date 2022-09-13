//! Trait definitions of operational repositories.
//!
//! *Operational* repository is a repository trait which implements exactly one operation.
//! This can be useful to destruct one huge trait into many small traits.

pub use clear::Clear;
pub use delete::{Delete, DeleteById};
pub use read::{ReadAll, ReadById};
pub use save::Save;

mod clear;
mod delete;
mod read;
mod save;
