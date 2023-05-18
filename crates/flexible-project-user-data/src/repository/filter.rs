use std::{borrow::Borrow, convert::identity};

use fp_filter::{CowSlice, Equal, In, NotEqual, NotIn, Regex};
use fp_user_domain::model::{
    Avatar, DisplayName, DisplayNameFilters, Email, Name, NameFilters, OptionAvatarFilters,
    OptionEmailFilters, Role, RoleFilters, UserDataFilters, UserFilters, UserId, UserIdFilters,
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
            avatar,
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
        if let Some(avatar) = avatar {
            document.insert("avatar", avatar.into_document()?);
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
            let id = id.into_owned();
            let id = LocalUserId::try_from(id)?;
            document.insert("$eq", to_bson(&id)?);
        }
        if let Some(NotEqual(id)) = ne {
            let id = id.into_owned();
            let id = LocalUserId::try_from(id)?;
            document.insert("$ne", to_bson(&id)?);
        }
        if let Some(In(CowSlice(ids))) = r#in {
            let ids = ids_to_bson(ids.borrow()).collect::<Result<Vec<_>, _>>()?;
            document.insert("$in", ids);
        }
        if let Some(NotIn(CowSlice(ids))) = nin {
            let ids = ids_to_bson(ids.borrow()).collect::<Result<Vec<_>, _>>()?;
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
        if let Some(In(CowSlice(ids))) = r#in {
            let ids: Vec<_> = ids.iter().map(Name::as_str).collect();
            document.insert("$in", ids);
        }
        if let Some(NotIn(CowSlice(ids))) = nin {
            let ids: Vec<_> = ids.iter().map(Name::as_str).collect();
            document.insert("$nin", ids);
        }
        if let Some(Regex(regex)) = regex {
            document.insert("$regex", &*regex);
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
        if let Some(In(CowSlice(ids))) = r#in {
            let ids: Vec<_> = ids.iter().map(DisplayName::as_str).collect();
            document.insert("$in", ids);
        }
        if let Some(NotIn(CowSlice(ids))) = nin {
            let ids: Vec<_> = ids.iter().map(DisplayName::as_str).collect();
            document.insert("$nin", ids);
        }
        if let Some(Regex(regex)) = regex {
            document.insert("$regex", &*regex);
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
        if let Some(Equal(role)) = eq {
            let role = role.into_owned();
            let role = LocalRole::from(role);
            document.insert("$eq", to_bson(&role)?);
        }
        if let Some(NotEqual(role)) = ne {
            let role = role.into_owned();
            let role = LocalRole::from(role);
            document.insert("$ne", to_bson(&role)?);
        }
        if let Some(In(CowSlice(roles))) = r#in {
            let roles = roles_to_bson(roles.borrow()).collect::<Result<Vec<_>, _>>()?;
            document.insert("$in", roles);
        }
        if let Some(NotIn(CowSlice(roles))) = nin {
            let roles = roles_to_bson(roles.borrow()).collect::<Result<Vec<_>, _>>()?;
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
            let email = email.into_owned();
            document.insert("$eq", email.as_ref().map(Email::as_str));
        }
        if let Some(NotEqual(email)) = ne {
            let email = email.into_owned();
            document.insert("$ne", email.as_ref().map(Email::as_str));
        }
        if let Some(In(CowSlice(emails))) = r#in {
            let emails: Vec<_> = emails
                .iter()
                .map(|email| email.as_ref().map(Email::as_str))
                .collect();
            document.insert("$in", emails);
        }
        if let Some(NotIn(CowSlice(emails))) = nin {
            let emails: Vec<_> = emails
                .iter()
                .map(|email| email.as_ref().map(Email::as_str))
                .collect();
            document.insert("$nin", emails);
        }
        if let Some(Regex(regex)) = regex {
            document.insert("$regex", &*regex);
        }
        Ok(document)
    }
}

impl IntoDocument for OptionAvatarFilters<'_> {
    fn into_document(self) -> Result<Document, LocalError> {
        let Self {
            eq,
            ne,
            r#in,
            nin,
            regex,
        } = self;

        let mut document = Document::new();
        if let Some(Equal(avatar)) = eq {
            let avatar = avatar.into_owned();
            document.insert("$eq", avatar.as_ref().map(Avatar::as_str));
        }
        if let Some(NotEqual(avatar)) = ne {
            let avatar = avatar.into_owned();
            document.insert("$ne", avatar.as_ref().map(Avatar::as_str));
        }
        if let Some(In(CowSlice(avatars))) = r#in {
            let avatars: Vec<_> = avatars
                .iter()
                .map(|avatar| avatar.as_ref().map(Avatar::as_str))
                .collect();
            document.insert("$in", avatars);
        }
        if let Some(NotIn(CowSlice(avatars))) = nin {
            let avatars: Vec<_> = avatars
                .iter()
                .map(|avatar| avatar.as_ref().map(Avatar::as_str))
                .collect();
            document.insert("$nin", avatars);
        }
        if let Some(Regex(regex)) = regex {
            document.insert("$regex", &*regex);
        }
        Ok(document)
    }
}
