use cdrs_con::test_query;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::parse_quote::ParseQuote;
use syn::parse_str;
use syn::punctuated::Punctuated;

/// Information about a query
pub struct Query {
    // The query with leading and trailing quotes
    pub query_raw: syn::LitStr,
    // The query without any quotes
    pub query_pretty: String,
    pub idents: Vec<syn::Ident>,
    // The corresponding database types of the argument list
    pub types: Vec<syn::Type>,
    // The query values for the idents
    pub qv: proc_macro2::TokenStream,
    // More metadata
    pub qmd: cdrs_con::QueryMetaData,
}

impl Parse for Query {
    /// Parses a query like this: my_proc_macro!("select * from table where a = 1 and b = ?", b);
    /// So a literal query followed by a comma separated list of arguments that will replace the
    /// question marks
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let query: syn::Lit = syn::parse::Parse::parse(input)?;
        // TODO: This is a lot of work just to get the str out of a LitStr, isn't there a better way?
        let query_raw = match query {
            syn::Lit::Str(s) => s,
            _ => panic!("First argument is not a literal"),
        };
        let stringified = format!("{:?}", query_raw);
        let starting_quote = stringified
            .find('\"')
            .expect("Failed to find leading quote");
        let ending_quote = stringified
            .rfind('\"')
            .expect("Failed to find trailing quote");
        let query_pretty = stringified[starting_quote + 1..ending_quote].to_string();
        let idents = if input.is_empty() {
            vec![]
        } else {
            let _: syn::Token![,] = syn::parse::Parse::parse(input)?;
            let punc_idents: Punctuated<syn::Ident, syn::Token![,]> =
                <Punctuated<syn::Ident, syn::Token![,]>>::parse(input)?;

            punc_idents.iter().cloned().collect()
        };

        let qmd = test_query(&query_pretty);

        // Parametrized columns equals columns values that are filled with dynamic values from the user
        let parameterized_columns = qmd
            .extracted_columns
            .iter()
            .filter(|r| r.parameterized)
            .collect::<Vec<_>>();

        let (types, types_comparison): (Vec<syn::Type>, Vec<syn::Type>) = idents
            .iter()
            .enumerate()
            .map(|(index, _)| {
                let cdt = &qmd.parameterized_columns_data_types[index];
                let mut ty_string = cdt.to_ty().to_string();
                let mut ty_comparison = ty_string.clone();

                if index == parameterized_columns.len() {
                    // The limit column
                    assert!(qmd.limited);
                } else if parameterized_columns[index].uses_in_value {
                    ty_string = format!("std::vec::Vec<{}>", ty_string);
                    // This is used to compare types, to see if the types are assignable
                    // Another turbofish is needed
                    ty_comparison = format!("std::vec::Vec::<{}>", ty_comparison);
                }

                (
                    parse_str(&ty_string).expect("Failed to parse to type"),
                    parse_str(&ty_comparison).expect("Failed to parse to type"),
                )
            })
            .unzip();

        let qv = quote! {{
            let mut query_values: Vec<cdrs_tokio::types::value::Value> = Vec::new();

            #(
                // Check if the type is correct
                debug_assert!((#types_comparison::from(#idents.clone()), true).1);

                query_values.push(#idents.clone().into());
            )*

            cdrs_tokio::query::QueryValues::SimpleValues(query_values)
        }};

        Ok(Query {
            query_raw,
            query_pretty,
            idents,
            types,
            qv,
            qmd,
        })
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_compile() {}
}
