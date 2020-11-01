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
        let mut idents = if input.is_empty() {
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

        let in_query = qmd
            .extracted_columns
            .iter()
            .filter(|c| c.uses_in_value)
            .collect::<Vec<_>>();
        // There can't be two 'in' keywords in a query
        assert!(in_query.len() <= 1);

        // Make a clone for the idents.
        // This is later on checked if the idents have the correct datatype
        // The idents variable can not be used, because an element can be removed from the idents
        // variable if the query contains a parameterized in query (see below)
        // But because the data type needs to be checked always, keep the data checking in the same vec.
        // So this comparison ident always contains the same elements as the ident variable, but when the
        // query contains a parameterized in query, this variable contains 1 element more.
        let idents_comparison = idents.clone();

        // This is some really ugly code
        // The idents must be transformed into the Value type from cdrs, so call into into them.
        // However, there is a bug for vecs: https://github.com/AlexPikalov/cdrs/issues/354.
        // If this bug is fixed, this ugly code can be removed.
        // A vec is only used for 'in' queries, and only 1 in keyword is allowed.
        // A workaround for the bug is to loop over the vec and call into on all elements separately.
        // Check if an 'in' query is used.
        let into_from_vec = match in_query.first() {
            Some(cv) if cv.parameterized => {
                // The query contains a parameterized in query.
                // The matching ident must be found which contains the values for the in query.
                // This is always a vec.
                // If a parameterized limit query is used, take the second-last element, else the last element.
                let index_to_remove = if query_pretty.ends_with(" limit ?") {
                    2
                } else {
                    1
                };

                let ident = idents.remove(idents.len() - index_to_remove);

                quote! {
                    for ident in #ident {
                        query_values.push(ident.into());
                    }
                }
            }
            _ => quote! {},
        };

        let qv = quote! {{
            let mut query_values: Vec<cdrs::types::value::Value> = Vec::new();

            #(
                // Check if the type is correct
                debug_assert!((#types_comparison::from(#idents_comparison.clone()), true).1);
            )*

            #(
                // Call into on every ident
                query_values.push(#idents.into());
            )*

            #into_from_vec

            cdrs::query::QueryValues::SimpleValues(query_values)
        }};

        Ok(Query {
            query_raw,
            query_pretty,
            idents,
            types,
            qmd,
            qv,
        })
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_compile() {}
}
