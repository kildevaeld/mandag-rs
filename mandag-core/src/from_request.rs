use std::convert::Infallible;

use async_trait::async_trait;
use dale::{IntoOutcome, IntoOutcomeExt};

use crate::{error::GuardError, Request};

pub type Outcome<T, E> = dale::Outcome<T, E, ()>;

#[async_trait]
pub trait FromRequest<'a>: Sized + Send {
    type Error: Into<GuardError> + Send;
    type Output: IntoOutcome<(), Success = Self, Failure = Self::Error> + Send;
    async fn from_request(req: &'a Request) -> Self::Output;
}

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
