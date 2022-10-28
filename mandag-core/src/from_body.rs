use std::convert::Infallible;

use async_trait::async_trait;
use hyper::Body;

#[async_trait]
pub trait FromBody: Sized + Send {
    type Error: Send;

    async fn from_body(body: Body) -> Result<Self, Self::Error>;
}

#[async_trait]
impl FromBody for Body {
    type Error = Infallible;

    async fn from_body(body: Body) -> Result<Self, Self::Error> {
        Ok(body)
    }
}

#[async_trait]
impl FromBody for () {
    type Error = Infallible;

    async fn from_body(_body: Body) -> Result<Self, Self::Error> {
        Ok(())
    }
}
