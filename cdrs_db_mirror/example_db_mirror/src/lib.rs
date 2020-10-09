mod some_serialized_struct;

use some_serialized_struct::SomeSerializedStruct;
use uuid::Uuid;

#[derive(
    cdrs_db_mirror::DBMirror,
    PartialEq,
    Debug,
    cdrs_helpers_derive::TryFromRow,
    rand_derive2::RandGen,
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
struct UUIDStruct {
    #[partition_key]
    id: Uuid,
    // Just some column that is not part of the primary key
    name: String,
}

#[derive(cdrs_db_mirror::DBMirror, rand_derive2::RandGen)]
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

#[derive(cdrs_db_mirror::DBMirror, rand_derive2::RandGen)]
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
        FooStruct, FooStructUpdateableColumns, SomeStruct, SomeStructPrimaryKey, StructJsonMapping,
        UUIDStruct, UUIDStructPrimaryKey,
    };
    use cdrs::query::QueryValues;
    use cdrs::query_values;
    use cdrs::types::value::Value;
    use cdrs_con::{create_test_db_session, keyspace, query_with_values, rows, use_keyspace};

    #[test]
    fn json() {
        let s = StructJsonMapping {
            a: SomeSerializedStruct { id: 1 },
            b: SomeSerializedStruct { id: 2 },
            c: Some(SomeSerializedStruct { id: 3 }),
        };

        let session = create_test_db_session();

        use_keyspace(&session, &keyspace());

        let (query, values) = StructJsonMapping::insert_qv(&s);
        query_with_values(&session, query, values);

        let (query, values) = s.primary_key().select_unique_qv();
        let r: Vec<StructJsonMapping> = rows(Ok(query_with_values(&session, query, values)));

        assert_eq!(1, r.len());
        assert_eq!(&s, r.first().unwrap());
    }

    #[test]
    fn test_insert_query() {
        let some_struct = generate_some_struct();
        let query_values: QueryValues = some_struct.query_values();

        if let QueryValues::NamedValues(nv) = query_values {
            assert_eq!(5, nv.len());

            let id_val: Value = some_struct.id.into();
            assert_eq!(&id_val, nv.get("id").unwrap());

            let cluster_key: Value = some_struct.cluster_key.into();
            assert_eq!(&cluster_key, nv.get("cluster_key").unwrap());

            let name_val: Value = some_struct.name.clone().into();
            assert_eq!(&name_val, nv.get("name").unwrap());
        } else {
            panic!("Expected named values");
        }

        let query = "insert into SomeStruct(id, another_id, cluster_key, another_cluster_key, name) values (?, ?, ?, ?, ?)";

        assert_eq!(query, SomeStruct::INSERT_QUERY);

        let (_query, _values) = SomeStruct::insert_qv(&some_struct);
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

        let (query, values) = FooStruct::update_qv(
            &foo_struct.primary_key(),
            Some(foo_struct.name.clone()),
            None,
        )
        .unwrap();

        assert_eq!(
            "update FooStruct set name = ? where id = ? and cluster_key = ?",
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

        let (query, values) = FooStruct::update_qv(
            &foo_struct.primary_key(),
            Some(foo_struct.name.clone()),
            Some(foo_struct.nickname.clone()),
        )
        .unwrap();

        assert_eq!(
            "update FooStruct set name = ?, nickname = ? where id = ? and cluster_key = ?",
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

        assert_eq!(
            None,
            FooStruct::update_qv(&foo_struct.primary_key(), None, None)
        );

        let (query, values) =
            FooStruct::update_qv_name(&foo_struct.primary_key(), foo_struct.name.clone());

        assert_eq!(
            "update FooStruct set name = ? where id = ? and cluster_key = ?",
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
            "update FooStruct set name = ? where id = ? and cluster_key = ?",
            FooStruct::UPDATE_NAME_QUERY
        );

        let (query, values) = FooStruct::update_multiple_qv(
            &foo_struct.primary_key(),
            vec![FooStructUpdateableColumns::Name(foo_struct.name.clone())],
        );

        assert_eq!(
            "update FooStruct set name = ? where id = ? and cluster_key = ?",
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

        let (query, values) = FooStruct::update_multiple_qv(
            &foo_struct.primary_key(),
            vec![
                FooStructUpdateableColumns::Name(foo_struct.name.clone()),
                FooStructUpdateableColumns::Nickname(foo_struct.nickname.clone()),
            ],
        );

        assert_eq!(
            "update FooStruct set name = ?, nickname = ? where id = ? and cluster_key = ?",
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
    fn test_dyn_update() {
        let f = FooStruct {
            id: 0,
            cluster_key: 0,
            name: "name".to_string(),
            nickname: "".to_string(),
        };
        let pk = f.primary_key();
        let s = "test".to_string();

        let (query_0, values_0) =
            FooStruct::update_dyn_qv(&pk, FooStructUpdateableColumns::Name(s.clone()));
        let (query_1, values_1) = FooStruct::update_qv_name(&pk, s);

        assert_eq!(query_0, query_1);
        assert_eq!(values_0, values_1);
    }

    #[test]
    fn test_select_queries() {
        // Tests unique
        let another_struct = UUIDStruct {
            id: uuid::Uuid::new_v4(),
            name: "".to_string(),
        };
        let (query, qv) = UUIDStructPrimaryKey {
            id: another_struct.id,
        }
        .select_unique_qv();

        assert_eq!("select * from UUIDStruct where id = ?", query);
        assert_eq!(query_values!(another_struct.id), qv);

        // Tests SelectAll
        assert_eq!("select * from UUIDStruct", UUIDStruct::SELECT_ALL_QUERY);

        // Tests SelectAllCount
        assert_eq!(
            "select count(*) from UUIDStruct",
            UUIDStruct::SELECT_ALL_COUNT_QUERY
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
        let pk = s.primary_key();

        assert_eq!(s.id, pk.id);
        assert_eq!(s.another_cluster_key, pk.another_cluster_key);
    }
}
