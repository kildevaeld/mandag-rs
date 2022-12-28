use crate::{body::FromBody, handler_service::HandlerService, req::FromRequest, Reply};
use async_trait::async_trait;

#[async_trait]
pub trait Handler: Send + Sync {
    type Input<'r>: FromRequest<'r>
    where
        Self: 'r;
    type Data: FromBody;
    type Output: Reply;
    type Error;

    async fn handle<'r>(
        &self,
        input: Self::Input<'r>,
        data: Self::Data,
    ) -> Result<Self::Output, Self::Error>;
}

pub trait HandlerExt: Handler {
    fn service(self) -> HandlerService<Self>
    where
        Self: Sized,
    {
        HandlerService(self)
    }
}

impl<H> HandlerExt for H where H: Handler {}
