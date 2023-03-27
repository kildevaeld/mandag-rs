use crate::{router::Routing, ExtensionError};
use async_trait::async_trait;
use johnfig::Config;

pub trait ExtensionCtx: Routing + Send + Sync {
    fn config(&self) -> &Config;

    fn register<S>(&mut self, i: S)
    where
        S: Send + Sync + Clone + 'static;
}

pub trait ExtensionConfig: serde::de::DeserializeOwned {
    const KEY: &'static str;
}

#[async_trait]
pub trait Extension<C>: Send
where
    C: ExtensionCtx,
{
    const NAME: &'static str;

    type Config: ExtensionConfig;
    type Error;

    async fn init(&self, ctx: &mut C, config: Self::Config) -> Result<(), Self::Error>;
}

#[async_trait]
pub trait DynamicExtension<C>
where
    C: ExtensionCtx,
{
    async fn init(&self, ctx: &mut C) -> Result<(), ExtensionError>;
}

struct DynamicExtensionImpl<T> {
    inner: T,
}

#[async_trait]
impl<T, C> DynamicExtension<C> for DynamicExtensionImpl<T>
where
    T: Extension<C> + Send + Sync,
    C: ExtensionCtx,
{
    async fn init(&self, ctx: &mut C) -> Result<(), ExtensionError> {
        let config = ctx
            .config()
            .try_get::<T::Config>(<T::Config as ExtensionConfig>::KEY);
        Ok(())
    }
}
