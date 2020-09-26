use std::env::{current_dir, set_current_dir};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

pub fn setup(workflow_dir: &Path) -> (Vec<String>, [(&'static str, &'static str); 2]) {
    // Ignore error
    let _ = env_logger::try_init();

    if let Ok(allow_dirty) = std::env::var("ALLOW_DIRTY_CI") {
        if &allow_dirty != "1" {
            log::info!("Checking for uncommited changes...");
            // Check if there are no uncommited changes, this is illegal.
            // This is because later on, cargo fmt and fix will run. It can occur it will try to fix
            // a file twice, and if the first the the file has been changed by the cargo commands,
            // it cargo will complain about uncommitted changes. This is fixed by passing the flag
            // --allow-dirty, but to make sure not to overwrite non-cargo related changes, check the diff here.
            let output = Command::new("git").arg("diff").output().unwrap();

            assert!(output.status.success());

            if !output.stdout.is_empty() {
                panic!(
                    "Uncommitted changes, please commit them first: {}",
                    String::from_utf8(output.stdout).unwrap()
                );
            }
        }
    }

    log::info!("Removing old workflow folders");
    // Ignore results, since maybe the folders do not exists atm
    let _ = std::fs::remove_dir_all(workflow_dir);
    let _ = std::fs::create_dir_all(workflow_dir);

    log::info!("Changing current dir because else clippy won't find anything even if there is something (bug in --package argument?)");
    let current_dir = current_dir().unwrap();
    let new_dir = current_dir.parent().unwrap();
    log::info!("New dir: {:#?}", new_dir);
    set_current_dir(new_dir).unwrap();

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

pub fn write_tests(
    yml: &mut File,
    whitespace: &str,
    package: &str,
    fmt_and_fix: bool,
    package_name_currently_executing: &str,
) {
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

        // Execute this before running a command, because there are bugs preventing cargo to run
        // checks the second time without a rebuild/touch (https://stackoverflow.com/questions/31125226/is-it-possible-to-have-cargo-always-show-warnings)
        let clean_and_build = || {
            // Format and fix project directly
            // Only clean if the current package isn't the CI package, since that isn't possible
            // (removing the executable while executing the program)
            if package != package_name_currently_executing {
                log::info!("Cleaning...");
                execute_command("clean", package, None);
                log::info!("Building...");
                execute_command("build", package, None);
            }
        };

        clean_and_build();
        log::info!("Formatting...");
        execute_command("fmt", package, None);

        clean_and_build();
        log::info!("Fixing...");
        execute_command(
            "fix",
            package,
            Some(vec!["--all-features", "--allow-dirty"]),
        );
        // No clean and build needed for this
        log::info!("Testing...");
        execute_command(
            "test",
            package,
            // Test threads is 1, because in tests sometimes tables be added, keyspace are being recreated,
            // if that all goes down at the same time, error will be thrown
            Some(vec!["--verbose", "--", "--test-threads=1"]),
        );

        clean_and_build();
        log::info!("Running clippy...");
        execute_command("clippy", package, Some(vec!["--", "-D", "warnings"]));

        log::info!("Done verifying package");
    }
}

pub fn execute_command(command: &str, package: &str, extra_command: Option<Vec<&str>>) {
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

    let output = Command::new("cargo").args(&args).output().unwrap();

    if !output.stderr.is_empty() && !output.status.success() {
        panic!("{}", String::from_utf8(output.stderr).unwrap());
    }

    assert!(output.status.success());
}
