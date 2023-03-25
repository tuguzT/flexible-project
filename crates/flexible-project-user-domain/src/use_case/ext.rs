use std::{borrow::Borrow, pin::pin};

use async_trait::async_trait;
use fp_core::filter::Borrowed;
use futures::{StreamExt, TryStreamExt};

use crate::{
    model::{Email, EmailFilters, Name, NameFilters, User, UserFilters, UserId, UserIdFilters},
    repository::UserDatabase,
};

#[async_trait(?Send)]
pub trait UserDatabaseExt: UserDatabase {
    async fn find_one_by_id<Id>(&self, id: Id) -> Result<Option<User>, Self::Error>
    where
        Id: Borrow<UserId>;

    async fn find_one_by_name<N>(&self, name: N) -> Result<Option<User>, Self::Error>
    where
        N: Borrow<Name>;

    async fn find_one_by_email<E>(&self, email: E) -> Result<Option<User>, Self::Error>
    where
        E: Borrow<Option<Email>>;
}

#[async_trait(?Send)]
impl<Db> UserDatabaseExt for Db
where
    Db: UserDatabase,
{
    async fn find_one_by_id<Id>(&self, id: Id) -> Result<Option<User>, Self::Error>
    where
        Id: Borrow<UserId>,
    {
        let id = id.borrow();
        let filter = UserFilters::builder()
            .id(UserIdFilters::builder().eq(id.borrowed()).build())
            .build();
        let users = self.read(filter).await?;
        let mut users = pin!(users);
        let user = users.try_next().await?;
        debug_assert!(
            users.count().await == 0,
            "exactly one user should present with id {id}",
        );
        Ok(user)
    }

    async fn find_one_by_name<N>(&self, name: N) -> Result<Option<User>, Self::Error>
    where
        N: Borrow<Name>,
    {
        let name = name.borrow();
        let filter = UserFilters::builder()
            .name(NameFilters::builder().eq(name.borrowed()).build())
            .build();
        let users = self.read(filter).await?;
        let mut users = pin!(users);
        let user = users.try_next().await?;
        debug_assert!(
            users.count().await == 0,
            "exactly one user should present with name {name}",
        );
        Ok(user)
    }

    async fn find_one_by_email<E>(&self, email: E) -> Result<Option<User>, Self::Error>
    where
        E: Borrow<Option<Email>>,
    {
        let email = email.borrow();
        let filter = UserFilters::builder()
            .email(EmailFilters::builder().eq(email.borrowed()).build())
            .build();
        let users = self.read(filter).await?;
        let mut users = pin!(users);
        let user = users.try_next().await?;
        if let Some(email) = email {
            debug_assert!(
                users.count().await == 0,
                "exactly one user should present with email {email}",
            );
        }
        Ok(user)
    }
}
