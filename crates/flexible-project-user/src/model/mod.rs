//! Data model of the user service.

pub use self::{
    avatar::{Avatar, AvatarFilters, OptionAvatarFilters},
    display_name::{DisplayName, DisplayNameFilters},
    email::{Email, EmailFilters, OptionEmailFilters},
    id::{ErasedId, ErasedIdFilters},
    name::{Name, NameFilters},
    role::{Role, RoleFilters},
    user::{TryFromUserDataError, User, UserData, UserDataFilters, UserFilters},
};

pub mod filter;

mod avatar;
mod display_name;
mod email;
mod id;
mod name;
mod role;
mod user;
