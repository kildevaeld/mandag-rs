use crate::Request;
use dale_extensions::{Extensible, Extensions};
use std::any::Any;

pub trait Plugin<T: ?Sized>: Sized {
    type Error;

    fn eval(extensible: &mut T) -> Result<Self, Self::Error>;
}

pub trait Pluggable {
    /// Return a copy of the plugin's produced value.
    ///
    /// The plugin will be created if it doesn't exist already.
    /// If plugin creation fails, an error is returned.
    ///
    /// `P` is the plugin type.
    fn get<P: Plugin<Self>>(&mut self) -> Result<P, P::Error>
    where
        P: Send + Sync + Clone + Any,
        Self: Extensible,
    {
        self.get_ref::<P>().map(|v| v.clone())
    }

    /// Return a reference to the plugin's produced value.
    ///
    /// The plugin will be created if it doesn't exist already.
    /// If plugin creation fails an error is returned.
    ///
    /// `P` is the plugin type.
    fn get_ref<P: Plugin<Self>>(&mut self) -> Result<&P, P::Error>
    where
        P: Send + Sync + Any,
        Self: Extensible,
    {
        self.get_mut::<P>().map(|mutref| &*mutref)
    }

    /// Return a mutable reference to the plugin's produced value.
    ///
    /// The plugin will be created if it doesn't exist already.
    /// If plugin creation fail an error is returned.
    ///
    /// `P` is the plugin type.
    fn get_mut<P: Plugin<Self>>(&mut self) -> Result<&mut P, P::Error>
    where
        P: Sync + Send + Any,
        Self: Extensible,
    {
        if self.extensions().contains::<P>() {
            return Ok(self.extensions_mut().get_mut::<P>().unwrap());
        }

        match P::eval(self) {
            Ok(ret) => {
                self.extensions_mut().insert(ret);
                Ok(self.extensions_mut().get_mut().unwrap())
            }
            Err(err) => Err(err),
        }
    }

    /// Create and evaluate a once-off instance of a plugin.
    fn compute<P: Plugin<Self>>(&mut self) -> Result<P, P::Error> {
        <P as Plugin<Self>>::eval(self)
    }
}

impl Pluggable for Request {}
