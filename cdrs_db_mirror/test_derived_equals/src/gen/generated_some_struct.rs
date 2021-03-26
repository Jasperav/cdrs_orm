mod generated_some_struct {
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
        name: String,
    }
    impl SomeStruct {
        pub const SELECT_UNIQUE_QUERY : & 'static str = "select * from some_struct where id = ? and another_id = ? and cluster_key = ? and another_cluster_key = ?" ;
    }
    impl SomeStructPrimaryKey {
        pub fn select_unique_qv(&self) -> (&'static str, cdrs_tokio::query::QueryValues) {
            (SomeStruct::SELECT_UNIQUE_QUERY, self.where_clause())
        }
    }
    impl SomeStruct {
        pub const SELECT_ALL_QUERY: &'static str = "select * from some_struct";
        pub fn select_all_q() -> (&'static str, cdrs_tokio::query::QueryValues) {
            (
                SomeStruct::SELECT_ALL_QUERY,
                cdrs_tokio::query::QueryValues::SimpleValues(::alloc::vec::Vec::new()),
            )
        }
    }
    impl SomeStruct {
        pub const SELECT_ALL_COUNT_QUERY: &'static str = "select count(*) from some_struct";
        pub fn select_all_count_q() -> (&'static str, cdrs_tokio::query::QueryValues) {
            (
                SomeStruct::SELECT_ALL_COUNT_QUERY,
                cdrs_tokio::query::QueryValues::SimpleValues(::alloc::vec::Vec::new()),
            )
        }
    }
    pub struct SomeStructPrimaryKey {
        pub id: i32,
        pub another_id: i32,
        pub cluster_key: i32,
        pub another_cluster_key: i32,
    }
    impl ::core::marker::StructuralPartialEq for SomeStructPrimaryKey {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for SomeStructPrimaryKey {
        #[inline]
        fn eq(&self, other: &SomeStructPrimaryKey) -> bool {
            match *other {
                SomeStructPrimaryKey {
                    id: ref __self_1_0,
                    another_id: ref __self_1_1,
                    cluster_key: ref __self_1_2,
                    another_cluster_key: ref __self_1_3,
                } => match *self {
                    SomeStructPrimaryKey {
                        id: ref __self_0_0,
                        another_id: ref __self_0_1,
                        cluster_key: ref __self_0_2,
                        another_cluster_key: ref __self_0_3,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                            && (*__self_0_3) == (*__self_1_3)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &SomeStructPrimaryKey) -> bool {
            match *other {
                SomeStructPrimaryKey {
                    id: ref __self_1_0,
                    another_id: ref __self_1_1,
                    cluster_key: ref __self_1_2,
                    another_cluster_key: ref __self_1_3,
                } => match *self {
                    SomeStructPrimaryKey {
                        id: ref __self_0_0,
                        another_id: ref __self_0_1,
                        cluster_key: ref __self_0_2,
                        another_cluster_key: ref __self_0_3,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                            || (*__self_0_3) != (*__self_1_3)
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for SomeStructPrimaryKey {
        #[inline]
        fn clone(&self) -> SomeStructPrimaryKey {
            match *self {
                SomeStructPrimaryKey {
                    id: ref __self_0_0,
                    another_id: ref __self_0_1,
                    cluster_key: ref __self_0_2,
                    another_cluster_key: ref __self_0_3,
                } => SomeStructPrimaryKey {
                    id: ::core::clone::Clone::clone(&(*__self_0_0)),
                    another_id: ::core::clone::Clone::clone(&(*__self_0_1)),
                    cluster_key: ::core::clone::Clone::clone(&(*__self_0_2)),
                    another_cluster_key: ::core::clone::Clone::clone(&(*__self_0_3)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for SomeStructPrimaryKey {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                SomeStructPrimaryKey {
                    id: ref __self_0_0,
                    another_id: ref __self_0_1,
                    cluster_key: ref __self_0_2,
                    another_cluster_key: ref __self_0_3,
                } => {
                    let mut debug_trait_builder = f.debug_struct("SomeStructPrimaryKey");
                    let _ = debug_trait_builder.field("id", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("another_id", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("cluster_key", &&(*__self_0_2));
                    let _ = debug_trait_builder.field("another_cluster_key", &&(*__self_0_3));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for SomeStructPrimaryKey {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "SomeStructPrimaryKey",
                    false as usize + 1 + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "another_id",
                    &self.another_id,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "cluster_key",
                    &self.cluster_key,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "another_cluster_key",
                    &self.another_cluster_key,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for SomeStructPrimaryKey {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                    {
                        match __value {
                            "id" => _serde::__private::Ok(__Field::__field0),
                            "another_id" => _serde::__private::Ok(__Field::__field1),
                            "cluster_key" => _serde::__private::Ok(__Field::__field2),
                            "another_cluster_key" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                    {
                        match __value {
                            b"id" => _serde::__private::Ok(__Field::__field0),
                            b"another_id" => _serde::__private::Ok(__Field::__field1),
                            b"cluster_key" => _serde::__private::Ok(__Field::__field2),
                            b"another_cluster_key" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<SomeStructPrimaryKey>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = SomeStructPrimaryKey;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct SomeStructPrimaryKey",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<i32>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct SomeStructPrimaryKey with 4 elements",
                                        ),
                                    );
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<i32>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct SomeStructPrimaryKey with 4 elements",
                                        ),
                                    );
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<i32>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct SomeStructPrimaryKey with 4 elements",
                                        ),
                                    );
                                }
                            };
                        let __field3 =
                            match match _serde::de::SeqAccess::next_element::<i32>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct SomeStructPrimaryKey with 4 elements",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(SomeStructPrimaryKey {
                            id: __field0,
                            another_id: __field1,
                            cluster_key: __field2,
                            another_cluster_key: __field3,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<i32> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<i32> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<i32> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<i32> = _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "id",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<i32>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "another_id",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<i32>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "cluster_key",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<i32>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "another_cluster_key",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<i32>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("id") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("another_id") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("cluster_key") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("another_cluster_key") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(SomeStructPrimaryKey {
                            id: __field0,
                            another_id: __field1,
                            cluster_key: __field2,
                            another_cluster_key: __field3,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["id", "another_id", "cluster_key", "another_cluster_key"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "SomeStructPrimaryKey",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<SomeStructPrimaryKey>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[allow(clippy::from_over_into)]
    impl Into<cdrs_tokio::types::value::Bytes> for SomeStructPrimaryKey {
        fn into(self) -> cdrs_tokio::types::value::Bytes {
            serde_json::to_string(&self).unwrap().into()
        }
    }
    impl cdrs_tokio::types::prelude::TryFromRow for SomeStructPrimaryKey {
        fn try_from_row(cdrs: cdrs_tokio::types::rows::Row) -> cdrs_tokio::Result<Self> {
            use cdrs_tokio::frame::TryFromUDT;
            use cdrs_tokio::types::from_cdrs::FromCDRSByName;
            use cdrs_tokio::types::AsRustType;
            Ok(SomeStructPrimaryKey {
                id: i32::from_cdrs_r(&cdrs, " id ".trim())?,
                another_id: i32::from_cdrs_r(&cdrs, " another_id ".trim())?,
                cluster_key: i32::from_cdrs_r(&cdrs, " cluster_key ".trim())?,
                another_cluster_key: i32::from_cdrs_r(&cdrs, " another_cluster_key ".trim())?,
            })
        }
    }
    impl SomeStruct {
        pub fn primary_key(&self) -> SomeStructPrimaryKey {
            SomeStructPrimaryKey {
                id: self.id.clone(),
                another_id: self.another_id.clone(),
                cluster_key: self.cluster_key.clone(),
                another_cluster_key: self.another_cluster_key.clone(),
            }
        }
    }
    impl SomeStructPrimaryKey {
        pub const WHERE_CLAUSE_PK: &'static str =
            " where id = ? and another_id = ? and cluster_key = ? and another_cluster_key = ?";
        pub fn where_clause(&self) -> cdrs_tokio::query::QueryValues {
            cdrs_tokio::query::QueryValues::SimpleValues(self.where_clause_raw())
        }
        pub fn where_clause_raw(&self) -> Vec<cdrs_tokio::types::value::Value> {
            use std::iter::FromIterator;
            let mut query_values: Vec<cdrs_tokio::types::value::Value> = Vec::new();
            query_values.push(cdrs_tokio::types::value::Value::new_normal(self.id.clone()));
            query_values.push(cdrs_tokio::types::value::Value::new_normal(
                self.another_id.clone(),
            ));
            query_values.push(cdrs_tokio::types::value::Value::new_normal(
                self.cluster_key.clone(),
            ));
            query_values.push(cdrs_tokio::types::value::Value::new_normal(
                self.another_cluster_key.clone(),
            ));
            query_values
        }
    }
    impl SomeStruct {
        pub const INSERT_QUERY : & 'static str = "insert into some_struct (id, another_id, cluster_key, another_cluster_key, name) values (?, ?, ?, ?, ?)" ;
        pub fn query_values(&self) -> cdrs_tokio::query::QueryValues {
            cdrs_tokio::query::QueryValues::SimpleValues(self.simple_values())
        }
        pub fn insert_qv(&self) -> (&'static str, cdrs_tokio::query::QueryValues) {
            (SomeStruct::INSERT_QUERY, self.query_values())
        }
    }
    impl SomeStruct {
        fn simple_values(&self) -> Vec<cdrs_tokio::types::value::Value> {
            let mut values: Vec<cdrs_tokio::types::value::Value> = Vec::new();
            values.push(cdrs_tokio::types::value::Value::new_normal(self.id.clone()));
            values.push(cdrs_tokio::types::value::Value::new_normal(
                self.another_id.clone(),
            ));
            values.push(cdrs_tokio::types::value::Value::new_normal(
                self.cluster_key.clone(),
            ));
            values.push(cdrs_tokio::types::value::Value::new_normal(
                self.another_cluster_key.clone(),
            ));
            values.push(cdrs_tokio::types::value::Value::new_normal(
                self.name.clone(),
            ));
            values
        }
    }
    impl SomeStruct {
        pub const INSERT_QUERY_USING_TTL : & 'static str = "insert into some_struct (id, another_id, cluster_key, another_cluster_key, name) values (?, ?, ?, ?, ?) using ttl ?" ;
        pub fn insert_qv_using_ttl(
            &self,
            using_ttl: i32,
        ) -> (&'static str, cdrs_tokio::query::QueryValues) {
            let mut values = self.simple_values();
            values.push(cdrs_tokio::types::value::Value::new_normal(using_ttl));
            let qv = cdrs_tokio::query::QueryValues::SimpleValues(values);
            (SomeStruct::INSERT_QUERY_USING_TTL, qv)
        }
    }
    pub enum SomeStructUpdatableColumns {
        Name(String),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for SomeStructUpdatableColumns {
        #[inline]
        fn clone(&self) -> SomeStructUpdatableColumns {
            match (&*self,) {
                (&SomeStructUpdatableColumns::Name(ref __self_0),) => {
                    SomeStructUpdatableColumns::Name(::core::clone::Clone::clone(&(*__self_0)))
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for SomeStructUpdatableColumns {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&SomeStructUpdatableColumns::Name(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Name");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl SomeStructPrimaryKey {
        pub fn update_dyn_qv(
            &self,
            dyn_column: SomeStructUpdatableColumns,
        ) -> (&'static str, cdrs_tokio::query::QueryValues) {
            match dyn_column {
                SomeStructUpdatableColumns::Name(val) => self.update_qv_name(val),
            }
        }
    }
    impl SomeStructPrimaryKey {
        pub fn update_qv(
            &self,
            name: std::option::Option<String>,
        ) -> std::option::Option<(String, cdrs_tokio::query::QueryValues)> {
            let mut to_update: Vec<String> = std::vec::Vec::new();
            let mut qv: Vec<cdrs_tokio::types::value::Value> = std::vec::Vec::new();
            if let Some(s) = name {
                to_update.push({
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["", " = ?"],
                        &match (&"name",) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ));
                    res
                });
                qv.push(cdrs_tokio::types::value::Value::new_normal(s));
            }
            if to_update.is_empty() {
                return None;
            }
            let to_update: String = to_update.join(", ");
            let to_update = {
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["set "],
                    &match (&to_update,) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ));
                res
            };
            let values = self.where_clause_raw();
            qv.extend(values);
            let string = {
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["update ", " ", ""],
                    &match (
                        &"some_struct",
                        &to_update,
                        &SomeStructPrimaryKey::WHERE_CLAUSE_PK,
                    ) {
                        (arg0, arg1, arg2) => [
                            ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(arg2, ::core::fmt::Display::fmt),
                        ],
                    },
                ));
                res
            };
            Some((string, cdrs_tokio::query::QueryValues::SimpleValues(qv)))
        }
    }
    impl SomeStructPrimaryKey {
        pub fn update_multiple_qv(
            &self,
            vec: std::vec::Vec<SomeStructUpdatableColumns>,
        ) -> (String, cdrs_tokio::query::QueryValues) {
            if !!vec.is_empty() {
                ::core::panicking::panic("assertion failed: !vec.is_empty()")
            };
            let mut query = ::alloc::vec::Vec::new();
            let mut values: std::vec::Vec<cdrs_tokio::types::value::Value> =
                ::alloc::vec::Vec::new();
            for ident in vec {
                match ident {
                    SomeStructUpdatableColumns::Name(val) => {
                        query.push("name = ?");
                        values.push(val.into());
                    }
                }
            }
            let columns_to_update: String = query.join(", ");
            let update_statement = {
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["update ", " set ", ""],
                    &match (
                        &"some_struct",
                        &columns_to_update,
                        &SomeStructPrimaryKey::WHERE_CLAUSE_PK,
                    ) {
                        (arg0, arg1, arg2) => [
                            ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(arg2, ::core::fmt::Display::fmt),
                        ],
                    },
                ));
                res
            };
            values.extend(self.where_clause_raw());
            let query_values = cdrs_tokio::query::QueryValues::SimpleValues(values);
            (update_statement, query_values)
        }
    }
    impl SomeStruct {
        pub const UPDATE_NAME_QUERY : & 'static str = "update some_struct set name = ? where id = ? and another_id = ? and cluster_key = ? and another_cluster_key = ?" ;
    }
    impl SomeStructPrimaryKey {
        pub fn update_qv_name(
            &self,
            name: String,
        ) -> (&'static str, cdrs_tokio::query::QueryValues) {
            let mut values = self.where_clause_raw();
            values.insert(0, cdrs_tokio::types::value::Value::new_normal(name));
            (
                SomeStruct::UPDATE_NAME_QUERY,
                cdrs_tokio::query::QueryValues::SimpleValues(values),
            )
        }
    }
    impl SomeStruct {
        pub const DELETE_UNIQUE_QUERY : & 'static str = "delete from some_struct where id = ? and another_id = ? and cluster_key = ? and another_cluster_key = ?" ;
    }
    impl SomeStructPrimaryKey {
        pub fn delete_unique_qv(&self) -> (&'static str, cdrs_tokio::query::QueryValues) {
            (SomeStruct::DELETE_UNIQUE_QUERY, self.where_clause())
        }
    }
    impl SomeStruct {
        pub const TRUNCATE_QUERY: &'static str = "truncate some_struct";
        pub fn truncate_q() -> (&'static str, cdrs_tokio::query::QueryValues) {
            (
                SomeStruct::TRUNCATE_QUERY,
                cdrs_tokio::query::QueryValues::SimpleValues(::alloc::vec::Vec::new()),
            )
        }
    }
}
