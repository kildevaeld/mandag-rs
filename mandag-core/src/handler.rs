use crate::{body::FromBody, handler_service::HandlerService, req::FromRequest, Reply, Response};
use async_trait::async_trait;
use dale_http::Error;

#[async_trait]
pub trait Handler<'r>: Send + Sync {
    type Input: FromRequest<'r>;
    type Data: FromBody;
    type Output: Reply;
    type Error;

    async fn handle(
        &'r self,
        input: Self::Input,
        data: Self::Data,
    ) -> Result<Self::Output, Self::Error>;
}

pub trait HandlerExt<'a>: Handler<'a> {
    fn service(self) -> HandlerService<Self>
    where
        Self: Sized,
    {
        HandlerService(self)
    }
}

impl<'a, H> HandlerExt<'a> for H where H: Handler<'a> {}
