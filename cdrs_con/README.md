# cdrs_con

This crate provides the following functionality:
- validating queries, this can be used to create compile-time safe queries (check cdrs_query_simple)
- creating test keyspaces
- translating tablenames to Rust struct names: sometimes tablenames are snake_cased and the corresponding Rust structure
most be CamelCased. There are methods to do this conversion.
- querying meta data about tables and also about materialized views