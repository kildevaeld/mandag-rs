mod app;
mod core;
mod phase;
mod store;
mod types;

mod from_request;

// Maybe move to core?
mod extension;
mod module;

mod request_ext;

pub use mandag_macros::*;

pub use mandag_core::{async_trait, Handler, Plugin, Reply};

pub mod router;

pub use self::{core::Core, extension::*, module::*, router::Route};

pub use dale_http::error::Error;

pub type Outcome = dale::Outcome<mandag_core::Response, Error, mandag_core::Request>;

pub use dale::Service;

pub use dale_http::reply;

pub mod prelude {
    pub use super::{
        request_ext::RequestExt,
        router::{IntoRoutesExt, Routing, RoutingExt},
    };
    pub use dale::{IntoOutcome, IntoOutcomeExt};
    pub use dale_http::prelude::*;
    pub use mandag_core::{HandlerExt, Pluggable};
    pub use mandag_serve::ServiceServeExt;
}

pub mod http {
    pub use dale_http::{
        error::{Error, KnownError},
        headers, HeaderMap, HeaderValue, Method, Reply, StatusCode,
    };
    pub use mandag_core::{Request, Response};
}

pub mod req {
    pub use super::from_request::*;
    pub use mandag_core::Ext;
}
