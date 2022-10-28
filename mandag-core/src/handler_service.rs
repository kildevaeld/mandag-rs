use dale::{boxed::BoxFuture, IntoOutcome, Outcome, Service};
use dale_http::{error::Error, Reply};
use hyper::Body;

use crate::{from_body::FromBody, handler::Handler, FromRequest, Request, Response};

#[derive(Debug, Clone, Copy)]
pub struct HandlerService<H>(pub H);

impl<H> Service<Request> for HandlerService<H>
where
    H: Clone + 'static,
    for<'a> H: Handler<'a>,
    for<'a> <H as Handler<'a>>::Error: Into<Error>,
    for<'a> <<H as Handler<'a>>::Input as FromRequest<'a>>::Error: Into<Error>,
{
    type Output = Outcome<Response, Error, Request>;
    type Future = BoxFuture<'static, Self::Output>;

    fn call(&self, mut req: Request) -> Self::Future {
        let handler = self.0.clone();
        Box::pin(async move {
            let body = std::mem::replace(req.body_mut(), Body::empty());

            let input = {
                let input_req = <H::Input as FromRequest>::from_request(&req)
                    .await
                    .into_outcome();

                let input = match input_req {
                    Outcome::Success(ret) => ret,
                    Outcome::Next(_) => {
                        drop(input_req);
                        return Outcome::Next(req);
                    }
                    Outcome::Failure(err) => return Outcome::Failure(err.into()),
                };
                input
            };

            let data_reg = <H::Data as FromBody>::from_body(body).await;

            let data = match data_reg {
                Ok(ret) => ret,
                Err(err) => panic!(),
            };

            let ret = match handler.handle(input, data).await {
                Ok(ret) => ret,
                Err(err) => return Outcome::Failure(err.into()),
            };

            let response = ret.into_response();

            Outcome::Success(response)
        })
    }
}
