use super::{Phase, Start};
use crate::{
    app::App,
    module::{Module, ModuleBuildCtx},
    router::{into_routes_box, IntoRoutesBox, Router},
    store::Store,
    types::IntoService,
};
use dale::ServiceExt;
use dale_extensions::StateMiddleware;
use dale_http::error::Error;

#[derive(Default)]
pub struct ModuleBuildContext {
    pub routes: Vec<Box<dyn IntoRoutesBox>>,
}

impl ModuleBuildCtx for ModuleBuildContext {
    fn route<R>(&mut self, route: R) -> &mut Self
    where
        R: crate::router::IntoRoutes + Sync + Send + 'static,
        R::Error: std::error::Error + Send + Sync,
    {
        self.routes.push(into_routes_box(route));
        self
    }
}

pub struct Build {
    pub store: Store,
    pub routes: Vec<Box<dyn IntoRoutesBox>>,
    pub modules: Vec<Box<dyn Module<ModuleBuildContext>>>,
}

impl Build {
    pub async fn build(self) -> Result<Start, Error> {
        let store = self.store;
        let mut router = Router::default();
        for route in self.routes {
            let routes = route.into_routes()?;
            for route in routes {
                router.register(route)?;
            }
        }

        let mut ctx = ModuleBuildContext {
            routes: Vec::default(),
        };

        for module in self.modules {
            module.build(&mut ctx).await;
        }

        let routes = ctx.routes;

        for route in routes {
            let routes = route.into_routes()?;
            for route in routes {
                router.register(route)?;
            }
        }

        let service = router
            .into_service()
            .wrap(StateMiddleware::new(App::new(store)));

        Ok(Start { service })
    }
}

impl Phase for Build {}
