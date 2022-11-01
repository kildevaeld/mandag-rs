mod into_routes;
mod route;
mod router;
mod routing;

pub use self::{
    into_routes::*,
    route::*,
    router::{Router, RouterService},
    routing::*,
};

pub use ::router::{Segment, Segments};
