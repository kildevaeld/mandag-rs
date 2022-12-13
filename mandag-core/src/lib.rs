mod app;
mod error;
mod extension;
mod handler;
mod handler_service;
mod module;
mod plugin;
mod request_ext;
mod store;
mod types;

pub mod body;
pub mod req;
pub mod router;

pub use self::{
    app::App,
    error::*,
    extension::{Extension, ExtensionCtx},
    handler::*,
    module::{Module, ModuleBuildCtx},
    plugin::{Pluggable, Plugin},
    store::Store,
    types::*,
};

pub use dale;

pub use async_trait::async_trait;

pub mod http {
    pub use super::types::{Body, Request, Response};
    pub use dale_http::{
        headers::*, reply, Error as HttpError, HeaderMap, KnownError, Method, Reply, StatusCode,
        Uri,
    };
}

pub mod prelude {
    pub use super::{
        request_ext::RequestExt,
        router::{IntoRoutesExt, Routing, RoutingExt},
        HandlerExt, Pluggable,
    };
    pub use dale::{IntoOutcome, IntoOutcomeExt};
    pub use dale_http::prelude::*;
}
