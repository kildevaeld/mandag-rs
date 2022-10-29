use proc_macro::TokenStream;
use router;
use syn::parse_macro_input;

pub fn segments(item: TokenStream) -> TokenStream {
    let lit = parse_macro_input!(item as syn::LitStr);

    let value = lit.value();

    let segments = router::parse(&value).expect("parse route");

    let len = segments.len();

    let segments = route::parse_route(segments);

    quote::quote!(
        {
            let segments: [daisy::router::Segment<'static>; #len] = [#(#segments),*];
            segments
        }
    )
    .into()
}
