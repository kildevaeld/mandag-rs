use crate::router::Routing;
use mandag_core::async_trait;

pub trait ModuleBuildCtx: Routing + Send + Sync {}

#[async_trait]
pub trait Module<C: ModuleBuildCtx>: Send + Sync {
    async fn build(&self, ctx: &mut C);
}
