use std::{convert::Infallible, sync::Arc};

use dale::{boxed::BoxFuture, BoxService, Outcome, Service};
use dale_http::{error::Error, router::Params, Method, StatusCode};
use mandag_core::{Body, Request, Response};
use router::IntoRoutes as _;

use crate::types::IntoService;

use super::{IntoRoutes, StaticRoute};

pub struct RouteEntry {
    method: Method,
    handler: BoxService<'static, Request, Response, Error>,
}

impl RouteEntry {
    pub fn new(
        method: Method,
        handler: BoxService<'static, Request, Response, Error>,
    ) -> RouteEntry {
        RouteEntry { method, handler }
    }
}

pub struct Router {
    i: router::Router<RouteEntry>,
}

impl Default for Router {
    fn default() -> Self {
        Router {
            i: router::Router::new(),
        }
    }
}

impl Router {
    pub fn register(&mut self, route: StaticRoute) -> Result<(), Error> {
        let entry = RouteEntry::new(route.method, route.service);

        self.i.register(route.segments, entry)?;

        Ok(())
    }
}

impl IntoRoutes for Router {
    type Error = Infallible;
    fn into_routes(self) -> Result<Vec<StaticRoute>, Self::Error> {
        let routes = self
            .i
            .into_routes()
            .into_iter()
            .map(|(segments, handlers)| {
                handlers.into_iter().map(move |handler| {
                    StaticRoute::new(handler.method, segments.clone(), handler.handler)
                })
            })
            .flatten()
            .collect();

        Ok(routes)
    }
}

impl IntoService<Request> for Router {
    type Service = RouterService;
    fn into_service(self) -> Self::Service {
        RouterService {
            router: Arc::new(self.i),
        }
    }
}

#[derive(Clone)]
pub struct RouterService {
    router: Arc<router::Router<RouteEntry>>,
}

impl Service<Request> for RouterService {
    type Output = Outcome<Response, Error, Request>;

    type Future = BoxFuture<'static, Self::Output>;

    fn call(&self, mut req: Request) -> Self::Future {
        let router = self.router.clone();

        Box::pin(async move {
            let mut params = Params::default();
            let found = match router.find(req.uri().path(), &mut params) {
                Some(found) => found,
                None => return Outcome::Next(req),
            };

            let method = req.method().clone();

            let is_head = method == Method::HEAD;

            req.extensions_mut().insert(params);

            for next in found
                .iter()
                .filter(|route| route.method == method || (is_head && route.method == Method::GET))
            {
                match next.handler.call(req).await {
                    Outcome::Next(r) => {
                        req = r;
                    }
                    Outcome::Success(mut success) => {
                        if method != next.method && is_head {
                            *success.body_mut() = Body::empty();
                            *success.status_mut() = StatusCode::NO_CONTENT;
                        }

                        return dale::Outcome::Success(success);
                    }
                    o => return o,
                }
            }

            req.extensions_mut().remove::<Params>();

            Outcome::Next(req)
        })
    }
}
