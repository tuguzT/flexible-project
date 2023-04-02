use std::{borrow::Borrow, pin::pin};

use futures::{StreamExt, TryStreamExt};

use crate::{
    model::{
        Email, EmailFilters, Name, NameFilters, User, UserDataFilters, UserFilters, UserId,
        UserIdFilters,
    },
    repository::UserDatabase,
};

pub async fn find_one_by_id<Id, Db>(database: &Db, id: Id) -> Result<Option<User>, Db::Error>
where
    Id: Borrow<UserId>,
    Db: UserDatabase,
{
    let id = id.borrow();
    let filter = {
        let id = UserIdFilters::builder().eq(id).build();
        UserFilters::builder().id(id).build()
    };
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
    let filter = {
        let name = NameFilters::builder().eq(name).build();
        let data = UserDataFilters::builder().name(name).build();
        UserFilters::builder().data(data).build()
    };
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
    let filter = {
        let email = EmailFilters::builder().eq(email).build();
        let data = UserDataFilters::builder().email(email).build();
        UserFilters::builder().data(data).build()
    };
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
