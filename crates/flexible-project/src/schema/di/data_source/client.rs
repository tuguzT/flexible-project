use anyhow::Context;
use shaku::{Component, Module};

mod data {
    pub use fp_data::data_source::local::Client;
}

pub struct ClientImpl(());

impl<M> Component<M> for ClientImpl
where
    M: Module,
{
    type Interface = data::Client;

    type Parameters = ();

    fn build(_: &mut shaku::ModuleBuildContext<M>, _: Self::Parameters) -> Box<Self::Interface> {
        let client = data::Client::new()
            .with_context(|| "tried to create local data source client")
            .unwrap();
        Box::new(client)
    }
}
