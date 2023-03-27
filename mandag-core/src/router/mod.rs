mod into_routes;
mod route;
mod router;
mod routing;

pub use self::{
    into_routes::*,
    route::*,
    router::{Router, RouterService, SharedRouter},
    routing::*,
};

pub use ::router::{Segment, Segments};
