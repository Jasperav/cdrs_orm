use crate::CAMEL_CASE_TO_DB_SNAKE_CASE_KEY;

pub fn struct_name_to_db_table_name(struct_name: &str) -> String {
    if camel_case_to_db_snake_case() {
        let mut db_table_name = String::new();

        for (index, c) in struct_name.chars().rev().enumerate() {
            let is_uppercase = c.is_uppercase();
            db_table_name.insert(0, c.to_ascii_lowercase());

            if is_uppercase && index != struct_name.len() - 1 {
                db_table_name.insert(0, '_');
            }
        }

        db_table_name
    } else {
        struct_name.to_string()
    }
}

pub fn camel_case_to_db_snake_case() -> bool {
    match std::env::var(CAMEL_CASE_TO_DB_SNAKE_CASE_KEY) {
        Ok(o) => {
            let r: &str = o.as_ref();

            r == "1"
        }
        Err(_) => false,
    }
}

pub fn table_name_to_struct_name(s: &str) -> String {
    if camel_case_to_db_snake_case() {
        snake_case_to_upper_camel_case(s)
    } else {
        s.to_string()
    }
}

pub fn snake_case_to_upper_camel_case(s: &str) -> String {
    capitalize_letter_after_symbol(&capitalized(s), '_')
}

// https://stackoverflow.com/a/38406885/7715250
pub fn capitalized(s: &str) -> String {
    let mut c = s.chars();

    c.next().unwrap().to_uppercase().collect::<String>() + c.as_str()
}

pub fn capitalize_letter_after_symbol(s: &str, symbol: char) -> String {
    let mut c = s.chars();
    let mut new_string = String::new();

    loop {
        match c.next() {
            None => return new_string,
            Some(some_char) => {
                if some_char == symbol {
                    let next = c.next().unwrap();

                    new_string.push(next.to_ascii_uppercase());
                } else {
                    new_string.push(some_char)
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::CAMEL_CASE_TO_DB_SNAKE_CASE_KEY;
    use std::env;

    #[test]
    fn test_capitalized() {
        assert_eq!("A", capitalized("a"));
        assert_eq!("ABc", capitalized("aBc"));
    }

    #[test]
    fn test_capitalize_letter_after_symbol() {
        assert_eq!("aBcC", capitalize_letter_after_symbol("a_bc_c", '_'))
    }

    #[test]
    fn test_struct_name_to_db_table_name() {
        let current_val = env::var(CAMEL_CASE_TO_DB_SNAKE_CASE_KEY);

        env::remove_var(CAMEL_CASE_TO_DB_SNAKE_CASE_KEY);

        let struct_name = "SomeStructName";

        assert_eq!(struct_name, struct_name_to_db_table_name(struct_name));

        env::set_var(CAMEL_CASE_TO_DB_SNAKE_CASE_KEY, "0");

        assert_eq!(struct_name, struct_name_to_db_table_name(struct_name));

        env::set_var(CAMEL_CASE_TO_DB_SNAKE_CASE_KEY, "1");

        assert_eq!(
            "some_struct_name",
            struct_name_to_db_table_name(struct_name)
        );

        if let Ok(e) = current_val {
            env::set_var(CAMEL_CASE_TO_DB_SNAKE_CASE_KEY, e);
        }
    }
}
