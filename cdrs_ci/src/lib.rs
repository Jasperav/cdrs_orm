use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

pub fn setup(workflow_dir: &Path) -> (Vec<String>, [(&'static str, &'static str); 2]) {
    std::env::set_var("RUST_LOG", "info");
    // Ignore error
    let _ = env_logger::try_init();

    log::info!("Checking for uncommited changes...");
    // Check if there are no uncommited changes, this is illegal.
    // This is because later on, cargo fmt and fix will run. It can occur it will try to fix
    // a file twice, and if the first the the file has been changed by the cargo commands,
    // it cargo will complain about uncommitted changes. This is fixed by passing the flag
    // --allow-dirty, but to make sure not to overwrite non-cargo related changes, check the diff here.
    let output = Command::new("git").arg("diff").output().unwrap();

    assert!(output.status.success());

    // if !output.stdout.is_empty() {
    //     panic!("Uncommitted changes, please commit them first: {:#?}", String::from_utf8(output.stdout).unwrap());
    // }

    log::info!("Removing old workflow folders");
    // Ignore results, since maybe the folders do not exists atm
    let _ = std::fs::remove_dir_all(workflow_dir);
    let _ = std::fs::create_dir_all(workflow_dir);

    let metadata = cargo_metadata::MetadataCommand::new()
        .no_deps()
        .exec()
        .unwrap();
    let packages = metadata
        .workspace_members
        .into_iter()
        .map(|w| w.repr)
        .map(|p| p[..p.find(' ').unwrap()].to_string())
        .collect::<Vec<_>>();

    let images = [("cassandra", "cassandra"), ("scylladb", "scylladb/scylla")];

    (packages, images)
}

pub fn write_template(
    path_to_workflow: &Path,
    container: &str,
    image: &str,
    template: &str,
) -> File {
    let mut yml = File::create(path_to_workflow.join(format!("{}.yml", container))).unwrap();

    let template = template
        .replace("$IMAGE$", image)
        .replace("$CONTAINER$", container);

    writeln!(yml, "# This file is generated, don't edit\n").unwrap();
    writeln!(yml, "{}", template).unwrap();

    yml
}

pub fn write_tests(yml: &mut File, whitespace: &str, package: &str, fmt_and_fix: bool) {
    log::info!("Writing tests for package: {}", package);

    writeln!(yml, "{}- name: Build {}", whitespace, package).unwrap();
    writeln!(
        yml,
        "{}  run: cargo build --package {} --verbose",
        whitespace, package
    )
    .unwrap();

    writeln!(yml, "{}- name: Run tests {}", whitespace, package).unwrap();
    writeln!(
        yml,
        "{}  run: cargo test --package {} --verbose -- --test-threads=1",
        whitespace, package
    )
    .unwrap();

    writeln!(yml, "{}- name: Check clippy {}", whitespace, package).unwrap();
    writeln!(
        yml,
        "{}  run: cargo +nightly clippy --package {} -- -D warnings",
        whitespace, package
    )
    .unwrap();

    writeln!(yml, "{}- name: Check fmt {}", whitespace, package).unwrap();
    writeln!(
        yml,
        "{}  run: cargo +nightly fmt --package {} -- --check",
        whitespace, package
    )
    .unwrap();

    writeln!(yml).unwrap();

    if fmt_and_fix {
        log::info!("Verifying package {}", package);

        let mut env = HashMap::new();

        env.insert(
            "TEST_CDRS_DB_KEYSPACE_KEY".to_string(),
            "test_keyspace_for_testing".to_string(),
        );

        // Format and fix project directly
        log::info!("Cleaning...");
        execute_cargo_command("clean", package, None, &env);
        log::info!("Building...");
        execute_cargo_command("build", package, None, &env);
        log::info!("Formatting...");
        execute_cargo_command("fmt", package, None, &env);
        log::info!("Fixing...");
        execute_cargo_command(
            "fix",
            package,
            Some(vec!["--all-features", "--allow-dirty"]),
            &env,
        );
        log::info!("Testing...");
        execute_cargo_command(
            "test",
            package,
            Some(vec!["--verbose", "--", "--test-threads=1"]),
            &env,
        );

        log::info!("Done verifying package");
    }

    // TODO: This does not work yet
    // Command::new("cargo")
    //     .env("TEST_CDRS_DB_KEYSPACE_KEY", "test_keyspace_for_testing")
    //     .args(&["build".to_string(), "--package".to_string(), format!("{}", package)])
    //     .output()
    //     .unwrap();
    //
    // // Use clippy to check for any errors
    // let clippy_out = Command::new("cargo")
    //     .env("TEST_CDRS_DB_KEYSPACE_KEY", "test_keyspace_for_testing")
    //     .args(&[
    //         "+nightly".to_string(),
    //         "clippy".to_string(),
    //         "--package".to_string(),
    //         format!("{}", package),
    //         "--".to_string(),
    //         "-D".to_string(),
    //         "warnings".to_string(),
    //     ])
    //     .output()
    //     .unwrap();
    //
    // if !clippy_out.stderr.is_empty() && !clippy_out.status.success() {
    //     let error = String::from_utf8(clippy_out.stderr).unwrap();
    //
    //     panic!("{}", error);
    // }
}

pub fn execute_cargo_command(
    command: &str,
    package: &str,
    extra_command: Option<Vec<&str>>,
    envs: &HashMap<String, String>,
) {
    let mut args = vec![
        "+nightly".to_string(),
        command.to_string(),
        "--package".to_string(),
        package.to_string(),
    ];

    if let Some(command) = extra_command {
        let string_vec = command
            .into_iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>();

        args.extend(string_vec);
    }

    let output = Command::new("cargo")
        .envs(envs)
        .args(&args)
        .output()
        .unwrap();

    if !output.stderr.is_empty() && !output.status.success() {
        panic!("{:#?}", String::from_utf8(output.stderr).unwrap());
    }

    assert!(output.status.success());
}
