use std::convert::Infallible;

use mandag::{
    async_trait, http::Request, prelude::*, router::Router, Core, Module, ModuleBuildCtx, Route,
};
use mandag_core::Body;

struct TestModule;

#[async_trait]
impl<C: ModuleBuildCtx> Module<C> for TestModule {
    async fn build(&self, ctx: &mut C) {
        ctx.route(Route::get("/module", |_| async move { "Hello, Module" }));
    }
}

#[mandag::handler(data = "data")]
fn index(data: Body) {
    "Hello, World!"
}

#[mandag::get(path = "/mig/:sub")]
fn test() {
    "Mif"
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Core::default()
        .build()
        .await
        .route(Route::get(
            "/",
            |_req: Request| async move { "Hello, World!" },
        ))
        .module(TestModule)
        .route(test)
        .route(Route::get("/hello", index.service()))
        .route(Router::default())
        .into_service()
        .await?
        .err_into::<dale_http::error::Error>()
        .listen(([127, 0, 0, 1], 3000))
        .await?;

    Ok(())
}
