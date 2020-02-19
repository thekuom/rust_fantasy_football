extern crate proc_macro;

use quote::quote;
use syn;

use crate::proc_macro::TokenStream;

#[proc_macro_derive(DeserializeErrorHandler)]
pub fn deserialize_error_handler_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_deserialize_error_handler(&ast)
}

fn impl_deserialize_error_handler(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl crate::common::DeserializeErrorHandler for #name {
            fn handle_deserialize(cfg: actix_web::web::JsonConfig) -> actix_web::web::JsonConfig {
                cfg.error_handler(|err, _req| {
                    let err_message = format!("{}", &err);
                    actix_web::error::InternalError::from_response(
                        err, actix_web::HttpResponse::BadRequest().json(crate::common::JsonError::<bool> {
                            message: err_message,
                            data: None,
                        })).into()
                })
            }
        }
    };

    gen.into()
}
