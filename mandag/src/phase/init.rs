use super::{Build, Phase};
use crate::{Extension, ExtensionCtx};
use dale::{IntoOutcome, Service};
use dale_http::error::Error;
use johnfig::{Config, ConfigBuilder};
use mandag_core::{
    router::{IntoRoutes, RoutesBuilder, Routing},
    Reply, Request, Store,
};

pub struct ExtensionContext {
    store: Store,
    routes: RoutesBuilder,
    config: Config,
}

impl ExtensionCtx for ExtensionContext {
    fn register<S>(&mut self, i: S)
    where
        S: Send + Sync + Clone + 'static,
    {
        self.store.insert(i)
    }
    fn config(&self) -> &Config {
        &self.config
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

pub struct Init {
    pub extensions: Vec<Box<dyn Extension<ExtensionContext>>>,
    pub config: ConfigBuilder,
}

impl Default for Init {
    fn default() -> Self {
        Init {
            extensions: Vec::default(),
            config: ConfigBuilder::new().with_name_pattern("*.{ext}"),
        }
    }
}

impl Init {
    pub async fn build(self) -> Result<Build, Error> {
        let config = self.config;

        let cfg = tokio::task::spawn_blocking(move || config.build_config())
            .await
            .map_err(Error::new)?
            .expect("config");

        let store = Store::new();
        let mut ctx = ExtensionContext {
            store,
            routes: RoutesBuilder::default(),
            config: cfg,
        };

        for ext in &self.extensions {
            ext.build(&mut ctx).await?;
        }

        let build = Build {
            store: ctx.store,
            routes: ctx.routes,
            modules: Vec::default(),
            config: ctx.config,
        };

        Ok(build)
    }
}

impl Phase for Init {}
