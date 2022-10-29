use super::shared::RouteArgs;
use crate::utils::{crate_ident_name, parse_route};
use darling::{FromMeta, ToTokens};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn};

pub enum Method {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    // Options,
    Head,
}

impl Method {
    fn to_tokens(self, crate_name: &syn::Ident) -> impl ToTokens {
        match self {
            Method::Get => quote!(#crate_name::http::Method::GET),
            Method::Post => quote!(#crate_name::http::Method::POST),
            Method::Put => quote!(#crate_name::http::Method::PUT),
            Method::Patch => quote!(#crate_name::http::Method::PATCH),
            Method::Delete => quote!(#crate_name::http::Method::DELETE),
            // Method::Options => quote!(#crate_name::http::Method::OPTIONS),
            Method::Head => quote!(#crate_name::http::Method::HEAD),
        }
    }
}

pub fn create(attr: TokenStream, item: TokenStream, method: Method) -> TokenStream {
    let crate_name = crate_ident_name("mandag");

    let attr_args = parse_macro_input!(attr as AttributeArgs);

    let RouteArgs { data, path } = match RouteArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let input = parse_macro_input!(item as ItemFn);

    let handler: proc_macro2::TokenStream = crate::handle::create_handler(&input, &data).into();

    let segments: Vec<_> = router::parse(&path).expect("parse").into();

    let len = segments.len();
    let segments_built = parse_route(&crate_name, segments).collect::<Vec<_>>();

    let method = method.to_tokens(&crate_name);

    let struct_name = &input.sig.ident;

    let route_name = "Test";

    quote!(

        #handler

        impl #struct_name {
            const SEGMENTS: [#crate_name::router::Segment<'static>; #len] = [#(#segments_built),*];

            const NAME: &'static str = #route_name;
        }

        impl #crate_name::router::IntoRoute for #struct_name {

            type Error = #crate_name::http::Error;

            fn into_route(self) -> Result<#crate_name::router::StaticRoute, Self::Error> {
                use #crate_name::prelude::*;
                let route = #crate_name::router::Route::new(#method, &Self::SEGMENTS, self.service());
                route.into_route()
            }
        }

        impl #crate_name::router::IntoRoutes for #struct_name


        {
            type Error = #crate_name::http::Error;

            fn into_routes(self) -> Result<Vec<#crate_name::router::StaticRoute>, Self::Error> {
                let route =  <#struct_name as #crate_name::router::IntoRoute>::into_route(self)?;
                Ok(vec![route])
            }
        }
    )
    .into()
}
