//! Use cases of the user microservice domain layer.

pub use self::{
    create::{CreateUser, CreateUserError},
    delete::{DeleteUser, DeleteUserError},
    read::FilterUsers,
    update::*,
};

mod create;
mod delete;
mod find_one;
mod read;
mod update;
