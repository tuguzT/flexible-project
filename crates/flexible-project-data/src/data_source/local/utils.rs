use fp_core::model::filter::{Contains, Equal, In, NotEqual, NotIn};
use fp_core::model::id::IdFilters;
use fp_core::model::user::{
    UserDisplayNameFilters, UserEmailFilters, UserFilters, UserRoleFilters, UsernameFilters,
};
use mongodb::bson::{to_bson, Document, Uuid};
use mongodb::{Collection, Database};

use crate::data_source::local::model::{UserData, UserRoleData};

pub trait UserCollection {
    fn user_collection(self) -> Collection<UserData>;
}

impl UserCollection for Database {
    fn user_collection(self) -> Collection<UserData> {
        self.collection("users")
    }
}

pub trait IntoDocument {
    // TODO: return error on failure
    fn into_document(self) -> Document;
}

impl IntoDocument for UserFilters {
    fn into_document(self) -> Document {
        let mut document = Document::new();
        if let Some(id) = self.id {
            document.insert("_id", id.into_document());
        }
        if let Some(name) = self.name {
            document.insert("name", name.into_document());
        }
        if let Some(display_name) = self.display_name {
            document.insert("display_name", display_name.into_document());
        }
        if let Some(email) = self.email {
            document.insert("email", email.into_document());
        }
        if let Some(role) = self.role {
            document.insert("role", role.into_document());
        }
        document
    }
}

impl<Owner> IntoDocument for IdFilters<Owner> {
    fn into_document(self) -> Document {
        let mut document = Document::new();
        if let Some(Equal(id)) = self.eq {
            let id = String::from(id);
            let id = Uuid::parse_str(id).expect("id is valid");
            document.insert("$eq", id);
        }
        if let Some(NotEqual(id)) = self.ne {
            let id = String::from(id);
            let id = Uuid::parse_str(id).expect("id is valid");
            document.insert("$ne", id);
        }
        if let Some(In(ids)) = self.r#in {
            let ids = ids
                .into_iter()
                .map(String::from)
                .map(|id| Uuid::parse_str(id).expect("id is valid"))
                .collect::<Vec<_>>();
            document.insert("$in", ids);
        }
        if let Some(NotIn(ids)) = self.nin {
            let ids = ids
                .into_iter()
                .map(String::from)
                .map(|id| Uuid::parse_str(id).expect("id is valid"))
                .collect::<Vec<_>>();
            document.insert("$nin", ids);
        }
        document
    }
}

impl IntoDocument for UsernameFilters {
    fn into_document(self) -> Document {
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
        if let Some(regex) = self.regex {
            document.insert("$regex", regex.as_str());
        }
        document
    }
}

impl IntoDocument for UserDisplayNameFilters {
    fn into_document(self) -> Document {
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
        if let Some(regex) = self.regex {
            document.insert("$regex", regex.as_str());
        }
        document
    }
}

impl IntoDocument for UserEmailFilters {
    fn into_document(self) -> Document {
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
        if let Some(regex) = self.regex {
            document.insert("$regex", regex.as_str());
        }
        document
    }
}

impl IntoDocument for UserRoleFilters {
    fn into_document(self) -> Document {
        let mut document = Document::new();
        if let Some(Equal(role)) = self.eq {
            let role = UserRoleData::from(role);
            let role = to_bson(&role).expect("user role is valid");
            document.insert("$eq", role);
        }
        if let Some(NotEqual(role)) = self.ne {
            let role = UserRoleData::from(role);
            let role = to_bson(&role).expect("user role is valid");
            document.insert("$ne", role);
        }
        if let Some(In(roles)) = self.r#in {
            let roles = roles
                .into_iter()
                .map(UserRoleData::from)
                .map(|role| to_bson(&role).expect("user role is valid"))
                .collect::<Vec<_>>();
            document.insert("$in", roles);
        }
        if let Some(NotIn(roles)) = self.nin {
            let roles = roles
                .into_iter()
                .map(UserRoleData::from)
                .map(|role| to_bson(&role).expect("user role is valid"))
                .collect::<Vec<_>>();
            document.insert("$nin", roles);
        }
        document
    }
}
