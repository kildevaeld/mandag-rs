use super::Request;
use crate::app::App;

mod sealed {
    use super::*;
    pub trait Sealed {}

    impl Sealed for Request {}
}

pub trait RequestExt: sealed::Sealed {
    fn app(&self) -> &App;
}

impl RequestExt for Request {
    fn app(&self) -> &App {
        self.extensions().get::<App>().expect("app")
    }
}
