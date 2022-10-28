mod into_routes;
mod route;
mod router;

pub use self::{
    into_routes::*,
    route::*,
    router::{Router, RouterService},
};

pub use ::router::{Segment, Segments};
