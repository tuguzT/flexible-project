use std::pin::pin;

use async_trait::async_trait;
use derive_more::{Display, Error, From};
use domain::{
    model::{User, UserData, UserFilters, UserId},
    repository::UserDatabase,
};
use futures::Stream;
use mongodb::{
    bson::{doc, ser, to_bson},
    error::Error,
    options::{FindOneAndUpdateOptions, IndexOptions, ReturnDocument},
    results::InsertOneResult,
    Collection, Cursor, IndexModel,
};

use crate::{
    client::Client,
    model::{LocalUser, LocalUserData, LocalUserDataError, LocalUserId, LocalUserIdError},
};

use super::filter::IntoDocument;

/// Local database of user data.
pub struct LocalUserDatabase {
    collection: Collection<LocalUser>,
}

impl LocalUserDatabase {
    /// Creates new local user repository instance.
    pub async fn new(client: Client) -> Result<Self, LocalError> {
        let database = client.inner.database("flexible-project-user");
        let collection = database.collection("user");

        let name_index = {
            let options = IndexOptions::builder().unique(true).build();
            IndexModel::builder()
                .keys(doc! { "name": 1 })
                .options(options)
                .build()
        };
        let email_index = {
            let options = IndexOptions::builder()
                .unique(true)
                .partial_filter_expression(doc! { "email": { "$exists": true, "$type": "string" } })
                .build();
            IndexModel::builder()
                .keys(doc! { "email": 1 })
                .options(options)
                .build()
        };
        collection
            .create_indexes([name_index, email_index], None)
            .await?;

        Ok(Self { collection })
    }
}

#[async_trait(?Send)]
impl UserDatabase for LocalUserDatabase {
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
    async fn read(&self, filter: UserFilters<'_>) -> Result<Self::Users, Self::Error> {
        let Self { collection } = self;
        let filter = filter.into_document()?;
        let users = LocalUsers {
            cursor: collection.find(filter, None).await?,
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
        } = data.into();

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
#[derive(Debug)]
pub struct LocalUsers {
    cursor: Cursor<LocalUser>,
}

impl Stream for LocalUsers {
    type Item = Result<User, LocalError>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        fn to_user(result: Result<LocalUser, Error>) -> Result<User, LocalError> {
            match result {
                Ok(user) => user.try_into().map_err(Into::into),
                Err(error) => Err(error.into()),
            }
        }

        let cursor = pin!(&mut self.cursor);
        cursor.poll_next(cx).map(|user| user.map(to_user))
    }
}
