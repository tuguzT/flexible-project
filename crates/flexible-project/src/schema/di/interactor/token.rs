use shaku::{Component, Module};

pub struct TokenSecret(pub String);

impl<M> Component<M> for TokenSecret
where
    M: Module,
{
    type Interface = Self;

    type Parameters = String;

    fn build(
        _: &mut shaku::ModuleBuildContext<M>,
        params: Self::Parameters,
    ) -> Box<Self::Interface> {
        Box::new(Self(params))
    }
}
