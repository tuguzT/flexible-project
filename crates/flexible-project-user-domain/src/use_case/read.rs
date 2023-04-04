use crate::{model::UserFilters, repository::UserDatabase};

/// Filter users interactor.
pub struct FilterUsers<Database>
where
    Database: UserDatabase,
{
    database: Database,
}

impl<Database> FilterUsers<Database>
where
    Database: UserDatabase,
{
    /// Creates new filter users interactor.
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    /// Filters users by provided filter object.
    pub async fn filter_users(
        &self,
        filter: UserFilters<'_>,
    ) -> Result<Database::Users, Database::Error> {
        let Self { database } = self;
        database.read(filter).await
    }
}
