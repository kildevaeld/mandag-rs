use super::route::{Route, StaticRoute};
use dale_http::error::Error;
use router::{AsSegments, Segments};

pub trait IntoRoute {
    type Error;
    fn into_route(self) -> Result<StaticRoute, Self::Error>;
}

pub trait IntoRoutes {
    type Error;
    fn into_routes(self) -> Result<Vec<StaticRoute>, Self::Error>;
}

pub trait IntoRoutesExt: IntoRoutes {
    fn mounted_on<S>(self, path: S) -> NamedSpaced<Self>
    where
        Self: Sized,
        S: ToString,
    {
        NamedSpaced {
            path: path.to_string(),
            routes: self,
        }
    }
}

impl<I> IntoRoutesExt for I where I: IntoRoutes {}

pub struct NamedSpaced<I> {
    path: String,
    routes: I,
}

impl<I> IntoRoutes for NamedSpaced<I>
where
    I: IntoRoutes,
{
    type Error = I::Error;

    fn into_routes(self) -> Result<Vec<StaticRoute>, Self::Error> {
        let routes = self.routes.into_routes()?;

        let path: Segments<'_> = self
            .path
            .as_segments()
            .expect("path")
            .collect::<Vec<_>>()
            .into();

        let path = path.to_static();

        let routes = routes
            .into_iter()
            .map(move |route| {
                let Route {
                    segments,
                    method,
                    service,
                    _a,
                } = route;

                let mut path: Vec<_> = path.clone().into();

                path.extend(segments.into_iter());

                let segments: Segments<'static> = path.into();

                Route::new(method, segments, service)
            })
            .collect::<Vec<_>>();

        Ok(routes)
    }
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

pub fn into_mounted_routes_box<R>(path: String, routes: R) -> Box<dyn IntoRoutesBox>
where
    R: IntoRoutes + Send + Sync + 'static,
    R::Error: std::error::Error + Send + Sync,
{
    Box::new(BoxedIntoRoutes(NamedSpaced { path, routes }))
}

macro_rules! into_routes {
    ($one: ident) => {
        impl<$one> IntoRoutes for ($one,)
        where
            $one: IntoRoute,
        {
            type Error = $one::Error;
            fn into_routes(self) -> Result<Vec<StaticRoute>, Self::Error> {
                Ok(vec![self.0.into_route()?])
            }
        }
    };

    ($first: ident $($rest: ident)*) => {
        into_routes!($($rest)*);

        impl<$first: IntoRoute, $($rest: IntoRoute<Error = $first::Error>),*> IntoRoutes for ($first, $($rest),*) {
            type Error = $first::Error;
            #[allow(non_snake_case)]
            fn into_routes(self) -> Result<Vec<StaticRoute>, Self::Error> {
                let ($first, $($rest),*) = self;
                Ok(vec![
                    $first.into_route()?,
                    $($rest.into_route()?),*
                ])
            }
        }
    };
}

into_routes!(T1 T2 T3 T4 T5 T6 T7 T8);
