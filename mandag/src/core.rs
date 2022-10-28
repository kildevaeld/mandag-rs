use crate::{
    app::App,
    phase::{Build, Init, ModuleBuildContext, Phase, Start},
    router::{into_routes_box, IntoRoutes, RouterService},
    Module,
};
use dale_extensions::State;
use dale_http::error::Error;

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
    pub async fn build(self) -> Core<Build> {
        Core {
            phase: Build {
                routes: Vec::default(),
                modules: Vec::default(),
            },
        }
    }
}

impl Core<Build> {
    pub fn route<R>(mut self, route: R) -> Self
    where
        R: IntoRoutes + Send + 'static,
        R::Error: std::error::Error + Send + Sync,
    {
        self.phase.routes.push(into_routes_box(route));
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

    pub async fn into_service(self) -> Result<State<RouterService, App>, Error> {
        let service = self.create().await?;
        let service = service.phase.service;
        // let mut router = Router::default();
        // for route in self.phase.routes {
        //     let routes = route.into_routes()?;
        //     for route in routes {
        //         router.register(route)?;
        //     }
        // }

        // let service = router.into_service().wrap(StateMiddleware::new(App {}));

        Ok(service)
    }
}
