use crate::{
    update_single_column, UPDATE_MULTIPLE_COLUMNS, UPDATE_OPTIONALS, UPDATE_SINGLE_COLUMN_DYNAMIC,
};
use crate::{DynamicMultipleUpdates, DynamicUpdate, Inf, Update, Writer, CRUD};
use cdrs_con::capitalizing::snake_case_to_upper_camel_case;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, Type};

/// Writes the updates query
pub fn generate(inf: &Inf, writer: &impl Writer) -> TokenStream {
    let mut tokens = TokenStream::new();

    let (idents, ty): (Vec<Ident>, Vec<Type>) = inf
        .non_pk_fields
        .iter()
        .map(|f| (f.ident.clone().unwrap(), f.ty.clone()))
        .unzip();
    let optional_ty = ty
        .iter()
        .map(|f| {
            let ty = f.clone();

            let ty_optional = quote! {
                std::option::Option<#ty>
            };

            syn::parse2(ty_optional).expect("Error while trying to wrap ty into Option<ty>")
        })
        .collect::<Vec<Type>>();

    let enum_cases = idents
        .iter()
        .map(|i| snake_case_to_upper_camel_case(&i.to_string()))
        .map(|i| format_ident!("{}", i))
        .collect();

    // Even if the entity does not have updateable columns, still create the enum
    // Also do it for materialized views, although they can not be updated, it's easier for code generation
    tokens.extend(
        writer.write(
            inf,
            &format_ident!("{}", UPDATE_SINGLE_COLUMN_DYNAMIC),
            &format_ident!("{}", writer.fn_name_update_column_dynamic()),
            CRUD::UpdateUnique(Update::Dynamic(DynamicUpdate {
                updateable_columns: &idents,
                updateable_columns_types: &ty,
                enum_cases: &enum_cases,
                enum_method_names: &idents
                    .iter()
                    .map(|i| fn_name_single_column(i, writer).1)
                    .collect(),
            })),
        ),
    );

    if inf.non_pk_fields.is_empty() || inf.materialized_view.is_some() {
        return tokens;
    }

    tokens.extend(writer.write(
        inf,
        &format_ident!("{}", UPDATE_OPTIONALS),
        &format_ident!("{}", writer.fn_name_update_optionals()),
        CRUD::UpdateUnique(Update::AllOptional((&idents, &optional_ty))),
    ));

    tokens.extend(writer.write(
        inf,
        &format_ident!("{}", UPDATE_MULTIPLE_COLUMNS),
        &format_ident!("{}", writer.fn_name_update_multiple_columns()),
        CRUD::UpdateUnique(Update::DynamicVec(DynamicMultipleUpdates {
            enum_cases: &enum_cases,
            enum_column_names: &idents,
        })),
    ));

    for (ident, ty) in idents.iter().zip(ty.iter()) {
        let (db_mirror_fn_name, custom_fn_name) = fn_name_single_column(ident, writer);

        tokens.extend(writer.write(
            inf,
            &db_mirror_fn_name,
            &custom_fn_name,
            CRUD::UpdateUnique(Update::SingleColumn((&ident, &ty))),
        ));
    }

    tokens
}

fn fn_name_single_column(ident: &Ident, writer: &impl Writer) -> (Ident, Ident) {
    let column_name = ident.to_string();
    let db_mirror_fn_name = format_ident!("{}", update_single_column(&column_name));
    let custom_fn_name = format_ident!("{}", writer.fn_name_update_column(&column_name));

    (db_mirror_fn_name, custom_fn_name)
}
