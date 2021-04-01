mod some_serialized_struct;

use some_serialized_struct::SomeSerializedStruct;
use uuid::Uuid;

#[derive(
    cdrs_db_mirror::DBMirror,
    PartialEq,
    Debug,
    cdrs_tokio_helpers_derive::TryFromRow,
    rand_derive2::RandGen,
    Clone,
)]
#[allow(dead_code)]
struct StructJsonMapping {
    #[partition_key]
    #[json_mapped]
    a: SomeSerializedStruct,
    #[json_mapped]
    b: SomeSerializedStruct,
    #[json_mapped]
    c: std::option::Option<SomeSerializedStruct>,
}

#[derive(cdrs_db_mirror::DBMirror, rand_derive2::RandGen)]
#[allow(dead_code)]
struct AnotherStruct {
    #[partition_key]
    id: i32,
    // Just some column that is not part of the primary key
    name: String,
}

#[derive(cdrs_db_mirror::DBMirror, rand_derive2::RandGen)]
#[allow(dead_code)]
struct UuidStruct {
    #[partition_key]
    id: Uuid,
    // Just some column that is not part of the primary key
    name: String,
}

#[derive(cdrs_db_mirror::DBMirror, rand_derive2::RandGen, Clone)]
#[allow(dead_code)]
struct FooStruct {
    #[partition_key]
    id: i32,
    #[clustering_key]
    cluster_key: i32,
    // Just some columns that are not part of the primary key
    name: String,
    nickname: String,
}

#[derive(cdrs_db_mirror::DBMirror, rand_derive2::RandGen, Clone)]
#[allow(dead_code)]
struct SomeStruct {
    #[partition_key]
    id: i32,
    #[partition_key]
    another_id: i32,
    #[clustering_key]
    cluster_key: i32,
    #[clustering_key]
    another_cluster_key: i32,
    // Just some column that is not part of the primary key
    name: String,
}

#[cfg(test)]
mod test_db_mirror {
    use crate::some_serialized_struct::SomeSerializedStruct;
    use crate::{
        FooStruct, FooStructUpdatableColumns, SomeStruct, SomeStructPrimaryKey, StructJsonMapping,
        UuidStruct, UuidStructPrimaryKey,
    };
    use cdrs_con::cdrs_tokio_transformer::{query_with_values_tokio, rows_tokio};
    use cdrs_con::{create_test_db_session, keyspace, use_keyspace};
    use cdrs_tokio::query::QueryValues;
    use cdrs_tokio::query_values;
    use cdrs_tokio::types::value::Value;

    #[test]
    fn json() {
        dotenv::dotenv().unwrap();

        let s = StructJsonMapping {
            a: SomeSerializedStruct { id: 1 },
            b: SomeSerializedStruct { id: 2 },
            c: Some(SomeSerializedStruct { id: 3 }),
        };

        let session = create_test_db_session();

        use_keyspace(&session, &keyspace());

        let (query, values) = StructJsonMapping::insert_qv(s.clone());

        query_with_values_tokio(&session, query, values);

        let (query, values) = s.clone_primary_key().select_unique_qv();
        let r: Vec<StructJsonMapping> =
            rows_tokio(Ok(query_with_values_tokio(&session, query, values)));

        assert_eq!(1, r.len());
        assert_eq!(&s, r.first().unwrap());
    }

    #[test]
    fn test_insert_query() {
        let some_struct = generate_some_struct();
        let query_values: QueryValues = some_struct.clone().query_values();

        if let QueryValues::SimpleValues(sv) = query_values {
            assert_eq!(5, sv.len());

            let id_val: Value = some_struct.id.into();
            assert_eq!(&id_val, sv.get(0).unwrap());

            let another_id: Value = some_struct.another_id.into();
            assert_eq!(&another_id, sv.get(1).unwrap());

            let name_val: Value = some_struct.name.clone().into();
            assert_eq!(&name_val, sv.get(4).unwrap());
        } else {
            panic!("Expected simple values");
        }

        let query = "insert into some_struct (id, another_id, cluster_key, another_cluster_key, name) values (?, ?, ?, ?, ?)";

        assert_eq!(query, SomeStruct::INSERT_QUERY);

        let _ = SomeStruct::insert_qv(some_struct);
    }

    fn generate_some_struct() -> SomeStruct {
        SomeStruct {
            id: 1,
            another_id: 2,
            cluster_key: 3,
            another_cluster_key: 4,
            name: "name".to_string(),
        }
    }

    #[test]
    fn test_update_queries() {
        let foo_struct = FooStruct {
            id: 1,
            cluster_key: 2,
            name: "name".to_string(),
            nickname: "nickname".to_string(),
        };

        let (query, values) = foo_struct
            .clone_primary_key()
            .update_qv(Some(foo_struct.name.clone()), None)
            .unwrap();

        assert_eq!(
            "update foo_struct set name = ? where id = ? and cluster_key = ?",
            query
        );
        assert_eq!(
            query_values!(
                foo_struct.name.clone(),
                foo_struct.id,
                foo_struct.cluster_key
            ),
            values
        );

        let (query, values) = foo_struct
            .clone()
            .primary_key()
            .update_qv(
                Some(foo_struct.name.clone()),
                Some(foo_struct.nickname.clone()),
            )
            .unwrap();

        assert_eq!(
            "update foo_struct set name = ?, nickname = ? where id = ? and cluster_key = ?",
            query
        );
        assert_eq!(
            query_values!(
                foo_struct.name.clone(),
                foo_struct.nickname.clone(),
                foo_struct.id,
                foo_struct.cluster_key
            ),
            values
        );

        assert_eq!(None, foo_struct.clone_primary_key().update_qv(None, None));

        let (query, values) = foo_struct
            .clone()
            .primary_key()
            .update_qv_name(foo_struct.name.clone());

        assert_eq!(
            "update foo_struct set name = ? where id = ? and cluster_key = ?",
            query
        );
        assert_eq!(
            query_values!(
                foo_struct.name.clone(),
                foo_struct.id,
                foo_struct.cluster_key
            ),
            values
        );
        assert_eq!(
            "update foo_struct set name = ? where id = ? and cluster_key = ?",
            FooStruct::UPDATE_NAME_QUERY
        );

        let (query, values) = foo_struct.clone_primary_key().update_multiple_qv(vec![
            FooStructUpdatableColumns::Name(foo_struct.name.clone()),
        ]);

        assert_eq!(
            "update foo_struct set name = ? where id = ? and cluster_key = ?",
            query
        );
        assert_eq!(
            query_values!(
                foo_struct.name.clone(),
                foo_struct.id,
                foo_struct.cluster_key
            ),
            values
        );

        let (query, values) = foo_struct.clone_primary_key().update_multiple_qv(vec![
            FooStructUpdatableColumns::Name(foo_struct.name.clone()),
            FooStructUpdatableColumns::Nickname(foo_struct.nickname.clone()),
        ]);

        assert_eq!(
            "update foo_struct set name = ?, nickname = ? where id = ? and cluster_key = ?",
            query
        );
        assert_eq!(
            query_values!(
                foo_struct.name.clone(),
                foo_struct.nickname.clone(),
                foo_struct.id,
                foo_struct.cluster_key
            ),
            values
        );
    }

    #[test]
    fn test_truncate() {
        let query = FooStruct::TRUNCATE_QUERY;

        assert_eq!("truncate foo_struct", query);
    }

    #[test]
    fn test_dyn_update() {
        let f = FooStruct {
            id: 0,
            cluster_key: 0,
            name: "name".to_string(),
            nickname: "".to_string(),
        };
        let s = "test".to_string();

        let (query_0, values_0) = f
            .clone_primary_key()
            .update_dyn_qv(FooStructUpdatableColumns::Name(s.clone()));
        let (query_1, values_1) = f.clone_primary_key().update_qv_name(s);

        assert_eq!(query_0, query_1);
        assert_eq!(values_0, values_1);
    }

    #[test]
    fn test_select_queries() {
        // Tests unique
        let another_struct = UuidStruct {
            id: uuid::Uuid::new_v4(),
            name: "".to_string(),
        };
        let (query, qv) = UuidStructPrimaryKey {
            id: another_struct.id,
        }
        .select_unique_qv();

        assert_eq!("select * from uuid_struct where id = ?", query);
        assert_eq!(query_values!(another_struct.id), qv);

        // Tests SelectAll
        assert_eq!("select * from uuid_struct", UuidStruct::SELECT_ALL_QUERY);

        // Tests SelectAllCount
        assert_eq!(
            "select count(*) from uuid_struct",
            UuidStruct::SELECT_ALL_COUNT_QUERY
        );
    }

    #[test]
    fn test_primary_key() {
        let pk = SomeStructPrimaryKey {
            id: 1,
            another_id: 1,
            cluster_key: 1,
            another_cluster_key: 1,
        };

        assert_eq!(
            " where id = ? and another_id = ? and cluster_key = ? and another_cluster_key = ?",
            SomeStructPrimaryKey::WHERE_CLAUSE_PK
        );
        assert_eq!(query_values!(1, 1, 1, 1), pk.where_clause());

        let s = generate_some_struct();
        let pk = s.clone_primary_key();

        assert_eq!(s.id, pk.id);
        assert_eq!(s.another_cluster_key, pk.another_cluster_key);
    }

    /// Checks that there are 2 methods to retrieve a primary key:
    /// 1. With `self`
    /// 2. Without `self`
    ///
    /// This is a test that checks if `self` is actually taken (case 1)
    /// ```compile_fail
    /// let some_struct = SomeStruct {
    ///     id: 0,
    ///     another_id: 0,
    ///     cluster_key: 2,
    ///     another_cluster_key: 0,
    ///     name: "nameee".to_string()
    /// };
    /// let _ = some_struct.primary_key();
    /// let _ = some_struct.primary_key();
    /// ```
    /// Below it tests case 2
    #[test]
    fn test_primary_key_without_clone() {
        let some_struct = SomeStruct {
            id: 0,
            another_id: 0,
            cluster_key: 2,
            another_cluster_key: 0,
            name: "nameee".to_string(),
        };

        let _ = some_struct.clone_primary_key();
        let pk = some_struct.clone_primary_key();

        assert_eq!(pk, some_struct.primary_key());
    }
}
