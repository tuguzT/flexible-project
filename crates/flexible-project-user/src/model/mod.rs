//! Data model of the user service.

pub use self::{
    avatar::Avatar,
    display_name::DisplayName,
    email::Email,
    id::ErasedId,
    name::Name,
    role::Role,
    user::{TryFromUserDataError, User, UserData},
};

pub mod filter;

mod avatar;
mod display_name;
mod email;
mod id;
mod name;
mod role;
mod user;
