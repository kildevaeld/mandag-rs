use mandag::{
    async_trait, http::Request, prelude::*, router::Router, Core, Error, Extension, ExtensionCtx,
    Module, ModuleBuildCtx, Route,
};
use mandag_core::{Body, Json};

struct TestModule;

#[async_trait]
impl<C: ModuleBuildCtx> Module<C> for TestModule {
    async fn build(&self, ctx: &mut C) -> Result<(), Error> {
        ctx.mount(
            "/test",
            Route::get("/module", |_| async move { "Hello, Module" }),
        );
        ctx.get("/module", |_| async move { "Hello, Module" });

        Ok(())
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

#[mandag::post(path = "/mig/:sub", data = "data")]
fn post(data: Json<String>) {
    "Mif"
}

struct TestExt;

#[async_trait]
impl<C: ExtensionCtx> Extension<C> for TestExt {
    async fn init(&self, _ctx: &mut C) -> Result<(), Error> {
        println!("init extension");
        Ok(())
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Core::default()
        .attach(TestExt)
        .build()
        .await?
        .route(Route::get(
            "/",
            |_req: Request| async move { "Hello, World!" },
        ))
        .module(TestModule)
        .route(test)
        .route(Route::get("/hello", index.service()))
        .route(Router::default())
        .listen(([127, 0, 0, 1], 3000))
        .await?;

    Ok(())
}
