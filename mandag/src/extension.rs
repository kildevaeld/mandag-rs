use crate::{router::Routing, store::Store};
use dale_http::error::Error;
use mandag_core::async_trait;

pub trait ExtensionCtx: Routing + Send + Sync {
    fn store(&mut self) -> &mut Store;
}

#[async_trait]
pub trait Extension<C>: Send
where
    C: ExtensionCtx,
{
    async fn init(&self, ctx: &mut C) -> Result<(), Error>;
}
