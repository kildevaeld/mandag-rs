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

#[derive(Debug)]
pub struct ExtensionError {
    inner: Box<dyn std::error::Error + Send + Sync>,
}

impl std::fmt::Display for ExtensionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for ExtensionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.inner.source()
    }
}

#[derive(Debug, ThisError)]
pub enum BodyError {
    #[error("infallible")]
    Infallible,
    #[error("http error: {0}")]
    Hyper(#[from] hyper::Error),
    #[error("dale error: {0}")]
    Dale(#[from] dale_http::encoder::DecodeError),
}

impl From<Infallible> for BodyError {
    fn from(_: Infallible) -> Self {
        BodyError::Infallible
    }
}

impl From<GuardError> for HttpError {
    fn from(err: GuardError) -> Self {
        HttpError::new(err)
    }
}
