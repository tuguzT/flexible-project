pub use self::{
    avatar::{UpdateAvatar, UpdateAvatarError},
    display_name::{UpdateDisplayName, UpdateDisplayNameError},
    email::{UpdateEmail, UpdateEmailError},
    name::{UpdateName, UpdateNameError},
    user::{UpdateUser, UpdateUserError, UpdateUserInput},
};

mod avatar;
mod display_name;
mod email;
mod name;
mod user;
