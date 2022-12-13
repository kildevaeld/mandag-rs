use super::FromRequest;
use crate::{app::App, error::GuardError, request_ext::RequestExt, Request};
use async_trait::async_trait;
use dale::{IntoOutcome, IntoOutcomeExt};
use dale_http::headers::HeaderMapExt;
use dale_http::headers::{
    authorization::{Basic, Bearer},
    Authorization, ContentLength, ContentType,
};
use std::convert::Infallible;

pub type Outcome<T, E> = dale::Outcome<T, E, ()>;

#[async_trait]
impl<'a> FromRequest<'a> for () {
    type Error = Infallible;
    type Output = Outcome<Self, Self::Error>;

    async fn from_request(_req: &'a Request) -> Self::Output {
        Outcome::Success(())
    }
}

#[async_trait]
impl<'a> FromRequest<'a> for &'a Request {
    type Error = Infallible;
    type Output = Outcome<Self, Self::Error>;

    async fn from_request(req: &'a Request) -> Self::Output {
        Outcome::Success(req)
    }
}

#[async_trait]
impl<'a> FromRequest<'a> for &'a dale_http::Uri {
    type Error = Infallible;
    type Output = Outcome<Self, Self::Error>;

    async fn from_request(req: &'a Request) -> Self::Output {
        Outcome::Success(req.uri())
    }
}

#[async_trait]
impl<'a> FromRequest<'a> for &'a dale_http::Method {
    type Error = Infallible;
    type Output = Outcome<Self, Self::Error>;

    async fn from_request(req: &'a Request) -> Self::Output {
        Outcome::Success(req.method())
    }
}

#[async_trait]
impl<'a, T> FromRequest<'a> for Option<T>
where
    T: FromRequest<'a>,
{
    type Error = T::Error;
    type Output = Outcome<Self, Self::Error>;

    async fn from_request(req: &'a Request) -> Self::Output {
        match T::from_request(req).await.into_outcome() {
            Outcome::Success(ret) => Outcome::Success(Some(ret)),
            Outcome::Next(_) => Outcome::Success(None),
            Outcome::Failure(err) => Outcome::Failure(err),
        }
    }
}

#[async_trait]
impl<'a, T> FromRequest<'a> for Result<T, T::Error>
where
    T: FromRequest<'a>,
{
    type Error = T::Error;
    type Output = Outcome<Self, Self::Error>;

    async fn from_request(req: &'a Request) -> Self::Output {
        match T::from_request(req).await.into_outcome() {
            Outcome::Success(ret) => Outcome::Success(Ok(ret)),
            Outcome::Next(_) => Outcome::Next(()),
            Outcome::Failure(err) => Outcome::Success(Err(err)),
        }
    }
}

pub struct Ext<'a, T>(&'a T);

impl<'a, T> std::ops::Deref for Ext<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

#[async_trait]
impl<'a, T> FromRequest<'a> for Ext<'a, T>
where
    T: Send + Sync + 'static,
{
    type Error = Infallible;
    type Output = Outcome<Self, Self::Error>;
    async fn from_request(req: &'a Request) -> Self::Output {
        match req.extensions().get() {
            Some(found) => Outcome::Success(Ext(found)),
            None => dale::Outcome::Next(()),
        }
    }
}

macro_rules! from_req {
    ($type: ident) => {
        #[async_trait]
        impl<'a, $type: FromRequest<'a>> FromRequest<'a> for ($type,) {
            type Error = $type::Error;
            type Output = Outcome<($type, ), Self::Error>;
            async fn from_request(req: &'a Request) -> Self::Output {
                $type::from_request(req).await.map(|ret| (ret,)).into_outcome()
            }
        }
    };
    ($first: ident $($rest: ident)*) => {
        from_req!($($rest)*);

        #[async_trait]
        impl<'a, $first: FromRequest<'a>, $($rest: FromRequest<'a>),*> FromRequest<'a> for ($first,$($rest),*) {
            type Error = GuardError;
            type Output = Outcome<($first, $($rest),*), Self::Error>;
            async fn from_request(req: &'a Request) -> Self::Output {
                let out = (
                    from_req!(@item $first, req),
                    $(
                        from_req!(@item $rest, req),
                    )*
                );

                Outcome::Success(out)
            }
        }

    };

    (@item $func: ident, $req: expr) => {
        match $func::from_request($req).await.into_outcome() {
            Outcome::Success(ret) => ret,
            Outcome::Next(next) => return Outcome::Next(next),
            Outcome::Failure(err) => return Outcome::Failure(err.into())
        }
    }
}

from_req!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16);

macro_rules! headers {
    ($($type: ty)*) => {
        $(
        #[async_trait]
        impl<'a> FromRequest<'a> for $type {
            type Error = Infallible;
            type Output = Outcome<$type, Self::Error>;
            async fn from_request(req: &'a Request) -> Self::Output {
               match req.headers().typed_get::<$type>() {
                Some(header) => Outcome::Success(header),
                None => Outcome::Next(())
               }
            }
        }
        )*
    };
}

headers!(ContentType ContentLength Authorization<Bearer> Authorization<Basic>);

pub struct AppExt<T>(T);

impl<T> std::ops::Deref for AppExt<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for AppExt<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[async_trait]
impl<'a, T> FromRequest<'a> for AppExt<T>
where
    T: Clone + Send + Sync + 'static,
{
    type Error = Infallible;

    type Output = Outcome<Self, Self::Error>;

    async fn from_request(req: &'a Request) -> Self::Output {
        match req.app().store().get::<T>() {
            Some(ret) => Outcome::Success(AppExt(ret)),
            None => Outcome::Next(()),
        }
    }
}

#[async_trait]
impl<'a> FromRequest<'a> for &'a App {
    type Error = Infallible;

    type Output = Outcome<Self, Self::Error>;

    async fn from_request(req: &'a Request) -> Self::Output {
        Outcome::Success(req.app())
    }
}
