use super::{Build, ModuleBuildContext, Phase};
use crate::{
    extension::{Extension, ExtensionCtx},
    router::{into_routes_box, IntoRoutesBox},
    store::Store,
    Module,
};
use dale_http::error::Error;

pub struct ExtensionContext {
    store: Store,
    routes: Vec<Box<dyn IntoRoutesBox>>,
    modules: Vec<Box<dyn Module<ModuleBuildContext>>>,
}

impl ExtensionCtx for ExtensionContext {
    fn store(&mut self) -> &mut Store {
        &mut self.store
    }

    fn route<R>(&mut self, route: R) -> &mut Self
    where
        R: crate::router::IntoRoutes + Sync + Send + 'static,
        R::Error: std::error::Error + Send + Sync,
    {
        self.routes.push(into_routes_box(route));
        self
    }
}

#[derive(Default)]
pub struct Init {
    pub extensions: Vec<Box<dyn Extension<ExtensionContext>>>,
}

impl Init {
    pub async fn build(self) -> Result<Build, Error> {
        let store = Store::new();
        let mut ctx = ExtensionContext {
            store,
            routes: Vec::default(),
            modules: Vec::default(),
        };

        for ext in &self.extensions {
            ext.init(&mut ctx).await?;
        }

        let build = Build {
            store: ctx.store,
            routes: ctx.routes,
            modules: ctx.modules,
        };

        Ok(build)
    }
}

impl Phase for Init {}
