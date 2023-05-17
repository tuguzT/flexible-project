//! Model of the user microservice domain layer.

pub use self::{
    avatar::{Avatar, AvatarError, AvatarFilters, OptionAvatarFilters},
    display_name::{DisplayName, DisplayNameError, DisplayNameFilters},
    email::{Email, EmailError, EmailFilters, OptionEmailFilters},
    id::{UserId, UserIdFilters},
    name::{Name, NameError, NameFilters},
    role::{Role, RoleFilters},
    user::{User, UserData, UserDataFilters, UserFilters},
};

mod avatar;
mod display_name;
mod email;
mod id;
mod name;
mod role;
mod user;
