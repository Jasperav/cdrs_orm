# Cassandra/ScyllaDB Object-Relation Mapper

[![Latest Version](https://img.shields.io/crates/v/cdrs_orm.svg)](https://crates.io/crates/cdrs_orm)
[![Build Status](https://img.shields.io/github/workflow/status/jasperav/cdrs_orm/tests-cassandra/master)](https://github.com/jasperav/cdrs_orm/actions)

This crate features subcrates which allows you to
- automatically map tables to Rust structures (cdrs_to_rust): you don't have to manually map your tables to Rust structs.
- write compile time checked queries: with the cdrs_query_simple! query you can write queries that are checked at compile time.
- derive macro to generate boiler plate queries (insert, delete, select and update)
- JSON mapping. If a column should represent a type that is serde::Serialize and serde::Deserialize, 
it is possible to have a field with that type in the Rust struct. This means you don't have to manually serialize and deserialize.

All features have examples and a readme.md in the corresponding crate.
 
There are CI tests for Cassandra and ScyllaDB.

## Note
- this crate does use https://github.com/Jasperav/cdrs. If the open PR's on https://github.com/AlexPikalov/cdrs are merged,
that cdrs crate will be used
- not all types are supported yet. Feel free to add them! Supported types can be found in CassandraDataType in crate cdrs_con.  
- currently only available through git, not by crates.io (because the PR's aren't merged in the cdrs branch).

## Contributing
- Before creating a PR, make sure to run to run the executable 'internal-ci'. This will validate the code and run tests.

## Usage

Add the following to your Cargo.toml...

```toml
[dependencies]
cdrs_orm = "*"
```

...or add a dependency on a separate crate.

### Tip
Follow the steps below to have always in sync Rust structs derived from the database and compile time safe queries:
1. Generate the Rust structs with the cdrs_to_rust crate. Use a build.rs so that the structs are regenerated at compile time.
2. Create your own function mapping with the cdrs_db_mirror so you have your own business logic with pre-generated queries.
3. Use the cdrs_query_simple crate for compile time and type checked queries (note: set the TEST_CDRS_DB_KEYSPACE_KEY env var
to a test keyspace).
4. To see a full expanded entity with the db_mirror attribute, see package test_derived_equals/src/gen 

all crates have example projects

## Crates
This is a global overview of all the crates, for more information read the README.md of the crate.
#### cdrs_con
- querying meta data.
- creating connections for testing.
- reading and validating raw queries.
#### cdrs_db_mirror
Proc macro crate which provides the following derive macros which you can place on your type:
  - DBJson: indicates this type maps to a column type. The column type must be of type string in the database, but
  you can use this type in a Rust struct. This is automatic JSON mapping so you don't have to manually serialize and deserialize. 
  - DBMirror: generates CRUD methods. Don't annotate types yourself, you should use cdrs_to_rust which writes the
  annotations correctly. 
#### cdrs_orm_util
Used for namespacing.
#### cdrs_query
Compile time and type checked queries. Use this with a proc macro crate.
#### cdrs_query_simple
Simple proc macro you can use for validating queries.
The return value from the macro is a &'static str and query values.
#### cdrs_query_writer
The derive macro DBMirror writes CRUD methods for the annotated struct.
Most of the time, you want to have custom behaviour for your application, like settings a default consistency level
or returning a custom error type.
DBMirror's method names are based on this crate and so should your crate. Your can add your custom function name
and this crate will make it easy to call DBMirror's derived functions within your own functions.
#### cdrs_to_rust
Maps tables to Rust structs. It queries the database for the tables in the database and than this crate will
turn it into Rust structs.

## TODO
- Support more types
- Test coverage CI

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
