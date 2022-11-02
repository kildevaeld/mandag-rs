#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod api {
    #[allow(non_camel_case_types)]
    pub struct index;
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::clone::Clone for index {
        #[inline]
        fn clone(&self) -> index {
            *self
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::marker::Copy for index {}
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::fmt::Debug for index {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "index")
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::default::Default for index {
        #[inline]
        fn default() -> index {
            index {}
        }
    }
    impl index {
        async fn _call<'r>(&self) -> mandag::http::Response {
            use mandag::{Outcome, http::Reply, Error};
            let ret = async move { "Hello, World!" }.await;
            ret.into_response()
        }
    }
    impl<'r> mandag::Handler<'r> for index {
        type Input = ();
        type Data = ();
        type Output = mandag::http::Response;
        type Error = mandag::http::Error;
        #[allow(
            clippy::let_unit_value,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn handle<'async_trait>(
            &'r self,
            input: Self::Input,
            data: Self::Data,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<Output = Result<Self::Output, Self::Error>>
                    + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            'r: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) =
                    ::core::option::Option::None::<Result<Self::Output, Self::Error>>
                {
                    return __ret;
                }
                let __self = self;
                let input = input;
                let data = data;
                let __ret: Result<Self::Output, Self::Error> = {
                    let () = input;
                    let ret = __self._call().await;
                    Ok(ret)
                };
                #[allow(unreachable_code)]
                __ret
            })
        }
    }
    impl index {
        const SEGMENTS: [mandag::router::Segment<'static>; 0usize] = [];
        const NAME: &'static str = "Test";
    }
    impl mandag::router::IntoRoute for index {
        type Error = mandag::http::Error;
        fn into_route(self) -> Result<mandag::router::StaticRoute, Self::Error> {
            use mandag::prelude::*;
            let route = mandag::router::Route::new(
                mandag::http::Method::GET,
                &Self::SEGMENTS,
                self.service(),
            );
            route.into_route()
        }
    }
    impl mandag::router::IntoRoutes for index {
        type Error = mandag::http::Error;
        fn into_routes(self) -> Result<Vec<mandag::router::StaticRoute>, Self::Error> {
            let route = <index as mandag::router::IntoRoute>::into_route(self)?;
            Ok(<[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([route]),
            ))
        }
    }
    #[allow(non_camel_case_types)]
    pub struct route;
    impl mandag::router::IntoRoutes for route {
        type Error = mandag::http::Error;
        fn into_routes(self) -> Result<Vec<mandag::router::StaticRoute>, Self::Error> {
            let mut routes = ::alloc::vec::Vec::new();
            routes.extend(index.into_routes()?);
            Ok(routes)
        }
    }
}
fn main() {
    api::route;
}
