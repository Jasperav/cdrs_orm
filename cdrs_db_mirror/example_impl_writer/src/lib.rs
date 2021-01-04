mod generated;

#[cfg(test)]
mod test {
    use crate::generated::some_struct::SomeStruct;

    #[test]
    fn test_methods_available() {
        SomeStruct::c_select_unique();
        SomeStruct::c_delete_unique();
        SomeStruct::c_insert_unique();
        SomeStruct::c_update_opt();
        SomeStruct::c_update_name();
    }
}
