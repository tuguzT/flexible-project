use crate::{model::UserFilters, repository::UserDatabase};

/// Filter users interactor.
pub struct FilterUsers<Db>
where
    Db: UserDatabase,
{
    database: Db,
}

impl<Db> FilterUsers<Db>
where
    Db: UserDatabase,
{
    /// Creates new filter users interactor.
    pub fn new(database: Db) -> Self {
        Self { database }
    }

    /// Filters users by provided filter object.
    pub async fn filter_users(&self, filter: UserFilters<'_>) -> Result<Db::Users, Db::Error> {
        let Self { database } = self;
        database.read(filter).await
    }
}
