use std::hash::Hash;

/// Type of identifier which are used to identify objects of the owner type.
pub trait Id<Owner>: Eq + Hash + Clone + 'static
where
    Owner: ?Sized + Node,
{
}

/// Type which can be identified by its [identifier](Node::Id).
pub trait Node {
    /// Type of identifier used to identify this object.
    type Id: Id<Self>;

    /// Returns an identifier of the object.
    fn id(&self) -> Self::Id;
}
