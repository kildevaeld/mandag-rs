use async_trait::async_trait;
pub use hyper::Body;

#[async_trait]
pub trait FromBody: Sized + Send {
    type Error: Send;

    async fn from_body(body: Body) -> Result<Self, Self::Error>;
}
