use mandag_core::{async_trait, FromRequest, Outcome, Request};
use std::convert::Infallible;

use crate::{app::App, prelude::RequestExt};

pub struct AppExt<T>(T);

impl<T> std::ops::Deref for AppExt<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
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
