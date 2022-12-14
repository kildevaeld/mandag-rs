use std::str::FromStr;

use super::shared::{RouteArgs, RouteDataArgs};
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
    Any,
}

impl Method {
    fn to_tokens(self, crate_name: &syn::Ident) -> impl ToTokens {
        match self {
            Method::Get => quote!(Some(#crate_name::http::Method::GET)),
            Method::Post => quote!(Some(#crate_name::http::Method::POST)),
            Method::Put => quote!(Some(#crate_name::http::Method::PUT)),
            Method::Patch => quote!(Some(#crate_name::http::Method::PATCH)),
            Method::Delete => quote!(Some(#crate_name::http::Method::DELETE)),
            // Method::Options => quote!(#crate_name::http::Method::OPTIONS),
            Method::Head => quote!(Some(#crate_name::http::Method::HEAD)),
            Method::Any => quote!(None),
        }
    }
}

impl FromStr for Method {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let method = match s {
            "get" => Method::Get,
            "post" => Method::Post,
            "patch" => Method::Patch,
            "put" => Method::Put,
            "delete" => Method::Delete,
            "any" => Method::Any,
            _ => return Err(format!("method not found: {}", s)),
        };
        Ok(method)
    }
}

pub fn route(
    input: &ItemFn,
    method: Method,
    path: String,
    data: Option<String>,
) -> proc_macro2::TokenStream {
    let crate_name = crate_ident_name("mandag");

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

            type Error = #crate_name::http::HttpError;

            fn into_route(self) -> Result<#crate_name::router::StaticRoute, Self::Error> {
                use #crate_name::prelude::*;
                let route = #crate_name::router::Route::new(#method, &Self::SEGMENTS, self.service());
                route.into_route()
            }
        }

        impl #crate_name::router::IntoRoutes for #struct_name


        {
            type Error = #crate_name::http::HttpError;

            fn into_routes(self) -> Result<Vec<#crate_name::router::StaticRoute>, Self::Error> {
                let route =  <#struct_name as #crate_name::router::IntoRoute>::into_route(self)?;
                Ok(vec![route])
            }
        }
    )
    .into()
}

pub fn create(attr: TokenStream, item: TokenStream, method: Method) -> TokenStream {
    let attr_args = parse_macro_input!(attr as AttributeArgs);

    let RouteArgs { path } = match RouteArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let item = parse_macro_input!(item as ItemFn);

    route(&item, method, path, None).into()
}

pub fn create_with_data(attr: TokenStream, item: TokenStream, method: Method) -> TokenStream {
    let attr_args = parse_macro_input!(attr as AttributeArgs);

    let RouteDataArgs { data, path } = match RouteDataArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let item = parse_macro_input!(item as ItemFn);

    route(&item, method, path, data).into()
}
