//! Definitions of requests are done by client of the user service.

use serde::{Deserialize, Serialize};

use crate::model::{ErasedId, Name, UserFilters};

pub use self::update::UpdateUserInput;

mod update;

/// Request from the clients of the user service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Request {
    /// Create new user in the system.
    CreateUser {
        /// Name of the new user.
        name: Name,
    },
    /// Filter users of the system.
    FilterUsers {
        /// User filters of the system.
        filters: Box<UserFilters>,
    },
    /// Update data of existing user of the system.
    UpdateUser {
        /// Identifier of the user to update.
        current_id: ErasedId,
        /// Data of the user to update.
        update: UpdateUserInput,
    },
    /// Delete user from the system.
    DeleteUser {
        /// Identifier of the user to delete.
        current_id: ErasedId,
    },
    // TODO other updates
}
