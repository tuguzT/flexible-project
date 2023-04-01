//! Model of the user microservice domain layer.

pub use self::{
    display_name::{DisplayName, DisplayNameError, DisplayNameFilters},
    email::{Email, EmailError, EmailFilters},
    id::{UserId, UserIdFilters},
    name::{Name, NameError, NameFilters},
    role::{Role, RoleFilters},
    user::{User, UserData, UserDataFilters, UserFilters},
};

mod display_name;
mod email;
mod id;
mod name;
mod role;
mod user;
