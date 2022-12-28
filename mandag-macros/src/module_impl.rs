use crate::{shared::ModuleArgs, utils::crate_ident_name};

use super::{
    route,
    shared::{RouteArgs, RouteDataArgs},
};
use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, Item, ItemFn, ItemMod};

pub fn create(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(attr as AttributeArgs);

    let ModuleArgs { path } = match ModuleArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let crate_name = crate_ident_name("mandag");

    let module = parse_macro_input!(item as ItemMod);

    let module_vis = &module.vis;
    let module_name = &module.ident;

    let mut routes = Vec::default();

    let content = match &module.content {
        Some((_, ret)) => process_items(ret, &mut routes),
        None => {
            panic!("no content")
        }
    };

    let struct_name = Ident::new("Routes", Span::call_site());

    let routes: Vec<_> = if let Some(path) = path {
        routes
            .into_iter()
            .map(|m| quote!(#m.mounted_on(#path).into_routes()?))
            .collect()
    } else {
        routes
            .into_iter()
            .map(|m| quote!(#m.into_routes()?))
            .collect()
    };

    let output = quote! {

        #module_vis mod #module_name {

            #content

            #[allow(non_camel_case_types)]
            pub struct #struct_name;

            impl #crate_name::router::IntoRoutes for #struct_name {
                type Error = #crate_name::http::HttpError;

                fn into_routes(self) -> Result<Vec<#crate_name::router::StaticRoute>, Self::Error> {
                    let mut routes = vec![];
                    #(
                        routes.extend(#routes);
                    )*
                    Ok(routes)
                }
            }


        }

    };

    output.into()
}

fn process_items(items: &Vec<Item>, routes: &mut Vec<Ident>) -> TokenStream2 {
    let items = items.iter().map(|item| {
        //
        match item {
            Item::Fn(func) => process_func(func, routes),
            item => quote!(#item).into(),
        }
    });

    //
    quote! {
        #(#items)*
    }
    .into()
}

fn process_func(item: &ItemFn, routes: &mut Vec<Ident>) -> TokenStream2 {
    let mut filtered = item
        .attrs
        .iter()
        .filter_map(|item| {
            if let Some(ident) = item.path.get_ident() {
                let method = ident.to_string();
                match method.as_str() {
                    "get" | "post" | "put" | "patch" | "delete" | "head" => Some((method, item)),
                    _ => None,
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    if filtered.is_empty() {
        return quote!(#item);
    } else if filtered.len() > 1 {
        panic!("multiple")
    }

    let (path, attr) = filtered.pop().expect("no attr");

    let meta = attr.parse_meta().unwrap();

    let method = path.parse().expect("method");

    let route = match path.as_str() {
        "get" | "delete" | "head" => {
            let options = RouteArgs::from_meta(&meta).expect("options");
            route::route(item, method, options.path, None)
        }
        "post" | "put" | "patch" => {
            let options = RouteDataArgs::from_meta(&meta).expect("options");
            route::route(item, method, options.path, options.data)
        }
        _ => {
            panic!("invalid method: {}", path)
        }
    };

    routes.push(item.sig.ident.clone());

    //
    quote! {
        #route
    }
    .into()
}
