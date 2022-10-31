use dale_http::error::Error;
use mandag_core::{async_trait, Handler, HandlerExt};
use mandag_serve::ServiceServeExt;

#[derive(Clone)]
struct Home;

#[async_trait]
impl<'r> Handler<'r> for Home {
    type Input = ();
    type Data = ();
    type Error = Error;
    type Output = &'static str;

    async fn handle(&'r self, _req: (), _data: ()) -> Result<Self::Output, Self::Error> {
        Ok("Hello, World")
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    Home.service().listen(([127, 0, 0, 1], 3000)).await.unwrap();
}
