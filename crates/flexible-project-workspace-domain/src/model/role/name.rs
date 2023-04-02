use std::borrow::Borrow;

use derive_more::{Display, Error};
use fp_core::filter::{Equal, Filter, In, NotEqual, NotIn, Regex};
use fp_user_domain::model::Name as UserName;
use typed_builder::TypedBuilder;

/// Name of workspace role with strong requirements about its content.
///
/// This requirements are the same as for user names.
#[derive(Debug, Display, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RoleName(UserName);

impl RoleName {
    /// Creates new workspace role name from input string.
    ///
    /// # Errors
    ///
    /// This function will return an error
    /// if input string does not match workspace role name requirements.
    pub fn new(name: impl Into<String>) -> Result<Self, RoleNameError> {
        let name = UserName::new(name).map_err(|_| RoleNameError::Invalid)?;
        Ok(Self(name))
    }

    /// Extracts string slice from a workspace role name.
    pub fn as_str(&self) -> &str {
        let Self(name) = self;
        name.as_str()
    }

    /// Converts workspace role name into a string.
    pub fn into_inner(self) -> String {
        let Self(name) = self;
        name.into_inner()
    }
}

/// Type of error which is returned when input does not meet workspace role name requirements.
#[derive(Debug, Display, Clone, Copy, Error)]
pub enum RoleNameError {
    /// Workspace role name does not meet requirements.
    #[display(fmt = "workspace role name does not meet requirements")]
    Invalid,
}

/// Filters for workspace role name of the backend.
#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct RoleNameFilters<'a> {
    /// Equality workspace role name filter.
    pub eq: Option<Equal<&'a RoleName>>,
    /// Inequality workspace role name filter.
    pub ne: Option<NotEqual<&'a RoleName>>,
    /// In workspace role name filter.
    pub r#in: Option<In<&'a [RoleName]>>,
    /// Not in workspace role name filter.
    pub nin: Option<NotIn<&'a [RoleName]>>,
    /// Regex workspace role name filter.
    pub regex: Option<Regex<&'a str>>,
}

impl<Input> Filter<Input> for RoleNameFilters<'_>
where
    Input: Borrow<RoleName>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self {
            eq,
            ne,
            r#in,
            nin,
            regex,
        } = self;
        let input = input.borrow();
        eq.satisfies(input)
            && ne.satisfies(input)
            && r#in.satisfies(input)
            && nin.satisfies(input)
            && regex.satisfies(input.as_str())
    }
}
