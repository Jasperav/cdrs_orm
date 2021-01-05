use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn db_json(derive_input: DeriveInput) -> TokenStream {
    let name = derive_input.ident;

    quote! {
        // Not sure how to make this works with From
        #[allow(clippy::from_over_into)]
        impl Into<cdrs::types::value::Bytes> for #name {
            fn into(self) -> cdrs::types::value::Bytes {
                serde_json::to_string(&self).unwrap().into()
            }
        }
    }
}
