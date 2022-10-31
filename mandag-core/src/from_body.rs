use std::convert::Infallible;

use async_trait::async_trait;
use dale_http::{prelude::BodyExt, Bytes};
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
impl FromBody for Bytes {
    type Error = hyper::Error;

    async fn from_body(body: Body) -> Result<Self, Self::Error> {
        body.bytes().await
    }
}

#[async_trait]
impl FromBody for () {
    type Error = Infallible;

    async fn from_body(_body: Body) -> Result<Self, Self::Error> {
        Ok(())
    }
}

pub struct Json<T>(T);

impl<T> Json<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> std::ops::Deref for Json<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for Json<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[async_trait]
impl<T> FromBody for Json<T>
where
    T: Send,
    for<'a> T: serde::de::Deserialize<'a>,
{
    type Error = dale_http::Error;

    async fn from_body(body: Body) -> Result<Self, Self::Error> {
        let resp = body.json::<T>().await?;
        Ok(Json(resp))
    }
}
