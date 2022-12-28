use mandag::{http::Error, prelude::Set, reply, req::AppExt, router::IntoRoutesExt, Route};
use mandag_tera::Tera;

use store::TodoStore;

mod store;

#[mandag::module(path = "/api/todos")]
mod api {

    use super::store::CreateTodo;
    use dale_http::{StatusCode, Uri};
    use mandag::{
        body::{Form, Json},
        http::Error,
        prelude::*,
        reply,
        req::AppExt,
    };
    use mandag_core::Body;

    use crate::store::Todos;

    #[get(path = "/")]
    pub fn list_todos(todos: AppExt<Todos>) {
        reply::json(todos.list())
    }

    // #[post(path = "/", data = "data")]
    // pub fn create_todo(todos: AppExt<Todos>, data: Json<CreateTodo>) {
    //     let todo = todos.insert(data.into_inner());
    //     reply::json(todo)
    // }

    #[post(path = "/", data = "data")]
    pub fn create_todo2(todos: AppExt<Todos>, data: Form<CreateTodo>) {
        let todo = todos.insert(data.into_inner());
        reply::redirect(Uri::from_static("/")).set(StatusCode::TEMPORARY_REDIRECT)
    }
}

#[mandag::any(path = "/")]
fn index(tera: AppExt<Tera>, todos: AppExt<store::Todos>) {
    let mut ctx = mandag_tera::Context::default();

    ctx.insert("todos", &todos.list());

    let html = tera.render("index.html", &ctx).unwrap();

    reply::html(html)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    mandag::Core::default()
        .config_search_path("mandag/examples/todo")
        .attach(TodoStore)
        .attach(mandag_tera::TeraExt)
        .build()
        .await?
        .route(api::Routes)
        .route(index)
        .route(Route::get("/create", Tera::template("form.html")))
        .listen(([127, 0, 0, 1], 3000))
        .await?;

    Ok(())
}
