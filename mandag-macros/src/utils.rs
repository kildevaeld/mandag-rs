use darling::ToTokens;
use proc_macro2::{Ident, Span};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use router::{AsSegments, Segment};
use syn::{FnArg, ItemFn, Pat, PatType};

pub fn crate_ident_name(name: &str) -> syn::Ident {
    let found_crate = crate_name(name).expect(&format!("{} is present in `Cargo.toml`", name));

    let name = name.replace("-", "_");

    match found_crate {
        FoundCrate::Itself => syn::Ident::new(&name, Span::call_site()),
        FoundCrate::Name(name) => {
            let ident = syn::Ident::new(&name, Span::call_site());
            ident
        }
    }
}

pub fn parse_parameters<'a>(item: &'a ItemFn) -> impl Iterator<Item = &'a PatType> {
    item.sig.inputs.iter().filter_map(|item| match item {
        FnArg::Receiver(_) => None,
        FnArg::Typed(item) => Some(item),
    })
}

pub fn get_name(pat: &syn::Pat) -> String {
    match pat {
        Pat::Ident(ident) => ident.ident.to_string(),
        _ => {
            panic!("not a ident")
        }
    }
}

// pub fn parse_fn_inputs<'a>(
//     item: &'a ItemFn,
//     data: Option<&'a str>,
// ) -> impl Iterator<Item = impl ToTokens + 'a> + 'a {
//     item.sig
//         .inputs
//         .iter()
//         .filter_map(|item| match item {
//             FnArg::Receiver(_) => None,
//             FnArg::Typed(item) => Some(item),
//         })
//         .filter_map(move |item| {
//             //
//             match &data {
//                 Some(data) => {
//                     if data != &get_name(&item.pat) {
//                         Some(item)
//                     } else {
//                         None
//                     }
//                 }
//                 None => Some(item),
//             }
//         })
//         .map(|item| {
//             let ty = &item.ty;
//             let ret = match &**ty {
//                 Type::Reference(refe) => {
//                     let ty = &refe.elem;
//                     quote! {
//                         &'r #ty
//                     }
//                 }
//                 _ => quote!(
//                     #ty
//                 ),
//             };

//             ret
//         })
// }

pub fn parse_route<'a, S: AsSegments<'a>>(
    crate_name: &'a Ident,
    segments: S,
) -> impl Iterator<Item = impl ToTokens + 'a>
where
    S::Error: std::fmt::Debug,
{
    let segments = segments.as_segments().expect("segments"); //router::parse(path).unwrap();

    segments.map(move |segment| match segment {
        Segment::Constant(path) => {
            quote!(
                #crate_name::router::Segment::Constant(std::borrow::Cow::Borrowed(#path))
            )
        }
        Segment::Parameter(param) => {
            quote!(
                #crate_name::router::Segment::Parameter(std::borrow::Cow::Borrowed(#param))
            )
        }
        Segment::Star(name) => {
            quote!(
                #crate_name::router::Segment::Star(std::borrow::Cow::Borrowed(#name))
            )
        }
    })
}
