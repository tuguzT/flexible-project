//! Use cases of the user microservice domain layer.

pub use self::{
    create::{create_user, CreateUserError},
    delete::{delete_user, DeleteUserError},
    read::filter_users,
    repository::Repository,
    update::{
        update_display_name, update_email, update_name, update_role, UpdateDisplayNameError,
        UpdateEmailError, UpdateNameError, UpdateRoleError,
    },
};

mod create;
mod delete;
mod read;
mod repository;
mod update;
