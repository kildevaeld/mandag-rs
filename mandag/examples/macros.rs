#[mandag::module]
mod api {

    #[get(path = "/")]
    pub fn index() {
        "Hello, World!"
    }

    #[get(path = "/")]
    pub fn indexes() {
        "Hello, World!"
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    mandag::Core::default()
        .build()
        .await?
        .route(api::Routes)
        .listen(([127, 0, 0, 1], 3000))
        .await?;

    Ok(())
}
