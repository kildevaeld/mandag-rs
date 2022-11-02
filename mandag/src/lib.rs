mod app;
mod core;
mod phase;
mod store;
mod types;

mod from_request;

mod request_ext;

pub use mandag_macros::*;

pub use mandag_core::{
    async_trait, router::Route, Extension, ExtensionCtx, Handler, Module, ModuleBuildCtx, Plugin,
    Reply,
};

pub use mandag_core::router;

pub use self::core::Core;

pub use dale_http::error::Error;

pub type Outcome = dale::Outcome<mandag_core::Response, Error, mandag_core::Request>;

pub use dale::Service;

pub use dale_http::reply;

pub mod prelude {
    pub use super::request_ext::RequestExt;

    pub use mandag_core::prelude::*;
    pub use mandag_serve::ServiceServeExt;
}

pub mod http {
    pub use mandag_core::http::*;
}

pub mod req {
    pub use super::from_request::*;
    pub use mandag_core::req::*;
}

pub mod body {
    pub use mandag_core::body::*;
}
