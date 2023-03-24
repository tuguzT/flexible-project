//! Implementation of local user repository.

use std::pin::pin;

use async_trait::async_trait;
use derive_more::{Display, Error, From};
use domain::{
    model::{User, UserData, UserFilters, UserId},
    use_case::Repository,
};
use futures::{stream::Map, Stream, StreamExt};
use mongodb::{
    bson::{doc, ser, to_bson},
    error::Error,
    options::{FindOneAndUpdateOptions, ReturnDocument},
    results::InsertOneResult,
    Collection, Cursor,
};

use crate::model::{LocalUser, LocalUserData, LocalUserDataError, LocalUserId, LocalUserIdError};

/// Local repository of user data.
pub struct LocalRepository {
    collection: Collection<LocalUser>,
}

impl LocalRepository {
    // TODO constructor with MongoDB indexes setup
}

#[async_trait]
impl Repository for LocalRepository {
    type Error = LocalError;

    async fn create(&self, id: UserId, data: UserData) -> Result<User, Self::Error> {
        let Self { collection } = self;
        let user = User { id, data }.try_into()?;
        let InsertOneResult { inserted_id, .. } = collection.insert_one(&user, None).await?;

        let filter = doc! { "_id": inserted_id };
        let user = collection
            .find_one(filter, None)
            .await?
            .expect("user was just created");
        let user = user.try_into()?;
        Ok(user)
    }

    type Users = LocalUsers;
    async fn read(&self, _filter: UserFilters<'_>) -> Result<Self::Users, Self::Error> {
        fn to_user(result: Result<LocalUser, Error>) -> Result<User, LocalError> {
            match result {
                Ok(user) => User::try_from(user).map_err(LocalError::from),
                Err(e) => Err(LocalError::from(e)),
            }
        }

        let Self { collection } = self;
        let filter = doc! {}; // TODO document from UserFilters object
        let users = LocalUsers {
            stream: collection.find(filter, None).await?.map(to_user),
        };
        Ok(users)
    }

    async fn update(&self, id: UserId, data: UserData) -> Result<User, Self::Error> {
        let Self { collection } = self;
        let id = LocalUserId::try_from(id)?;
        let LocalUserData {
            name,
            display_name,
            role,
            email,
        } = LocalUserData::from(data);

        let filter = doc! { "_id": to_bson(&id)? };
        let update = doc! {
            "name": name,
            "display_name": display_name,
            "role": to_bson(&role)?,
            "email": email,
        };
        let options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();
        let user = collection
            .find_one_and_update(filter, update, options)
            .await?;
        let user = user.ok_or(LocalErrorKind::NoUser)?.try_into()?;
        Ok(user)
    }

    async fn delete(&self, id: UserId) -> Result<User, Self::Error> {
        let Self { collection } = self;
        let id = LocalUserId::try_from(id)?;

        let filter = doc! { "_id": to_bson(&id)? };
        let user = collection.find_one_and_delete(filter, None).await?;
        let user = user.ok_or(LocalErrorKind::NoUser)?.try_into()?;
        Ok(user)
    }
}

/// Type of error which is returned on local repository failure.
#[derive(Debug, Display, Clone, From, Error)]
#[from(forward)]
pub struct LocalError {
    kind: LocalErrorKind,
}

#[derive(Debug, Display, Clone, From, Error)]
enum LocalErrorKind {
    #[display(fmt = "no user was found by provided identifier")]
    NoUser,
    Id(LocalUserIdError),
    UserData(LocalUserDataError),
    ToBson(ser::Error),
    Database(Error),
}

/// Stream of filtered user data from local repository.
pub struct LocalUsers {
    stream: Map<Cursor<LocalUser>, FnToUser>,
}

type FnToUser = fn(Result<LocalUser, Error>) -> Result<User, LocalError>;

impl Stream for LocalUsers {
    type Item = Result<User, LocalError>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let stream = pin!(&mut self.stream);
        stream.poll_next(cx)
    }
}
