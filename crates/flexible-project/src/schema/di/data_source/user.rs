use std::sync::Arc;

use anyhow::Context;
use futures::executor::block_on;
use shaku::{Component, HasComponent, Interface, Module};
use tokio::runtime::Handle;

mod data {
    pub use fp_data::data_source::local::{Client, LocalUserDataSource};
    pub use fp_data::data_source::user::UserDataSource;
}

pub trait UserDataSource: data::UserDataSource + Interface {
    fn upcast(self: Arc<Self>) -> Arc<dyn data::UserDataSource>;
}

impl<T> UserDataSource for T
where
    T: data::UserDataSource + Interface,
{
    fn upcast(self: Arc<Self>) -> Arc<dyn data::UserDataSource> {
        self
    }
}

pub struct UserDataSourceImpl(());

impl<M> Component<M> for UserDataSourceImpl
where
    M: Module + HasComponent<data::Client>,
{
    type Interface = dyn UserDataSource;

    type Parameters = ();

    fn build(
        context: &mut shaku::ModuleBuildContext<M>,
        _: Self::Parameters,
    ) -> Box<Self::Interface> {
        let client = M::build_component(context);

        let handle = Handle::current();
        let _ = handle.enter();
        let data_source = block_on(data::LocalUserDataSource::new(client))
            .with_context(|| "tried to create local user data source")
            .unwrap();

        Box::new(data_source)
    }
}
