use crate::types::IntoService;
use dale::{combinators::shared::SharedService, BoxService};
use dale_http::Error;
use mandag_core::{Request, Response};

use super::Phase;

pub struct Start {
    pub service: SharedService<BoxService<'static, Request, Response, Error>>,
}

impl Phase for Start {}

impl IntoService<Request> for Start {
    type Service = SharedService<BoxService<'static, Request, Response, Error>>;

    fn into_service(self) -> Self::Service {
        self.service
    }
}
