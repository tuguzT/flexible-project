use std::convert::identity;

use fp_filter::{Equal, In, NotEqual, NotIn, Regex};
use fp_user_domain::model::{
    DisplayName, DisplayNameFilters, Email, Name, NameFilters, OptionEmailFilters, Role,
    RoleFilters, UserDataFilters, UserFilters, UserId, UserIdFilters,
};
use mongodb::bson::{to_bson, Bson, Document};

use crate::model::{LocalRole, LocalUserId};

use super::user::LocalError;

pub trait IntoDocument {
    fn into_document(self) -> Result<Document, LocalError>;
}

impl IntoDocument for UserFilters<'_> {
    fn into_document(self) -> Result<Document, LocalError> {
        let Self { id, data } = self;

        let mut document = Document::new();
        if let Some(id) = id {
            document.insert("_id", id.into_document()?);
        }
        if let Some(data) = data {
            document.insert("data", data.into_document()?);
        }
        Ok(document)
    }
}

impl IntoDocument for UserDataFilters<'_> {
    fn into_document(self) -> Result<Document, LocalError> {
        let Self {
            name,
            display_name,
            role,
            email,
        } = self;

        let mut document = Document::new();
        if let Some(name) = name {
            document.insert("name", name.into_document()?);
        }
        if let Some(display_name) = display_name {
            document.insert("display_name", display_name.into_document()?);
        }
        if let Some(role) = role {
            document.insert("role", role.into_document()?);
        }
        if let Some(email) = email {
            document.insert("email", email.into_document()?);
        }
        Ok(document)
    }
}

impl IntoDocument for UserIdFilters<'_> {
    fn into_document(self) -> Result<Document, LocalError> {
        let Self {
            owner: _,
            eq,
            ne,
            r#in,
            nin,
        } = self;

        fn ids_to_bson(ids: &[UserId]) -> impl Iterator<Item = Result<Bson, LocalError>> + '_ {
            ids.iter()
                .cloned()
                .map(|id| LocalUserId::try_from(id).map_err(Into::into))
                .map(|result| {
                    result
                        .map(|id| to_bson(&id).map_err(Into::into))
                        .and_then(identity)
                })
        }

        let mut document = Document::new();
        if let Some(Equal(id)) = eq {
            let id = id.clone();
            let id = LocalUserId::try_from(id)?;
            document.insert("$eq", to_bson(&id)?);
        }
        if let Some(NotEqual(id)) = ne {
            let id = id.clone();
            let id = LocalUserId::try_from(id)?;
            document.insert("$ne", to_bson(&id)?);
        }
        if let Some(In(ids)) = r#in {
            let ids = ids_to_bson(ids).collect::<Result<Vec<_>, _>>()?;
            document.insert("$in", ids);
        }
        if let Some(NotIn(ids)) = nin {
            let ids = ids_to_bson(ids).collect::<Result<Vec<_>, _>>()?;
            document.insert("$nin", ids);
        }
        Ok(document)
    }
}

impl IntoDocument for NameFilters<'_> {
    fn into_document(self) -> Result<Document, LocalError> {
        let Self {
            eq,
            ne,
            r#in,
            nin,
            regex,
        } = self;

        let mut document = Document::new();
        if let Some(Equal(name)) = eq {
            document.insert("$eq", name.as_str());
        }
        if let Some(NotEqual(name)) = ne {
            document.insert("$ne", name.as_str());
        }
        if let Some(In(ids)) = r#in {
            let ids: Vec<_> = ids.iter().map(Name::as_str).collect();
            document.insert("$in", ids);
        }
        if let Some(NotIn(ids)) = nin {
            let ids: Vec<_> = ids.iter().map(Name::as_str).collect();
            document.insert("$nin", ids);
        }
        if let Some(Regex(regex)) = regex {
            document.insert("$regex", regex);
        }
        Ok(document)
    }
}

impl IntoDocument for DisplayNameFilters<'_> {
    fn into_document(self) -> Result<Document, LocalError> {
        let Self {
            eq,
            ne,
            r#in,
            nin,
            regex,
        } = self;

        let mut document = Document::new();
        if let Some(Equal(name)) = eq {
            document.insert("$eq", name.as_str());
        }
        if let Some(NotEqual(name)) = ne {
            document.insert("$ne", name.as_str());
        }
        if let Some(In(ids)) = r#in {
            let ids: Vec<_> = ids.iter().map(DisplayName::as_str).collect();
            document.insert("$in", ids);
        }
        if let Some(NotIn(ids)) = nin {
            let ids: Vec<_> = ids.iter().map(DisplayName::as_str).collect();
            document.insert("$nin", ids);
        }
        if let Some(Regex(regex)) = regex {
            document.insert("$regex", regex);
        }
        Ok(document)
    }
}

impl IntoDocument for RoleFilters<'_> {
    fn into_document(self) -> Result<Document, LocalError> {
        let Self { eq, ne, r#in, nin } = self;

        fn roles_to_bson(roles: &[Role]) -> impl Iterator<Item = Result<Bson, LocalError>> + '_ {
            roles
                .iter()
                .copied()
                .map(LocalRole::from)
                .map(|role| to_bson(&role).map_err(Into::into))
        }

        let mut document = Document::new();
        if let Some(Equal(&role)) = eq {
            let role = LocalRole::from(role);
            document.insert("$eq", to_bson(&role)?);
        }
        if let Some(NotEqual(&role)) = ne {
            let role = LocalRole::from(role);
            document.insert("$ne", to_bson(&role)?);
        }
        if let Some(In(roles)) = r#in {
            let roles = roles_to_bson(roles).collect::<Result<Vec<_>, _>>()?;
            document.insert("$in", roles);
        }
        if let Some(NotIn(roles)) = nin {
            let roles = roles_to_bson(roles).collect::<Result<Vec<_>, _>>()?;
            document.insert("$nin", roles);
        }
        Ok(document)
    }
}

impl IntoDocument for OptionEmailFilters<'_> {
    fn into_document(self) -> Result<Document, LocalError> {
        let Self {
            eq,
            ne,
            r#in,
            nin,
            regex,
        } = self;

        let mut document = Document::new();
        if let Some(Equal(email)) = eq {
            document.insert("$eq", email.as_ref().map(Email::as_str));
        }
        if let Some(NotEqual(email)) = ne {
            document.insert("$ne", email.as_ref().map(Email::as_str));
        }
        if let Some(In(emails)) = r#in {
            let emails: Vec<_> = emails
                .iter()
                .map(|email| email.as_ref().map(Email::as_str))
                .collect();
            document.insert("$in", emails);
        }
        if let Some(NotIn(emails)) = nin {
            let emails: Vec<_> = emails
                .iter()
                .map(|email| email.as_ref().map(Email::as_str))
                .collect();
            document.insert("$nin", emails);
        }
        if let Some(Regex(regex)) = regex {
            document.insert("$regex", regex);
        }
        Ok(document)
    }
}
