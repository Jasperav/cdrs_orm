mod impl_writer;

use crate::impl_writer::ImplWriter;
use syn::DeriveInput;

#[proc_macro_derive(Entity)]
pub fn entity(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();

    cdrs_query_writer::write(input, ImplWriter).into()
}
