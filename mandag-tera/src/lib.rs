use std::{future::Future, sync::Arc};

use mandag_core::{
    async_trait,
    dale::Service,
    http::{reply, HttpError},
    prelude::*,
    Extension, ExtensionCtx, Request,
};
use relative_path::RelativePathBuf;

pub use tera::Context;

#[derive(Debug, Clone)]
pub struct Tera(Arc<tera::Tera>);

impl std::ops::Deref for Tera {
    type Target = tera::Tera;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

pub struct TeraExt;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct TeraConfig {
    path: RelativePathBuf,
}

#[async_trait]
impl<C> Extension<C> for TeraExt
where
    C: ExtensionCtx,
{
    async fn build(&self, ctx: &mut C) -> Result<(), HttpError> {
        let cfg: TeraConfig = ctx.config().try_get("templates").expect("tera config");

        let path = cfg.path.join("**/*").to_string();

        let tera = tera::Tera::new(&path).map_err(HttpError::new)?;

        ctx.register(Tera(Arc::new(tera)));

        Ok(())
    }
}

impl Tera {
    pub fn template(
        path: &str,
    ) -> impl Service<Request, Output = reply::Html<String>, Future = impl Future + Send> {
        let path = path.to_string();
        move |req: Request| {
            let path = path.clone();
            async move {
                let tera: Tera = req.app().store().get().expect("fetching Tera from store");

                let ctx = Context::default();

                let template = tera.render(&path, &ctx).map_err(HttpError::new).unwrap();

                reply::html(template)
            }
        }
    }
}
