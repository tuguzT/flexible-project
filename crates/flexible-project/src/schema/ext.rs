use std::sync::Arc;

use async_graphql::Context;
use shaku::{HasComponent, Interface};

use super::di::SchemaModule;

pub(super) trait ContextExt {
    fn resolve<I>(&self) -> Arc<I>
    where
        I: Interface + ?Sized,
        SchemaModule: HasComponent<I>;

    fn resolve_ref<I>(&self) -> &I
    where
        I: Interface + ?Sized,
        SchemaModule: HasComponent<I>;
}

impl ContextExt for Context<'_> {
    fn resolve<I>(&self) -> Arc<I>
    where
        I: Interface + ?Sized,
        SchemaModule: HasComponent<I>,
    {
        let module = self.data_unchecked::<SchemaModule>();
        module.resolve()
    }

    fn resolve_ref<I>(&self) -> &I
    where
        I: Interface + ?Sized,
        SchemaModule: HasComponent<I>,
    {
        let module = self.data_unchecked::<SchemaModule>();
        module.resolve_ref()
    }
}
