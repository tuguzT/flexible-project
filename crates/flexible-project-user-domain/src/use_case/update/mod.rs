pub use self::{
    display_name::{UpdateDisplayName, UpdateDisplayNameError},
    email::{UpdateEmail, UpdateEmailError},
    name::{UpdateName, UpdateNameError},
    user::{UpdateUser, UpdateUserError},
};

mod display_name;
mod email;
mod name;
mod user;
