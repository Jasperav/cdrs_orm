#[test]
fn test() {
    let b = 1;
    let (query, values) =
        cdrs_query_simple::str_qv!("select * from test_table where b = 1 and c = ?", b);

    assert_eq!(query, "select * from test_table where b = 1 and c = ?");
    assert_eq!(values, cdrs::query_values!(1))
}
