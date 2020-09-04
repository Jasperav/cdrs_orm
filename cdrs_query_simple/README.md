This crate provides a simple way how to use compile time and type checked queries. 
It just returns the query as a `&'static str` and the `QueryValues` as a tuple.

Note: this crate will insert rows in the keyspace that corresponds to env var TEST_CDRS_DB_KEYSPACE_KEY