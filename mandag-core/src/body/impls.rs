use super::{Body, FromBody, Outcome};
use crate::Request;
use async_trait::async_trait;
use dale::IntoOutcome;
use dale_http::{
    headers::{ContentType, HeaderMapExt},
    mime::Mime,
    prelude::BodyExt,
    Bytes,
};
use std::convert::Infallible;

#[async_trait]
impl FromBody for Body {
    type Error = Infallible;
    type Output = Result<Self, Self::Error>;
    async fn from_body(_req: &Request, body: Body) -> Result<Self, Self::Error> {
        Ok(body)
    }
}

#[async_trait]
impl FromBody for Bytes {
    type Error = hyper::Error;
    type Output = Result<Self, Self::Error>;
    async fn from_body(_req: &Request, body: Body) -> Result<Self, Self::Error> {
        body.bytes().await
    }
}

#[async_trait]
impl FromBody for () {
    type Error = Infallible;
    type Output = Result<Self, Self::Error>;
    async fn from_body(_req: &Request, _body: Body) -> Result<Self, Self::Error> {
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
    type Error = dale_http::encoder::DecodeError;
    type Output = Outcome<Self, Self::Error>;

    async fn from_body(req: &Request, body: Body) -> Self::Output {
        let Some(content_type) = req.headers().typed_get::<ContentType>() else {
            return Outcome::Next(body);
        };

        let content_type: Mime = content_type.into();

        if content_type != mime::APPLICATION_JSON {
            return Outcome::Next(body);
        }

        body.json::<T>().await.into_outcome().map(Json)
    }
}

pub struct Form<T>(T);

impl<T> Form<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> std::ops::Deref for Form<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for Form<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[async_trait]
impl<T> FromBody for Form<T>
where
    T: Send,
    for<'a> T: serde::de::Deserialize<'a>,
{
    type Error = dale_http::encoder::DecodeError;

    type Output = Outcome<Self, Self::Error>;

    async fn from_body(req: &Request, body: Body) -> Self::Output {
        let Some(content_type) = req.headers().typed_get::<ContentType>() else {
            return Outcome::Next(body);
        };

        let content_type: Mime = content_type.into();

        if content_type != mime::APPLICATION_WWW_FORM_URLENCODED {
            return Outcome::Next(body);
        }

        body.form::<T>().await.into_outcome().map(Form)
    }
}
