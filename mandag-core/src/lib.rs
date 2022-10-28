mod error;
mod from_body;
mod from_request;
mod handler;
mod handler_service;
mod types;

pub use self::{error::*, from_body::FromBody, from_request::FromRequest, handler::*, types::*};

pub use async_trait::async_trait;

struct Route {
    method: dale_http::Method,
}

const ROUTE: Route = Route {
    method: dale_http::Method::GET,
};
