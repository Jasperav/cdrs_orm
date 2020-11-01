#[test]
fn test() {
    let b = 1;
    let (query, values) =
        cdrs_query_simple::str_qv!("select * from test_table where b = 1 and c = ?", b);

    assert_eq!(query, "select * from test_table where b = 1 and c = ?");
    assert_eq!(values, cdrs::query_values!(1));

    let uuid0 = uuid::Uuid::new_v4();
    let uuid1 = uuid::Uuid::new_v4();
    let values = vec![uuid0, uuid1];
    let (query, values) =
        cdrs_query_simple::str_qv!("select * from uuid_table where pk in (?)", values);

    assert_eq!(query, "select * from uuid_table where pk in (?)");
    assert_eq!(values, cdrs::query_values!(uuid0, uuid1));

    let uuid2 = uuid::Uuid::new_v4();
    let values = vec![uuid2];
    let (_, values) =
        cdrs_query_simple::str_qv!("select * from uuid_table where pk in (?)", values);

    assert_eq!(values, cdrs::query_values!(uuid2));
}
