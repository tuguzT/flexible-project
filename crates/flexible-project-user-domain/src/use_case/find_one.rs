use std::{borrow::Borrow, pin::pin};

use fp_core::filter::Borrowed;
use futures::{StreamExt, TryStreamExt};

use crate::{
    model::{Email, EmailFilters, Name, NameFilters, User, UserFilters, UserId, UserIdFilters},
    repository::UserDatabase,
};

pub async fn find_one_by_id<Id, Db>(database: &Db, id: Id) -> Result<Option<User>, Db::Error>
where
    Id: Borrow<UserId>,
    Db: UserDatabase,
{
    let id = id.borrow();
    let filter = UserFilters::builder()
        .id(UserIdFilters::builder().eq(id.borrowed()).build())
        .build();
    let users = database.read(filter).await?;
    let mut users = pin!(users);
    let user = users.try_next().await?;
    debug_assert!(
        users.count().await == 0,
        "exactly one user should present with id {id}",
    );
    Ok(user)
}

pub async fn find_one_by_name<N, Db>(database: &Db, name: N) -> Result<Option<User>, Db::Error>
where
    N: Borrow<Name>,
    Db: UserDatabase,
{
    let name = name.borrow();
    let filter = UserFilters::builder()
        .name(NameFilters::builder().eq(name.borrowed()).build())
        .build();
    let users = database.read(filter).await?;
    let mut users = pin!(users);
    let user = users.try_next().await?;
    debug_assert!(
        users.count().await == 0,
        "exactly one user should present with name {name}",
    );
    Ok(user)
}

pub async fn find_one_by_email<E, Db>(database: &Db, email: E) -> Result<Option<User>, Db::Error>
where
    E: Borrow<Option<Email>>,
    Db: UserDatabase,
{
    let email = email.borrow();
    let filter = UserFilters::builder()
        .email(EmailFilters::builder().eq(email.borrowed()).build())
        .build();
    let users = database.read(filter).await?;
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
