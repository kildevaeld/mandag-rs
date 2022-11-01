use super::{Build, Phase};
use crate::{
    extension::{Extension, ExtensionCtx},
    router::{IntoRoutes, RoutesBuilder, Routing},
    store::Store,
};
use dale::{IntoOutcome, Service};
use dale_http::error::Error;
use mandag_core::{Reply, Request};

pub struct ExtensionContext {
    store: Store,
    routes: RoutesBuilder,
}

impl ExtensionCtx for ExtensionContext {
    fn store(&mut self) -> &mut Store {
        &mut self.store
    }
}

impl Routing for ExtensionContext {
    fn route<R>(&mut self, route: R) -> &mut Self
    where
        R: IntoRoutes + Sync + Send + 'static,
        R::Error: std::error::Error + Send + Sync,
    {
        self.routes.route(route);
        self
    }

    fn service<S>(&mut self, service: S) -> &mut Self
    where
        S: Service<Request> + Send + Sync + 'static,
        S::Future: Send,
        <S::Output as IntoOutcome<Request>>::Success: Reply + Send,
        <S::Output as IntoOutcome<Request>>::Failure: Into<Error>,
    {
        self.routes.service(service);
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
            routes: RoutesBuilder::default(),
        };

        for ext in &self.extensions {
            ext.init(&mut ctx).await?;
        }

        let build = Build {
            store: ctx.store,
            routes: ctx.routes,
            modules: Vec::default(),
        };

        Ok(build)
    }
}

impl Phase for Init {}
