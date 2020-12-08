mod generated_some_struct;

#[cfg(test)]
mod test {
    use std::fs::{create_dir, remove_dir_all, File};
    use std::io::{Read, Write};
    use std::process::Command;

    #[test]
    fn test_equals() {
        let file = std::env::current_dir().unwrap().join("src");

        let output = Command::new("cargo")
            .env("TEST_CDRS_DB_KEYSPACE_KEY", "test_keyspace_for_testing")
            .arg("expand")
            .arg("generated_some_struct")
            .output()
            .unwrap();

        if output.stdout.is_empty() {
            panic!("{:#?}", output);
        }

        let rs = "generated_some_struct.rs";
        let temp_dir = file.join("temp");
        let _ = remove_dir_all(&temp_dir);

        create_dir(&temp_dir).unwrap();

        let mut f = File::create(temp_dir.join(rs)).unwrap();

        f.write(&output.stdout).unwrap();

        let mut source_str = String::new();
        File::open(file.join("gen").join(rs))
            .unwrap()
            .read_to_string(&mut source_str)
            .unwrap();

        let mut target_str = String::new();
        File::open(temp_dir.join(rs))
            .unwrap()
            .read_to_string(&mut target_str)
            .unwrap();

        // // TODO: After Rust 1.50.0 is stable, remove this code
        // // This is needed to make the test pass on Github CI
        // target_str = target_str.replace(
        //     "::core::panicking::panic(\"assertion failed: !vec.is_empty()\")",
        //     "{::std::rt::begin_panic(\"assertion failed: !vec.is_empty()\")}",
        // );

        // Remove weird auto indenting when a file is in the module system
        let replaced = |s: &str| {
            s.replace("\n", "")
                .replace("\t", "")
                .replace(" ", "")
                .trim()
                .to_owned()
        };

        let target_replaced = replaced(&target_str);

        if replaced(&source_str) == target_replaced {
            // Ok, equal
            return;
        } else {
            panic!("Not equal, target was: {}", target_str);
        }
    }
}
