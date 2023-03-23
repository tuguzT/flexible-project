use crate::model::UserFilters;

use super::repository::Repository;

/// Filters users by provided filter object.
pub async fn filter_users<R>(repository: R, filter: UserFilters<'_>) -> Result<R::Users, R::Error>
where
    R: Repository,
{
    repository.read(filter).await
}
