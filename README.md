# Mandag

Not there yet...

```rust

use mandag::{prelude::*, router::{Route, Params}, reply}

#[mandag::get("/)]
fn index() {
    reply::html("<h1>Hello, World!</h1>)
}

#[mandag::get("/)]
fn about() {
    reply::html("<h1>About!</h1>)
}



#[tokio::main]
async fn main() -> mandag::Result<()> {

    mandag::start()
        .config("./config")
        .attach()
        .build().await?
        .route((index, about))
        .listen(([127,0,0,1], 3000)).await


}

```