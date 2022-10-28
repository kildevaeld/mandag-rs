use dale_extensions::State;

use crate::{app::App, router::RouterService};

use super::Phase;

pub struct Start {
    pub service: State<RouterService, App>,
}

impl Phase for Start {}
