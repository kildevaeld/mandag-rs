use super::{Phase, Start};
use crate::{Module, ModuleBuildCtx};
use dale::{IntoOutcome, Service, ServiceExt};
use dale_extensions::StateMiddleware;
use dale_http::error::Error;
use johnfig::Config;
use mandag_core::{
    router::{IntoRoutes, RoutesBuilder, Routing},
    App, Reply, Request, Store,
};

pub struct ModuleBuildContext {
    builder: RoutesBuilder,
    config: Config,
    store: Store,
}

impl ModuleBuildCtx for ModuleBuildContext {
    fn config(&self) -> &Config {
        &self.config
    }

    fn get<S>(&self) -> Option<S>
    where
        S: Send + Clone + Sync + 'static,
    {
        self.store.get()
    }
}

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
    pub config: Config,
}

impl Build {
    pub async fn build(self) -> Result<Start, Error> {
        let store = self.store;

        let mut ctx = ModuleBuildContext {
            builder: self.routes,
            config: self.config,
            store,
        };

        for module in self.modules {
            module.build(&mut ctx).await?;
        }

        let (router, service) = ctx.builder.build()?;

        let service = service.wrap(StateMiddleware::new(App::new(
            router, ctx.store, ctx.config,
        )));

        let service = service.boxed().shared();

        Ok(Start { service })
    }
}

impl Phase for Build {}
