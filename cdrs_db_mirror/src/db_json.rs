use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn db_json(derive_input: DeriveInput) -> TokenStream {
    let name = derive_input.ident;

    quote! {
        impl From<#name> for cdrs_tokio::types::value::Bytes {
            fn from(s: #name) -> cdrs_tokio::types::value::Bytes {
                serde_json::to_string(&s).unwrap().into()
            }
        }
    }
}
