use crate::router::Routing;
use dale_http::Error;
use johnfig::Config;
use mandag_core::async_trait;

pub trait ModuleBuildCtx: Routing + Send + Sync {
    fn config(&self) -> &Config;
}

#[async_trait]
pub trait Module<C: ModuleBuildCtx>: Send + Sync {
    async fn build(&self, ctx: &mut C) -> Result<(), Error>;
}
