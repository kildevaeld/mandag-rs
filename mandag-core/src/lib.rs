mod error;
mod extension;
mod handler;
mod handler_service;
mod module;
mod types;

pub mod router;

mod plugin;

pub mod body;
pub mod req;

pub use self::{
    error::*,
    extension::{Extension, ExtensionCtx},
    handler::*,
    module::{Module, ModuleBuildCtx},
    plugin::{Pluggable, Plugin},
    types::*,
};

pub use async_trait::async_trait;

pub mod http {
    pub use super::types::{Body, Request, Response};
    pub use dale_http::{headers::*, Error, HeaderMap, KnownError, Method, Reply, StatusCode, Uri};
}

pub mod prelude {
    pub use super::{
        router::{IntoRoutesExt, Routing, RoutingExt},
        HandlerExt, Pluggable,
    };
    pub use dale::{IntoOutcome, IntoOutcomeExt};
    pub use dale_http::prelude::*;
}
