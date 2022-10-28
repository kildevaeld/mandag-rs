use dale::Service;

pub trait IntoService<R> {
    type Service: Service<R>;
    fn into_service(self) -> Self::Service;
}
