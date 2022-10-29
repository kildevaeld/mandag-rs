#[macro_use]
mod macros;

use std::{
    future::{Future, IntoFuture},
    net::SocketAddr,
    pin::Pin,
    task::{Context, Poll},
};

use dale::{IntoOutcome, Service};
use dale_http::{hyper::MakeTaskHyperService, Reply};
use hyper::{
    server::{conn::AddrIncoming, Builder},
    Server,
};
use pin_project_lite::pin_project;

type Request = dale_http::Request<hyper::Body>;

pub trait ServiceServeExt: Service<Request> {
    fn listen<A>(self, addr: A) -> Serve<Self>
    where
        Self: Sized + Send + Clone + 'static,
        <Self::Output as IntoOutcome<Request>>::Failure: std::error::Error + Send + Sync,
        <Self::Output as IntoOutcome<Request>>::Success: Reply<hyper::Body>,
        Self::Future: Send,
        A: Into<SocketAddr>,
    {
        Serve {
            service: self,
            builder: Server::bind(&addr.into()),
        }
    }
}

impl<S> ServiceServeExt for S where S: Service<Request> {}

pub struct Serve<T> {
    service: T,
    builder: Builder<AddrIncoming>,
}

impl<T> IntoFuture for Serve<T>
where
    T: Service<Request> + Send + Clone + 'static,
    <T::Output as IntoOutcome<Request>>::Failure: std::error::Error + Send + Sync,
    <T::Output as IntoOutcome<Request>>::Success: Reply<hyper::Body>,
    T::Future: Send,
{
    type Output = hyper::Result<()>;

    type IntoFuture = ServeFuture<T>;

    fn into_future(self) -> Self::IntoFuture {
        let service = dale_http::hyper::make(self.service);
        let server = self.builder.serve(service);
        ServeFuture { server }
    }
}

pin_project! {
    pub struct ServeFuture<T> {
        #[pin]
        server: Server<AddrIncoming, MakeTaskHyperService<T>>
    }
}

impl<T> Future for ServeFuture<T>
where
    T: Service<Request> + Send + Clone + 'static,
    <T::Output as IntoOutcome<Request>>::Failure: std::error::Error + Send + Sync,
    <T::Output as IntoOutcome<Request>>::Success: Reply<hyper::Body>,
    T::Future: Send,
    // <T::Output as IntoOutcome<Request>>::Failure: std::error::Error + Send + Sync + 'static,
{
    type Output = hyper::Result<()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        match ready!(this.server.poll(cx)) {
            Ok(_) => Poll::Ready(Ok(())),
            Err(err) => Poll::Ready(Err(err)),
        }
    }
}
