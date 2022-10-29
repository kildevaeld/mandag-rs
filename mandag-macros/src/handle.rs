use super::utils::parse_parameters;
use darling::{FromMeta, ToTokens};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, FnArg, ItemFn, PatType};

use crate::{
    shared::HandlerArgs,
    utils::{crate_ident_name, get_name},
};

pub fn create_handler(input: &ItemFn, data: &Option<String>) -> TokenStream {
    let crate_name = crate_ident_name("mandag");

    let call_params = parse_parameters(&input).map(|item| {
        let ty = &item;
        quote!(#ty)
    });

    let call_args = parse_parameters(&input).map(|item| {
        let pat = &item.pat;
        quote!(#pat)
    });

    let filter_input = |item: &&PatType| {
        //
        match &data {
            Some(data) => data != &get_name(&item.pat),
            None => false,
        }
    };

    let input_types = parse_parameters(&input).filter(filter_input).map(|item| {
        let ty = &item.ty;
        quote!(#ty)
    });

    let input_args = parse_parameters(&input).filter(filter_input).map(|item| {
        let ty = &item.pat;
        quote!(#ty)
    });

    let data_type = parse_data_type(&input, &crate_name, data.as_ref().map(|m| m.as_str()));
    let data_arg = syn::Ident::new(
        data.as_ref().map(|m| m.as_str()).unwrap_or("data"),
        Span::call_site(),
    );

    let struct_name = &input.sig.ident;
    let vis = &input.vis;

    let block = &input.block;

    quote! {

        #[derive(Clone, Copy, Debug, Default)]
        #[allow(non_camel_case_types)]
        #vis struct #struct_name;

        impl #struct_name {

            async fn _call<'r>(&self, #(#call_params),*) -> #crate_name::http::Response {
                use #crate_name::{Outcome, http::Reply, Error};
                let ret = async move #block.await;

                ret.into_response()

            }
        }

        #[mandag::async_trait]
        impl<'r> #crate_name::Handler<'r> for #struct_name {
            type Input = (
                #(#input_types),*
            );

            type Data = #data_type;
            type Output = #crate_name::http::Response;
            type Error = #crate_name::http::Error;


            async fn handle(&'r self, input: Self::Input, #data_arg: Self::Data) -> Result<Self::Output, Self::Error> {
                let (#(#input_args),*) = input;
                let ret = self._call(#(#call_args),*).await;
                Ok(ret)
            }
        }



    }.into()
}

pub fn create(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _crate_name = crate_ident_name("mandag");

    let attr_args = parse_macro_input!(attr as AttributeArgs);

    let HandlerArgs { data } = match HandlerArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let input = parse_macro_input!(item as ItemFn);

    create_handler(&input, &data)
}

fn parse_data_type<'a>(
    item: &'a ItemFn,
    _crate_name: &syn::Ident,
    data: Option<&str>,
) -> impl ToTokens + 'a {
    match data {
        None => quote!(()),
        Some(name) => {
            let found = item
                .sig
                .inputs
                .iter()
                .filter_map(|item| match item {
                    FnArg::Receiver(_) => None,
                    FnArg::Typed(item) => Some(item),
                })
                .find(|item| {
                    let pat_name = get_name(&item.pat);
                    name == &pat_name
                })
                .expect("data");
            let ty = &found.ty;
            quote!(#ty)
        }
    }
}
