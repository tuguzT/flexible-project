//! Model of the user microservice domain layer.

pub use self::display_name::*;
pub use self::email::*;
pub use self::id::*;
pub use self::name::*;
pub use self::role::*;
pub use self::user::*;

mod display_name;
mod email;
mod id;
mod name;
mod role;
mod user;
