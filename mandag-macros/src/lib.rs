mod handle;
mod route;
// mod segments;
mod shared;
mod utils;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn handler(attr: TokenStream, item: TokenStream) -> TokenStream {
    handle::create(attr, item).into()
}

macro_rules! methods {
    ($($name: ident => $method:ident),*) => {
        $(
            #[proc_macro_attribute]
            pub fn $name(attr: TokenStream, item: TokenStream) -> TokenStream {
                route::create(attr, item, route::Method::$method).into()
            }

        )*
    };
}

methods!(
    get => Get,
    post => Post,
    put => Put,
    patch => Patch,
    delete => Delete,
    head => Head
);
