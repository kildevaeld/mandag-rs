use dale::{boxed::BoxFuture, IntoOutcome, Outcome, Service};
use dale_http::{error::Error, Reply};
use hyper::Body;

use crate::{body::FromBody, handler::Handler, req::FromRequest, Request, Response};

#[derive(Debug, Clone, Copy)]
pub struct HandlerService<H>(pub H);

impl<H> Service<Request> for HandlerService<H>
where
    H: Clone + 'static,
    H: Handler,
    <H as Handler>::Error: Into<Error>,
    for<'a> <<H as Handler>::Input<'a> as FromRequest<'a>>::Error: Into<Error>,
    <<H as Handler>::Data as FromBody>::Error: Into<Error>,
{
    type Output = Outcome<Response, Error, Request>;
    type Future = BoxFuture<'static, Self::Output>;

    fn call(&self, mut req: Request) -> Self::Future {
        let handler = self.0.clone();
        Box::pin(async move {
            let body = std::mem::replace(req.body_mut(), Body::empty());

            let input = {
                let input_req = <H::Input<'_> as FromRequest>::from_request(&req)
                    .await
                    .into_outcome();

                let input = match input_req {
                    Outcome::Success(ret) => ret,
                    Outcome::Next(_) => {
                        drop(input_req);
                        *req.body_mut() = body;
                        return Outcome::Next(req);
                    }
                    Outcome::Failure(err) => return Outcome::Failure(err.into()),
                };
                input
            };

            let data_reg = <H::Data as FromBody>::from_body(body).await;

            let data = match data_reg {
                Ok(ret) => ret,
                Err(err) => return Outcome::Failure(err.into()),
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
