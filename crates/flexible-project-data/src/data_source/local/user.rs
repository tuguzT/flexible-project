use async_trait::async_trait;
use derive_more::{Display, Error, From};
use fp_core::model::Node;
use futures::stream::TryStreamExt;
use mongodb::bson::{doc, to_document};
use mongodb::options::{FindOneAndReplaceOptions, IndexOptions, ReturnDocument};
use mongodb::{Collection, Database, IndexModel};

use crate::data_source::ops::{Clear, Delete, DeleteById, ReadAll, ReadById, Save};
use crate::data_source::user::UserDataSource;
use crate::data_source::DataSource;
use crate::model::User;

/// Local user data source implementation.
pub struct LocalUserDataSource {
    collection: Collection<User>,
}

impl LocalUserDataSource {
    /// Creates new local user data source.
    pub async fn new(database: Database) -> Result<Self, LocalError> {
        let collection = database.collection("users");
        let indexes = [
            IndexModel::builder().keys(doc! { "id": 1 }).build(),
            IndexModel::builder().keys(doc! { "name": 1 }).build(),
            IndexModel::builder()
                .keys(doc! { "email": 1 })
                .options(
                    IndexOptions::builder()
                        .partial_filter_expression(
                            doc! { "email": { "$exists": true, "$ne": null } },
                        )
                        .build(),
                )
                .build(),
        ];
        collection.create_indexes(indexes, None).await?;
        Ok(Self { collection })
    }
}

impl UserDataSource for LocalUserDataSource {}

impl DataSource for LocalUserDataSource {
    type Item = User;
}

#[derive(Debug, Display, From, Error)]
#[display(fmt = "local data source error: {}", _0)]
pub struct LocalError(#[error(source)] mongodb::error::Error);

#[async_trait]
impl Clear for LocalUserDataSource {
    type Error = LocalError;

    async fn clear(&self) -> Result<(), Self::Error> {
        self.collection.drop(None).await?;
        Ok(())
    }
}

#[async_trait]
impl ReadAll for LocalUserDataSource {
    type Error = LocalError;

    async fn read_all(&self) -> Result<Vec<Self::Item>, Self::Error> {
        let cursor = self.collection.find(None, None).await?;
        let vec = cursor.try_collect().await?;
        Ok(vec)
    }
}

#[async_trait]
impl ReadById for LocalUserDataSource {
    type Error = LocalError;

    async fn read_by_id(
        &self,
        id: <Self::Item as Node>::Id,
    ) -> Result<Option<Self::Item>, Self::Error> {
        let filter = doc! { "id": id.to_string() };
        let user = self.collection.find_one(filter, None).await?;
        Ok(user)
    }
}

/// Error that can occur when deleting some user from the system.
#[derive(Error, Debug, Display, From)]
pub enum DeleteUserError {
    /// User was not found.
    #[display(fmt = "user not found")]
    NotFound,
    /// Other kind of error.
    #[from(types(mongodb::error::Error))]
    Other(#[error(source)] LocalError),
}

#[async_trait]
impl Delete for LocalUserDataSource {
    type Error = DeleteUserError;

    async fn delete(&self, item: Self::Item) -> Result<Self::Item, Self::Error> {
        let query = to_document(&item).expect("should be valid");
        let user = self.collection.find_one_and_delete(query, None).await?;
        match user {
            None => Err(DeleteUserError::NotFound),
            Some(user) => Ok(user),
        }
    }
}

#[async_trait]
impl DeleteById for LocalUserDataSource {
    type Error = DeleteUserError;

    async fn delete_by_id(&self, id: <Self::Item as Node>::Id) -> Result<Self::Item, Self::Error> {
        let query = doc! { "id": id.to_string() };
        let user = self.collection.find_one_and_delete(query, None).await?;
        match user {
            None => Err(DeleteUserError::NotFound),
            Some(user) => Ok(user),
        }
    }
}

#[async_trait]
impl Save for LocalUserDataSource {
    type Error = LocalError;

    async fn save(&self, item: Self::Item) -> Result<Self::Item, Self::Error> {
        let filter = doc! { "id": item.id.to_string() };
        let options = FindOneAndReplaceOptions::builder()
            .return_document(ReturnDocument::After)
            .build();
        let user = self
            .collection
            .find_one_and_replace(filter, item, options)
            .await?
            .expect("`returnDocument: after` was provided; should return new value which cannot be `None`");
        Ok(user)
    }
}
