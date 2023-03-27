use std::sync::Arc;

use super::router::SharedRouter;
use super::{
    into_routes_box, IntoRoutes, IntoRoutesBox, IntoRoutesExt, Route, Router, RouterService,
};
use crate::{Reply, Request, Response};
use dale::IntoService;
use dale::{BoxService, IntoOutcome, Service, ServiceExt, VecService};
use dale_http::{Error, Method, Reply as _};

pub trait Routing {
    fn route<R>(&mut self, route: R) -> &mut Self
    where
        R: IntoRoutes + Sync + Send + 'static,
        R::Error: std::error::Error + Send + Sync;

    fn service<S>(&mut self, service: S) -> &mut Self
    where
        S: Service<Request> + Send + Sync + 'static,
        S::Future: Send,
        <S::Output as IntoOutcome<Request>>::Success: Reply + Send,
        <S::Output as IntoOutcome<Request>>::Failure: Into<Error>;
}

macro_rules! method {
    ($($name: ident => $method: ident),*) => {
        $(
            fn $name<P, S>(&mut self, path: P, service: S) -> &mut Self
        where
            P: ToString,
            S: Service<Request> + Send + Sync + 'static,
            S::Future: Send,
            <S::Output as IntoOutcome<Request>>::Success: Reply + Send,
            <S::Output as IntoOutcome<Request>>::Failure: Into<Error>,
        {
            self.route(Route::new(Method::$method.into(), path.to_string(), service));
            self
        }
        )*
    };
}

pub trait RoutingExt: Routing {
    fn mount<S, R>(&mut self, path: S, route: R) -> &mut Self
    where
        S: ToString,
        R: IntoRoutes + Sync + Send + 'static,
        R::Error: std::error::Error + Send + Sync,
    {
        self.route(route.mounted_on(path))
    }

    method!(
        get => GET,
        delete => DELETE,
        post => POST,
        put => PUT,
        patch => PATCH
    );
}

impl<R> RoutingExt for R where R: Routing {}

#[derive(Default)]
pub struct RoutesBuilder {
    pub routes: Vec<Box<dyn IntoRoutesBox>>,
    pub services: Vec<BoxService<'static, Request, Response, Error>>,
}

type RoutesBuilderService = dale::combinators::Unify<
    dale::combinators::Or<
        VecService<BoxService<'static, Request, Response, Error>>,
        RouterService,
        Request,
    >,
>;

impl RoutesBuilder {
    pub fn extend(&mut self, service: RoutesBuilder) -> &mut Self {
        self.routes.extend(service.routes);
        self.services.extend(service.services);
        self
    }

    pub fn build(self) -> Result<(SharedRouter, RoutesBuilderService), Error> {
        let service = VecService::new(self.services);

        let mut router = Router::default();
        for route in self.routes {
            let routes = route.into_routes()?;
            for route in routes {
                router.register(route)?;
            }
        }

        let router = Arc::new(router.into_inner());

        let service = service.or(RouterService::new(router.clone())).unify();

        Ok((router, service))
    }
}

impl IntoService<Request> for RoutesBuilder {
    type Error = Error;
    type Service = RoutesBuilderService;
    fn into_service(self) -> Result<Self::Service, Self::Error> {
        let (_, service) = self.build()?;
        Ok(service)
    }
}

impl Routing for RoutesBuilder {
    fn route<R>(&mut self, route: R) -> &mut Self
    where
        R: IntoRoutes + Sync + Send + 'static,
        R::Error: std::error::Error + Send + Sync,
    {
        self.routes.push(into_routes_box(route));
        self
    }

    fn service<S>(&mut self, service: S) -> &mut Self
    where
        S: Service<Request> + Send + Sync + 'static,
        S::Future: Send,
        <S::Output as IntoOutcome<Request>>::Success: Reply + Send,
        <S::Output as IntoOutcome<Request>>::Failure: Into<Error>,
    {
        self.services.push(
            service
                .then(
                    |ret: <S::Output as IntoOutcome<Request>>::Success| async move {
                        Result::<_, Error>::Ok(ret.into_response())
                    },
                )
                .err_into()
                .boxed(),
        );
        self
    }
}
