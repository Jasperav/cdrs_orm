mod generated_some_struct;

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::fs::{create_dir, remove_dir_all, File};
    use std::io::{Read, Write};
    use std::process::Command;

    #[test]
    fn test_equals() {
        env_logger::init();
        dotenv::dotenv().unwrap();

        let file = std::env::current_dir().unwrap().join("src");
        let envs: HashMap<_, _> = std::env::vars().collect();
        let output = Command::new("cargo")
            .envs(envs)
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

        // Remove weird auto indenting when a file is in the module system
        let replaced = |s: &str| {
            s.replace("\n", "")
                .replace("\t", "")
                .replace(" ", "")
                .trim()
                // Needed because else still newlines are shown
                .split_whitespace()
                .into_iter()
                .collect::<Vec<_>>()
                .join("")
        };

        let target_replaced = replaced(&target_str);
        let replaced = replaced(&source_str);

        if replaced == target_replaced {
            // Ok, equal
        } else {
            panic!(
                "Not equal, target: \n{}\n, replaced: \n{}",
                target_replaced, replaced
            );
        }
    }
}
