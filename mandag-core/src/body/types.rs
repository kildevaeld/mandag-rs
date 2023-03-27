use crate::error::BodyError;
use crate::Request;
use async_trait::async_trait;
use dale::IntoOutcome;
pub use hyper::Body;

pub type Outcome<S, E> = dale::Outcome<S, E, Body>;

#[async_trait]
pub trait FromBody: Sized + Send {
    type Error: Into<BodyError>;
    type Output: IntoOutcome<Body, Success = Self, Failure = Self::Error>;

    async fn from_body(req: &Request, body: Body) -> Self::Output;
}
