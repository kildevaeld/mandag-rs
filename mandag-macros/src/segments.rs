use proc_macro::TokenStream;
use router;
use syn::parse_macro_input;

use crate::utils::{crate_ident_name, parse_route};

pub fn create(item: TokenStream) -> TokenStream {
    let crate_name = crate_ident_name("mandag");

    let lit = parse_macro_input!(item as syn::LitStr);

    let value = lit.value();

    let segments = router::parse(&value).expect("parse route");

    let len = segments.len();

    let segments = parse_route(&crate_name, segments);

    quote::quote!(
        {
            let segments: [#crate_name::router::Segment<'static>; #len] = [#(#segments),*];
            segments
        }
    )
    .into()
}
