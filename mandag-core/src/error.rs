use dale_http::error::Error as HttpError;
use std::convert::Infallible;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {}

#[derive(Debug, ThisError)]
pub enum GuardError {
    #[error("infallible")]
    Infallible,
}

impl From<Infallible> for GuardError {
    fn from(_: Infallible) -> Self {
        GuardError::Infallible
    }
}

#[derive(Debug, ThisError)]
pub enum BodyError {}

impl From<GuardError> for HttpError {
    fn from(err: GuardError) -> Self {
        HttpError::new(err)
    }
}
