use crate::router::Routing;
use async_trait::async_trait;
use dale_http::Error;
use johnfig::Config;

pub trait ModuleBuildCtx: Routing + Send + Sync {
    fn config(&self) -> &Config;
    fn get<S>(&self) -> Option<S>
    where
        S: Send + Clone + Sync + 'static;
}

#[async_trait]
pub trait Module<C: ModuleBuildCtx>: Send + Sync {
    async fn build(&self, ctx: &mut C) -> Result<(), Error>;
}
