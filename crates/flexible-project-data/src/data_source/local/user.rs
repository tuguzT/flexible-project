use async_trait::async_trait;
use fp_core::model::id::Id;
use fp_core::model::user::{User, UserFilters};
use futures::future;
use futures::stream::TryStreamExt;
use mongodb::bson::{doc, to_bson};
use mongodb::options::IndexOptions;
use mongodb::{Collection, Database, IndexModel};

use crate::data_source::user::UserDataSource;
use crate::data_source::{DataSource, Result};

use super::model::{IdData, UserData, UserRoleData};
use super::utils::{IntoDocument, UserCollection};

/// Local user data source implementation.
pub struct LocalUserDataSource {
    collection: Collection<UserData>,
}

impl LocalUserDataSource {
    /// Creates new local user data source.
    pub async fn new(database: Database) -> Result<Self> {
        let collection = database.user_collection();
        let indexes = [
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
                            doc! { "email": { "$exists": true, "$type": "string" } },
                        )
                        .build(),
                )
                .build(),
        ];
        collection.create_indexes(indexes, None).await?;
        Ok(Self { collection })
    }
}

#[async_trait]
impl UserDataSource for LocalUserDataSource {
    async fn create(&self, user: Self::Item, password_hash: String) -> Result<Self::Item> {
        let user = UserData {
            id: user.id.try_into()?,
            name: user.name,
            display_name: user.display_name,
            email: user.email,
            password_hash,
            role: user.role.into(),
        };
        self.collection.insert_one(&user, None).await?;
        Ok(user.into())
    }

    async fn read(&self, filter: UserFilters) -> Result<Vec<Self::Item>> {
        let filter = filter.into_document()?;
        log::debug!("{:#?}", filter);
        let cursor = self.collection.find(filter, None).await?;
        let vec = cursor
            .and_then(|user| future::ok(User::from(user)))
            .try_collect()
            .await?;
        Ok(vec)
    }

    async fn update(&self, user: Self::Item) -> Result<Option<Self::Item>> {
        let id = IdData::try_from(user.id)?;
        let filter = doc! { "_id": to_bson(&id)? };
        let update = doc! {
            "name": &user.name,
            "display_name": &user.display_name,
            "email": user.email.as_deref(),
            "role": to_bson(&UserRoleData::from(user.role))?,
        };
        let old_user = self
            .collection
            .find_one_and_update(filter, update, None)
            .await?
            .map(Into::into);
        Ok(old_user)
    }

    async fn delete(&self, user: Self::Item) -> Result<Option<Self::Item>> {
        let id = IdData::try_from(user.id)?;
        let query = doc! { "_id": to_bson(&id)? };
        let user = self
            .collection
            .find_one_and_delete(query, None)
            .await?
            .map(Into::into);
        Ok(user)
    }

    async fn get_password_hash(&self, id: Id<User>) -> Result<Option<String>> {
        let id = IdData::try_from(id)?;
        let filter = doc! { "_id": to_bson(&id)? };
        let user = self.collection.find_one(filter, None).await?;
        let password_hash = user.map(|user| user.password_hash);
        Ok(password_hash)
    }
}

impl DataSource for LocalUserDataSource {
    type Item = User;
}
