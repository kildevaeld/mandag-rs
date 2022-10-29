use dale_http::error::Error;
use mandag_core::async_trait;

use crate::store::Store;

pub trait ExtensionCtx: Send + Sync {
    fn store(&mut self) -> &mut Store;
    fn route<R>(&mut self, route: R) -> &mut Self
    where
        R: crate::router::IntoRoutes + Sync + Send + 'static,
        R::Error: std::error::Error + Send + Sync;
}

#[async_trait]
pub trait Extension<C>: Send
where
    C: ExtensionCtx,
{
    async fn init(&self, ctx: &mut C) -> Result<(), Error>;
}
