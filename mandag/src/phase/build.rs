use super::{Phase, Start};
use crate::{
    app::App,
    module::{Module, ModuleBuildCtx},
    router::{IntoRoutes, RoutesBuilder, Routing},
    store::Store,
};
use dale::{IntoOutcome, Service, ServiceExt};
use dale_extensions::StateMiddleware;
use dale_http::error::Error;
use mandag_core::{Reply, Request};

#[derive(Default)]
pub struct ModuleBuildContext {
    builder: RoutesBuilder,
}

impl ModuleBuildCtx for ModuleBuildContext {}

impl Routing for ModuleBuildContext {
    fn route<R>(&mut self, route: R) -> &mut Self
    where
        R: IntoRoutes + Sync + Send + 'static,
        R::Error: std::error::Error + Send + Sync,
    {
        self.builder.route(route);
        self
    }

    fn service<S>(&mut self, service: S) -> &mut Self
    where
        S: Service<Request> + Send + Sync + 'static,
        S::Future: Send,
        <S::Output as IntoOutcome<Request>>::Success: Reply + Send,
        <S::Output as IntoOutcome<Request>>::Failure: Into<Error>,
    {
        self.builder.service(service);
        self
    }
}

pub struct Build {
    pub store: Store,
    pub routes: RoutesBuilder,
    pub modules: Vec<Box<dyn Module<ModuleBuildContext>>>,
}

impl Build {
    pub async fn build(self) -> Result<Start, Error> {
        let store = self.store;

        let mut ctx = ModuleBuildContext {
            builder: self.routes,
        };

        for module in self.modules {
            module.build(&mut ctx).await;
        }

        let service = ctx
            .builder
            .into_service()?
            .wrap(StateMiddleware::new(App::new(store)))
            .boxed()
            .shared();

        Ok(Start { service })
    }
}

impl Phase for Build {}
