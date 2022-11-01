use crate::{
    extension::Extension,
    phase::{Build, ExtensionContext, Init, ModuleBuildContext, Phase, Start},
    router::{IntoRoutes, Routing},
    types::IntoService,
    Module,
};
use dale::{combinators::shared::SharedService, BoxService};
use dale_http::error::Error;
use mandag_core::{Request, Response};
use mandag_serve::ServiceServeExt;
use std::net::SocketAddr;

pub struct Core<P: Phase> {
    phase: P,
}

impl Default for Core<Init> {
    fn default() -> Self {
        Core {
            phase: Init::default(),
        }
    }
}

impl Core<Init> {
    pub async fn build(self) -> Result<Core<Build>, Error> {
        let phase = self.phase.build().await?;

        let core = Core { phase };

        Ok(core)
    }

    pub fn attach<E>(mut self, extension: E) -> Self
    where
        E: Extension<ExtensionContext> + 'static,
    {
        self.phase.extensions.push(Box::new(extension));
        self
    }
}

impl Core<Build> {
    pub fn route<R>(mut self, route: R) -> Self
    where
        R: IntoRoutes + Sync + Send + 'static,
        R::Error: std::error::Error + Send + Sync,
    {
        self.phase.routes.route(route);
        self
    }

    pub fn module<M>(mut self, module: M) -> Self
    where
        M: Module<ModuleBuildContext> + 'static,
    {
        self.phase.modules.push(Box::new(module));
        self
    }

    pub async fn create(self) -> Result<Core<Start>, Error> {
        let start = self.phase.build().await?;
        Ok(Core { phase: start })
    }

    pub async fn into_service(
        self,
    ) -> Result<SharedService<BoxService<'static, Request, Response, Error>>, Error> {
        let service = self.create().await?.phase.into_service();
        Ok(service)
    }

    pub async fn listen<I>(self, incoming: I) -> Result<(), Error>
    where
        I: Into<SocketAddr>,
    {
        Ok(self.into_service().await?.listen(incoming).await?)
    }
}
