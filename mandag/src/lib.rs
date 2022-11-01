mod app;
mod core;
mod core_service;
mod phase;
mod store;
mod types;

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

pub mod prelude {
    pub use super::{request_ext::RequestExt, router::IntoRoutesExt};
    pub use dale::{IntoOutcome, IntoOutcomeExt};
    pub use dale_http::prelude::*;
    pub use mandag_core::{HandlerExt, Pluggable};
    pub use mandag_serve::ServiceServeExt;
}

pub mod http {
    pub use dale_http::{
        error::{Error, KnownError},
        Method, Reply, StatusCode,
    };
    pub use mandag_core::{Request, Response};
}
