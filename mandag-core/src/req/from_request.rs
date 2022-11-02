use crate::GuardError;
use crate::Request;
use async_trait::async_trait;
use dale::IntoOutcome;

#[async_trait]
pub trait FromRequest<'a>: Sized + Send {
    type Error: Into<GuardError> + Send;
    type Output: IntoOutcome<(), Success = Self, Failure = Self::Error> + Send;
    async fn from_request(req: &'a Request) -> Self::Output;
}
