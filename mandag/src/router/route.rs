use dale::{BoxService, IntoOutcome, Service, ServiceExt};
use dale_http::{error::Error, Method, Reply as _};
use mandag_core::{Reply, Request, Response};
use router::{AsSegments, Segments};
use std::marker::PhantomData;

use super::into_routes::{IntoRoute, IntoRoutes};

pub type StaticRoute =
    Route<'static, Segments<'static>, BoxService<'static, Request, Response, Error>>;

pub struct Route<'a, P, S> {
    pub method: Method,
    pub service: S,
    pub segments: P,
    _a: PhantomData<&'a fn()>,
}

impl<'a, P, S> Route<'a, P, S> {
    pub const fn new(method: Method, segments: P, service: S) -> Route<'a, P, S> {
        Route {
            method,
            service,
            segments,
            _a: PhantomData,
        }
    }

    pub const fn get(path: P, service: S) -> Route<'a, P, S> {
        Route::new(Method::GET, path, service)
    }

    pub const fn post(path: P, service: S) -> Route<'a, P, S> {
        Route::new(Method::POST, path, service)
    }
}

impl<'a, P, S> Route<'a, P, S>
where
    P: AsSegments<'a> + 'a,
    <P as AsSegments<'a>>::Error: std::error::Error + Send + Sync + 'static,
    S: Service<Request> + Send + Sync + 'static,
    <S::Output as IntoOutcome<Request>>::Success: Reply + Send,
    <S::Output as IntoOutcome<Request>>::Failure: Into<Error>,
    S::Future: Send + 'static,
{
    pub fn to_static(self) -> Result<StaticRoute, Error> {
        let segments = self
            .segments
            .as_segments()
            .map_err(Error::new)?
            .collect::<Vec<_>>();

        let service = self
            .service
            .then(
                |ret: <S::Output as IntoOutcome<Request>>::Success| async move {
                    Result::<_, Error>::Ok(ret.into_response())
                },
            )
            .err_into()
            .boxed();

        Ok(StaticRoute::new(
            self.method,
            Segments::new(segments).to_static(),
            service,
        ))
    }
}

impl<'a, P, S> IntoRoute for Route<'a, P, S>
where
    P: AsSegments<'a> + 'a,
    <P as AsSegments<'a>>::Error: std::error::Error + Send + Sync + 'static,
    S: Service<Request> + Send + Sync + 'static,
    <S::Output as IntoOutcome<Request>>::Success: Reply + Send,
    <S::Output as IntoOutcome<Request>>::Failure: Into<Error>,
    S::Future: Send + 'static,
{
    type Error = Error;

    fn into_route(self) -> Result<StaticRoute, Self::Error> {
        let segments = self
            .segments
            .as_segments()
            .map_err(Error::new)?
            .collect::<Vec<_>>();

        Ok(Route {
            method: self.method,
            service: self
                .service
                .then(
                    |ret: <S::Output as IntoOutcome<Request>>::Success| async move {
                        Result::<_, Error>::Ok(ret.into_response())
                    },
                )
                .err_into()
                .boxed(),
            segments: Segments::new(segments).to_static(),
            _a: PhantomData,
        })
    }
}

impl<'a, P, S> IntoRoutes for Route<'a, P, S>
where
    P: AsSegments<'a> + 'a,
    <P as AsSegments<'a>>::Error: std::error::Error + Send + Sync + 'static,
    S: Service<Request> + Send + Sync + 'static,
    <S::Output as IntoOutcome<Request>>::Success: Reply + Send,
    <S::Output as IntoOutcome<Request>>::Failure: Into<Error>,
    S::Future: Send + 'static,
{
    type Error = Error;

    fn into_routes(self) -> Result<Vec<StaticRoute>, Self::Error> {
        let route = self.into_route()?;
        Ok(vec![route])
    }
}
