use std::convert;
use std::result::Result as CoreResult;

use fp_core::model::filter::{Contains, Equal, In, NotEqual, NotIn, Regex};
use fp_core::model::id::IdFilters;
use fp_core::model::user::{
    UserDisplayNameFilters, UserEmailFilters, UserFilters, UserRoleFilters, UsernameFilters,
};
use mongodb::bson::{to_bson, Document};
use mongodb::{Collection, Database};

use super::{
    model::{IdData, UserData, UserRoleData},
    Error, Result,
};

pub trait UserCollection {
    fn user_collection(self) -> Collection<UserData>;
}

impl UserCollection for Database {
    fn user_collection(self) -> Collection<UserData> {
        self.collection("users")
    }
}

pub trait IntoDocument {
    fn into_document(self) -> Result<Document>;
}

impl IntoDocument for UserFilters {
    fn into_document(self) -> Result<Document> {
        let mut document = Document::new();
        if let Some(id) = self.id {
            document.insert("_id", id.into_document()?);
        }
        if let Some(name) = self.name {
            document.insert("name", name.into_document()?);
        }
        if let Some(display_name) = self.display_name {
            document.insert("display_name", display_name.into_document()?);
        }
        if let Some(email) = self.email {
            document.insert("email", email.into_document()?);
        }
        if let Some(role) = self.role {
            document.insert("role", role.into_document()?);
        }
        Ok(document)
    }
}

impl<Owner> IntoDocument for IdFilters<Owner> {
    fn into_document(self) -> Result<Document> {
        let mut document = Document::new();
        if let Some(Equal(id)) = self.eq {
            let id = IdData::try_from(id)?;
            let id = to_bson(&id)?;
            document.insert("$eq", id);
        }
        if let Some(NotEqual(id)) = self.ne {
            let id = IdData::try_from(id)?;
            let id = to_bson(&id)?;
            document.insert("$ne", id);
        }
        if let Some(In(ids)) = self.r#in {
            let ids = ids
                .into_iter()
                .map(|id| IdData::try_from(id).map_err(Error::from))
                .map(|res| {
                    res.map(|id| to_bson(&id).map_err(Error::from))
                        .and_then(convert::identity)
                })
                .collect::<Result<Vec<_>>>()?;
            document.insert("$in", ids);
        }
        if let Some(NotIn(ids)) = self.nin {
            let ids = ids
                .into_iter()
                .map(|id| IdData::try_from(id).map_err(Error::from))
                .map(|res| {
                    res.map(|id| to_bson(&id).map_err(Error::from))
                        .and_then(convert::identity)
                })
                .collect::<Result<Vec<_>>>()?;
            document.insert("$nin", ids);
        }
        Ok(document)
    }
}

impl IntoDocument for UsernameFilters {
    fn into_document(self) -> Result<Document> {
        let mut document = Document::new();
        if let Some(Equal(id)) = self.eq {
            document.insert("$eq", id);
        }
        if let Some(NotEqual(id)) = self.ne {
            document.insert("$ne", id);
        }
        if let Some(In(ids)) = self.r#in {
            document.insert("$in", ids);
        }
        if let Some(NotIn(ids)) = self.nin {
            document.insert("$nin", ids);
        }
        if let Some(Contains(substring)) = self.contains {
            document.insert("$regex", substring);
        }
        if let Some(Regex(regex)) = self.regex {
            document.insert("$regex", regex);
        }
        Ok(document)
    }
}

impl IntoDocument for UserDisplayNameFilters {
    fn into_document(self) -> Result<Document> {
        let mut document = Document::new();
        if let Some(Equal(id)) = self.eq {
            document.insert("$eq", id);
        }
        if let Some(NotEqual(id)) = self.ne {
            document.insert("$ne", id);
        }
        if let Some(In(ids)) = self.r#in {
            document.insert("$in", ids);
        }
        if let Some(NotIn(ids)) = self.nin {
            document.insert("$nin", ids);
        }
        if let Some(Contains(substring)) = self.contains {
            document.insert("$regex", substring);
        }
        if let Some(Regex(regex)) = self.regex {
            document.insert("$regex", regex);
        }
        Ok(document)
    }
}

impl IntoDocument for UserEmailFilters {
    fn into_document(self) -> Result<Document> {
        let mut document = Document::new();
        if let Some(Equal(id)) = self.eq {
            document.insert("$eq", id);
        }
        if let Some(NotEqual(id)) = self.ne {
            document.insert("$ne", id);
        }
        if let Some(In(ids)) = self.r#in {
            document.insert("$in", ids);
        }
        if let Some(NotIn(ids)) = self.nin {
            document.insert("$nin", ids);
        }
        if let Some(Contains(substring)) = self.contains {
            document.insert("$regex", substring);
        }
        if let Some(Regex(regex)) = self.regex {
            document.insert("$regex", regex);
        }
        Ok(document)
    }
}

impl IntoDocument for UserRoleFilters {
    fn into_document(self) -> Result<Document> {
        let mut document = Document::new();
        if let Some(Equal(role)) = self.eq {
            let role = UserRoleData::from(role);
            let role = to_bson(&role)?;
            document.insert("$eq", role);
        }
        if let Some(NotEqual(role)) = self.ne {
            let role = UserRoleData::from(role);
            let role = to_bson(&role)?;
            document.insert("$ne", role);
        }
        if let Some(In(roles)) = self.r#in {
            let roles = roles
                .into_iter()
                .map(UserRoleData::from)
                .map(|role| to_bson(&role))
                .collect::<CoreResult<Vec<_>, _>>()?;
            document.insert("$in", roles);
        }
        if let Some(NotIn(roles)) = self.nin {
            let roles = roles
                .into_iter()
                .map(UserRoleData::from)
                .map(|role| to_bson(&role))
                .collect::<CoreResult<Vec<_>, _>>()?;
            document.insert("$nin", roles);
        }
        Ok(document)
    }
}
