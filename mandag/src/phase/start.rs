use dale_extensions::State;
use mandag_core::Request;

use crate::{app::App, router::RouterService, types::IntoService};

use super::Phase;

pub struct Start {
    pub service: State<RouterService, App>,
}

impl Phase for Start {}

impl IntoService<Request> for Start {
    type Service = State<RouterService, App>;

    fn into_service(self) -> Self::Service {
        self.service
    }
}
