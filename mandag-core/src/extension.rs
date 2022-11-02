use crate::router::Routing;
use async_trait::async_trait;
use dale_http::error::Error;
use johnfig::Config;

pub trait ExtensionCtx: Routing + Send + Sync {
    fn config(&self) -> &Config;

    fn register<S>(&mut self, i: S)
    where
        S: Send + Sync + Clone + 'static;
}

#[async_trait]
pub trait Extension<C>: Send
where
    C: ExtensionCtx,
{
    async fn build(&self, ctx: &mut C) -> Result<(), Error>;
}
