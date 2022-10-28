use std::sync::Arc;

use dale::{boxed::BoxFuture, Outcome, Service};
use dale_http::error::Error;
use mandag_core::{Request, Response};

struct CoreServiceInner {}

#[derive(Clone)]
pub struct CoreService(Arc<CoreServiceInner>);

impl Service<Request> for CoreService {
    type Output = Outcome<Response, Error, Request>;

    type Future = BoxFuture<'static, Self::Output>;

    fn call(&self, _req: Request) -> Self::Future {
        Box::pin(async move {
            //
            Outcome::Success(Response::default())
        })
    }
}
