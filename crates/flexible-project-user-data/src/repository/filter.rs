use std::convert::identity;

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
        if let Some(eq) = eq {
            let id = eq.0.clone();
            let id = LocalUserId::try_from(id)?;
            document.insert("$eq", to_bson(&id)?);
        }
        if let Some(ne) = ne {
            let id = ne.0.clone();
            let id = LocalUserId::try_from(id)?;
            document.insert("$ne", to_bson(&id)?);
        }
        if let Some(r#in) = r#in {
            let ids = r#in.0;
            let ids = ids_to_bson(ids).collect::<Result<Vec<_>, _>>()?;
            document.insert("$in", ids);
        }
        if let Some(nin) = nin {
            let ids = nin.0;
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
        if let Some(eq) = eq {
            let name = eq.0;
            document.insert("$eq", name.as_str());
        }
        if let Some(ne) = ne {
            let name = ne.0;
            document.insert("$ne", name.as_str());
        }
        if let Some(r#in) = r#in {
            let ids = r#in.0;
            let ids: Vec<_> = ids.iter().map(Name::as_str).collect();
            document.insert("$in", ids);
        }
        if let Some(nin) = nin {
            let ids = nin.0;
            let ids: Vec<_> = ids.iter().map(Name::as_str).collect();
            document.insert("$nin", ids);
        }
        if let Some(regex) = regex {
            let regex = regex.0;
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
        if let Some(eq) = eq {
            let name = eq.0;
            document.insert("$eq", name.as_str());
        }
        if let Some(ne) = ne {
            let name = ne.0;
            document.insert("$ne", name.as_str());
        }
        if let Some(r#in) = r#in {
            let ids = r#in.0;
            let ids: Vec<_> = ids.iter().map(DisplayName::as_str).collect();
            document.insert("$in", ids);
        }
        if let Some(nin) = nin {
            let ids = nin.0;
            let ids: Vec<_> = ids.iter().map(DisplayName::as_str).collect();
            document.insert("$nin", ids);
        }
        if let Some(regex) = regex {
            let regex = regex.0;
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
        if let Some(eq) = eq {
            let role = *eq.0;
            let role = LocalRole::from(role);
            document.insert("$eq", to_bson(&role)?);
        }
        if let Some(ne) = ne {
            let role = *ne.0;
            let role = LocalRole::from(role);
            document.insert("$ne", to_bson(&role)?);
        }
        if let Some(r#in) = r#in {
            let roles = r#in.0;
            let roles = roles_to_bson(roles).collect::<Result<Vec<_>, _>>()?;
            document.insert("$in", roles);
        }
        if let Some(nin) = nin {
            let roles = nin.0;
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
        if let Some(eq) = eq {
            let email = eq.0;
            document.insert("$eq", email.as_ref().map(Email::as_str));
        }
        if let Some(ne) = ne {
            let email = ne.0;
            document.insert("$ne", email.as_ref().map(Email::as_str));
        }
        if let Some(r#in) = r#in {
            let emails = r#in.0;
            let emails: Vec<_> = emails
                .iter()
                .map(|email| email.as_ref().map(Email::as_str))
                .collect();
            document.insert("$in", emails);
        }
        if let Some(nin) = nin {
            let emails = nin.0;
            let emails: Vec<_> = emails
                .iter()
                .map(|email| email.as_ref().map(Email::as_str))
                .collect();
            document.insert("$nin", emails);
        }
        if let Some(regex) = regex {
            let regex = regex.0;
            document.insert("$regex", regex);
        }
        Ok(document)
    }
}
