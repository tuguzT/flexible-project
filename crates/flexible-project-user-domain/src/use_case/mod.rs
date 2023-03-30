//! Use cases of the user microservice domain layer.

pub use self::{
    create::{CreateUser, CreateUserError},
    delete::{DeleteUser, DeleteUserError},
    read::FilterUsers,
    update::{
        display_name::{UpdateDisplayName, UpdateDisplayNameError},
        email::{UpdateEmail, UpdateEmailError},
        name::{UpdateName, UpdateNameError},
        role::{UpdateRole, UpdateRoleError},
        user::{UpdateUser, UpdateUserError},
    },
};

mod create;
mod delete;
mod find_one;
mod read;
mod update;
