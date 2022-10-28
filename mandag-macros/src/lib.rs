mod handle;
mod route;
mod segments;
mod shared;
mod utils;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn handler(attr: TokenStream, item: TokenStream) -> TokenStream {
    handle::create(attr, item).into()
}

#[proc_macro_attribute]
pub fn get(attr: TokenStream, item: TokenStream) -> TokenStream {
    route::create(attr, item, route::Method::Get).into()
}

// #[proc_macro]
// pub fn segments(item: TokenStream) -> TokenStream {
//     segments::segments(item)
// }
