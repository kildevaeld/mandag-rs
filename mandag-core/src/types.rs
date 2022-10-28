pub use hyper::Body;

pub type Request = dale_http::Request<Body>;

pub type Response = dale_http::Response<Body>;

pub trait Reply: dale_http::Reply<Body> {}

impl<R> Reply for R where R: dale_http::Reply<Body> {}
