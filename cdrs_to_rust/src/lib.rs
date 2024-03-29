use proc_macro2::TokenStream;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

mod column_mapper;
mod constants;
mod sort;
mod transform;

use cdrs_con::materialized_view::query_materialized_views;
use cdrs_con::*;
use column_mapper::*;
use quote::format_ident;
use sort::*;
use std::path::Path;
pub use transform::{DefaultTransformer, Transformer};

/// base_dir: an absolute path where to place the generated files.
///     CAREFUL: all the files in this map can be deleted
///     Some something like: Users/myself/project/src/generated/
///     On Windows, the path should be something like C:\\users\\myself\\project\\src\\generated\\
///
/// transformer: trait in which customization can take place.
/// imports: custom imports to add for all types
pub fn generate(base_dir: &Path, transformer: impl Transformer, imports: TokenStream) {
    let keyspace = keyspace();
    let mut current_dir = env::current_dir().unwrap();

    comp_pb(&base_dir, &current_dir);

    current_dir.push("src");

    comp_pb(&base_dir, &current_dir);

    let session = create_test_db_session();

    use_keyspace(&session, &keyspace);

    // Query all the tables
    let mut tables: Vec<TableName> = rows(Ok(query(
        &session,
        &format!(
            "select * from system_schema.tables where keyspace_name = '{}'",
            &keyspace
        ),
    )));
    // Materialized views
    let mut views = query_materialized_views(&session)
        .into_iter()
        .map(|r| TableName {
            table_name: r.table_name,
        })
        .collect();

    tables.append(&mut views);

    let table_names = tables.into_iter().map(|t| t.table_name).collect::<Vec<_>>();

    // Ignore result, because it can fail if this is the first time generating the structs
    let _ = std::fs::remove_dir_all(base_dir);
    std::fs::create_dir_all(base_dir).unwrap();

    // Create the mod file
    // For each and every generated rust file, 3 lines will be written inside this mod file:
    // 1. mod x
    // 2. pub use x::X;
    // 3. newline
    let mut path_mod_file = base_dir.to_path_buf();

    path_mod_file.push("mod.rs");

    let mut mod_file = File::create(path_mod_file).unwrap();

    add_generated_header(&mut mod_file);

    for table in table_names {
        println!("Processing table: {}", table.clone());

        // Transform table name to struct name
        let struct_name = format_ident!("{}", transformer.table_name_to_struct_name(&table));

        let file_name = transformer.filename(&table, &struct_name);

        write!(
            mod_file,
            "{}",
            &transformer.mod_file(&table, &struct_name, &file_name)
        )
        .unwrap();

        // Query all the columns for this table
        let mut columns = query_columns(&session, table.as_str());

        // Sort the columns
        sort_columns(&mut columns);

        // Create the file to place the generated rust code in
        let path_mod_file = base_dir.join(format!("{}.rs", file_name));

        let mut file = File::create(path_mod_file).unwrap();

        add_generated_header(&mut file);

        // Maps columns to properties
        let properties = column_to_property(&table, columns, &transformer, &mut file);
        // Generate the tokens needed for the rust struct
        let struct_tokens = properties_to_struct(&struct_name, properties, &transformer, &imports);

        write!(file, "{}", struct_tokens).unwrap();

        // Format the output, since everything is on 1 line
        assert!(Command::new("rustfmt")
            .arg(format!("{}.rs", table))
            .current_dir(base_dir)
            .status()
            .unwrap()
            .success());
    }

    drop(mod_file);

    // Format the output, since everything is on 1 line
    assert!(Command::new("rustfmt")
        .arg("mod.rs")
        .current_dir(base_dir)
        .status()
        .unwrap()
        .success());
}

pub fn add_generated_header(file: &mut File) {
    assert_eq!(0, file.metadata().unwrap().len());

    writeln!(file, "// Generated file").unwrap();
}

fn comp_pb(left: &Path, right: &Path) {
    let err_message = "Please create a map inside src, e.g. src/generated";

    assert_ne!(
        left.to_str().unwrap(),
        right.to_str().unwrap(),
        "{}",
        err_message
    );
}

pub struct JsonMapping {
    pub import: String,
    pub raw_type: proc_macro2::TokenStream,
    pub nullable: bool,
}

#[cfg(test)]
mod test {
    use super::*;
    use quote::quote;

    #[should_panic]
    #[test]
    fn test_illegal_base_dir0() {
        test_dir(&env::current_dir().unwrap());
    }

    #[should_panic]
    #[test]
    fn test_illegal_base_dir1() {
        let mut current_dir = env::current_dir().unwrap();

        current_dir.push("src");

        test_dir(&current_dir);
    }

    fn test_dir(dir: &Path) {
        generate(dir, DefaultTransformer, quote! {})
    }

    #[test]
    fn test_generate() {
        dotenv::dotenv().unwrap();

        std::env::set_var("CAMEL_CASE_TO_DB_SNAKE_CASE", "0");

        let session = create_test_db_session();

        // It's important no other tables exists for this test
        recreate_keyspace(&session, &keyspace());
        create_dummy_table(&session);

        query(
            &session,
            "create table Test2(a int, b int, primary key((a), b))",
        );
        query(
            &session,
            "create materialized view Test2Materialized as
            select *
            from Test2
            where a is not null and b is not null
            primary key ((b), a)",
        );
        query(&session, "create table TestJsonMapping(a int, json text, json_nullable text, primary key((a), json))");

        let mut current_dir = env::current_dir().unwrap();

        current_dir.push("src");

        let mut gen_dir = current_dir.clone();

        gen_dir.push("generated");

        struct Trans;

        impl Transformer for Trans {
            fn json_mapping(&self, _table_name: &str, column_name: &str) -> Option<JsonMapping> {
                if column_name.contains("json") {
                    return Some(JsonMapping {
                        import: "".to_string(),
                        raw_type: quote! {
                            MyJsonType
                        },
                        nullable: column_name == "json_nullable",
                    });
                }

                None
            }

            fn metadata(&self, struct_name: &str) -> Vec<String> {
                if struct_name == "testjsonmapping" {
                    vec!["use crate::MyJsonType;".to_string()]
                } else {
                    vec![]
                }
            }
        }

        generate(&gen_dir, Trans, quote! {});

        let paths = std::fs::read_dir(&gen_dir).unwrap();

        for path in paths {
            let path = path.unwrap();

            println!("Checking {:#?}", path);

            let file1 = std::fs::read_to_string(path.path()).unwrap();
            let filename = path.file_name();
            let filename_str = filename.to_str().unwrap();

            let mut dir_file_result = current_dir.clone();

            dir_file_result.push("test_result");
            dir_file_result.push(filename_str);

            let file2 = std::fs::read_to_string(dir_file_result).unwrap();
            let file2 = file2.replace("\r", "");

            assert_eq!(file1, file2, "{}", filename_str);
        }

        // remove the generated files
        std::fs::remove_dir_all(gen_dir).unwrap();
    }
}
