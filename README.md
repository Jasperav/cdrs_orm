❗This crates aren't maintained anymore. Consider using https://github.com/scylladb/scylla-rust-driver with the
new orm: https://github.com/Jasperav/scylla_orm. The new orm provides much better code generation❗



# Cassandra/ScyllaDB Object-Relation Mapper

[![Latest Version](https://img.shields.io/crates/v/cdrs_orm.svg)](https://crates.io/crates/cdrs_orm)
[![Build Status](https://img.shields.io/github/workflow/status/jasperav/cdrs_orm/tests-cassandra/master)](https://github.com/jasperav/cdrs_orm/actions)

This crate features subcrates which allows you to
- automatic map tables to Rust `struct`s (crate: **cdrs_to_rust**).
- compile time checked queries (crate: **cdrs_query_simple**)
- derive macro to generate CRUD boilerplate queries (crate: **db_mirror**)
- automatic JSON mapping (crate: **db_mirror**)

All features have examples and a readme.md in the corresponding crate.
 
## Testing
There are CI tests for both Cassandra and ScyllaDB.
Before creating a PR, make sure to run to run the executable 'internal-ci'. This will validate the code and run the tests.

## Note
- this crate does use https://github.com/Jasperav/cdrs_tokio because it is not fully compatible with https://github.com/krojew/cdrs-tokio. 
- not all types are supported yet. Feel free to add them! Supported types can be found in CassandraDataType in crate **cdrs_con**.  
- currently only available through git, not by crates.io.
- cdrs_con uses the sync version of cdrs, because proc macro's are not async.

## Compatibility
- sync version is supported in the `sync` branch
- cdrs-tokio version supported by default
- scylla native driver will be supported when it is production ready

## Usage

Add the following to your Cargo.toml...

```toml
[dependencies]
cdrs_orm = "*"
```

...or add a dependency on one of the separate crates. The **cdrs_orm** crate re-exports all of the separate crates.

This is a global overview of all the crates, for more information read the README.md in the crate or check the example projects
in the crates.

**cdrs_con**
- querying meta data.
- creating connections for testing.
- reading and validating raw queries.

**cdrs_db_mirror**

Proc macro crate which provides the following derive macros which you can place on top of your type:
- `DBJson`: indicates this type maps to a column type. The column type must be of type `string` in the database, but
  you can use the JSON type in a Rust `struct`. You don't have to manually serialize and deserialize the JSON type anymore. 
- `DBMirror`: generates CRUD methods based on property annotations. Note: don't annotate the fields yourself, you should use the `cdrs_to_rust` crate which writes the
  `struct` and annotations correctly. 

**cdrs_orm_util**

Used for namespacing.

**cdrs_query**

Compile time and type checked queries.

**cdrs_query_simple**

Simple proc macro you can use for validating queries with the **cdrs_query** crate.
The return value from the macro is a `&'static str` and query values.

**cdrs_query_writer**

The derive macro `DBMirror` writes CRUD methods for the annotated struct.
Most of the time, you want to have custom behaviour for your application, like settings a default consistency level
or returning custom error types.
`DBMirror`'s method names are based on this crate and so should your crate. Your can add your custom function name
and this crate will make it easy to call `DBMirror`'s derived functions within your own functions.

**cdrs_to_rust**

Maps tables to Rust `struct`s. It queries the database for the tables in the database and than this crate will
turn it into Rust structs.

### Tip
Follow the steps below to have always in sync Rust `struct`s derived from the database and compile time safe queries:
1. Generate the Rust `struct`s with the **cdrs_to_rust** crate. Use a build.rs so that the `struct`s are regenerated at compile time.
2. Create your own function mapping with the **cdrs_db_mirror** crate so you have your own business logic with pre-generated queries.
3. Use the **cdrs_query_simple** crate for compile time and type checked queries.
4. To see a full expanded entity with the `db_mirror` attribute, see package test_derived_equals/src/gen 

All crates have example projects!

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
