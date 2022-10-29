use mandag_core::async_trait;

use crate::router::IntoRoutes;

pub trait ModuleBuildCtx: Send + Sync {
    fn route<R>(&mut self, route: R) -> &mut Self
    where
        R: IntoRoutes + Sync + Send + 'static,
        R::Error: std::error::Error + Send + Sync;
}

#[async_trait]
pub trait Module<C: ModuleBuildCtx>: Send + Sync {
    async fn build(&self, ctx: &mut C);
}
