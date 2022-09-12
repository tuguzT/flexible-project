use std::hash::Hash;

/// Trait for types of identifiers
/// which can be used to identify objects of owner type.
pub trait Id<Owner>: Eq + Ord + Hash + Clone + 'static
where
    Owner: ?Sized + Identifiable,
{
}

/// Trait for types which can be identified by [`Id`](Identifiable::Id) associated type.
pub trait Identifiable {
    /// Type of identifier used to identify the instance of identifiable.
    type Id: Id<Self>;

    /// Get an identifier of the object.
    fn id(&self) -> &Self::Id;
}
