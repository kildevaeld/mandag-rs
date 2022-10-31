use dale_http::error::Error;

use super::route::StaticRoute;

pub trait IntoRoute {
    type Error;
    fn into_route(self) -> Result<StaticRoute, Self::Error>;
}

pub trait IntoRoutes {
    type Error;
    fn into_routes(self) -> Result<Vec<StaticRoute>, Self::Error>;
}

pub trait IntoRoutesBox: Send + Sync {
    fn into_routes(self: Box<Self>) -> Result<Vec<StaticRoute>, Error>;
}

struct BoxedIntoRoutes<I>(I);

impl<I> IntoRoutesBox for BoxedIntoRoutes<I>
where
    I: IntoRoutes + Send + Sync,
    I::Error: std::error::Error + Send + Sync + 'static,
{
    fn into_routes(self: Box<Self>) -> Result<Vec<StaticRoute>, Error> {
        self.0.into_routes().map_err(|err| Error::new(err))
    }
}

pub fn into_routes_box<R>(routes: R) -> Box<dyn IntoRoutesBox>
where
    R: IntoRoutes + Send + Sync + 'static,
    R::Error: std::error::Error + Send + Sync,
{
    Box::new(BoxedIntoRoutes(routes))
}
