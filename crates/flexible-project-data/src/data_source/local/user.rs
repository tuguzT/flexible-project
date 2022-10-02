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

use super::utils::UserCollection;

/// Local user data source implementation.
pub struct LocalUserDataSource {
    collection: Collection<User>,
}

impl LocalUserDataSource {
    /// Creates new local user data source.
    pub async fn new(database: Database) -> Result<Self, LocalError> {
        let collection = database.user_collection();
        let indexes = [
            IndexModel::builder()
                .keys(doc! { "id": 1 })
                .options(IndexOptions::builder().unique(true).build())
                .build(),
            IndexModel::builder()
                .keys(doc! { "name": 1 })
                .options(IndexOptions::builder().unique(true).build())
                .build(),
            IndexModel::builder()
                .keys(doc! { "email": 1 })
                .options(
                    IndexOptions::builder()
                        .unique(true)
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
    type Error = LocalError;
}

#[derive(Debug, Display, From, Error)]
#[display(fmt = "local data source error: {}", _0)]
pub struct LocalError(#[error(source)] mongodb::error::Error);

#[async_trait]
impl Clear for LocalUserDataSource {
    async fn clear(&self) -> Result<(), Self::Error> {
        self.collection.delete_many(doc! {}, None).await?;
        Ok(())
    }
}

#[async_trait]
impl ReadAll for LocalUserDataSource {
    async fn read_all(&self) -> Result<Vec<Self::Item>, Self::Error> {
        let cursor = self.collection.find(None, None).await?;
        let vec = cursor.try_collect().await?;
        Ok(vec)
    }
}

#[async_trait]
impl ReadById for LocalUserDataSource {
    async fn read_by_id(
        &self,
        id: <Self::Item as Node>::Id,
    ) -> Result<Option<Self::Item>, Self::Error> {
        let filter = doc! { "id": id };
        let user = self.collection.find_one(filter, None).await?;
        Ok(user)
    }
}

#[async_trait]
impl Delete for LocalUserDataSource {
    async fn delete(&self, item: Self::Item) -> Result<Option<Self::Item>, Self::Error> {
        let query = to_document(&item).expect("should be valid");
        let user = self.collection.find_one_and_delete(query, None).await?;
        Ok(user)
    }
}

#[async_trait]
impl DeleteById for LocalUserDataSource {
    async fn delete_by_id(
        &self,
        id: <Self::Item as Node>::Id,
    ) -> Result<Option<Self::Item>, Self::Error> {
        let query = doc! { "id": id };
        let user = self.collection.find_one_and_delete(query, None).await?;
        Ok(user)
    }
}

#[async_trait]
impl Save for LocalUserDataSource {
    async fn save(&self, item: Self::Item) -> Result<Self::Item, Self::Error> {
        let filter = doc! { "id": &item.id };
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
