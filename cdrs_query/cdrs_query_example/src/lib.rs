#[cfg(test)]
mod test {
    use cdrs_query_example_proc_macro::control;
    use cdrs_tokio::query_values;

    #[test]
    fn test_non_compiling_code() {
        let t = trybuild::TestCases::new();
        let current_dir = std::env::current_dir().unwrap().join("src");

        t.compile_fail(current_dir.join("failing_wrong_type_primitive.rs"));
        t.compile_fail(current_dir.join("failing_wrong_type_vec.rs"));
        t.compile_fail(current_dir.join("non_complete_insert.rs"));
        t.compile_fail(current_dir.join("non_existing_column.rs"));
        t.compile_fail(current_dir.join("non_existing_table.rs"));
    }

    #[test]
    fn test_qmd() {
        assert_eq!(0, control!("select * from test_table").0);
        assert_eq!(
            1,
            control!("select * from test_table where b = 1 and c = 2 and d = 3 and a = 3").0
        );
        assert_eq!(2, control!("select * from test_table limit 1").0);
        assert_eq!(
            3,
            control!("select count(*) from test_table where b = 1 and c = 2").0
        );
        assert_eq!(
            4,
            control!("update test_table set e = 3 where b = 1 and c = 2 and d = 3 and a =3").0
        );
        assert_eq!(
            5,
            control!("delete from test_table where b = 1 and c = 3").0
        );
        assert_eq!(
            6,
            control!("delete from test_table where b = 1 and c = 3 and d = 4 and a = 2").0
        );
        assert_eq!(
            7,
            control!("insert into test_table (b, c, d, a, e) values (2, 3, 4, 5, 6)").0
        );

        assert_eq!(query_values!(), control!("select * from test_table").1);
        assert_eq!(
            query_values!(),
            control!("select * from test_table where b = 1 and c = 2").1
        );

        let a = 1;
        assert_eq!(
            query_values!(1),
            control!("select * from test_table where b = 1 and c = ?", a).1
        );

        let b = "sadas";
        control!("select * from another_test_table where a = 1 and b = ?", b);

        let a = vec![1, 2, 3];
        let (_, values) = control!("select * from test_table where b = 1 and c in ?", a);

        assert_eq!(query_values!(a), values);

        let a = vec![1, 2];
        control!("select * from test_table where b = 1 and c in ? limit 1", a);

        let val = 1;
        let c = 5;

        let (_, values) = control!(
            "select * from test_table where b = ? and c in ? limit ?",
            c,
            a,
            val
        );

        assert_eq!(query_values!(c, a, val), values);

        assert_eq!(8, control!("truncate test_table").0);
    }
}
