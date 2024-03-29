use std::collections::HashMap;
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
    log::info!("Verifying package {}", package);

    // Execute this before running a command, because there are bugs preventing cargo to run
    // checks the second time without a rebuild/touch (https://stackoverflow.com/questions/31125226/is-it-possible-to-have-cargo-always-show-warnings)
    let clean_and_build = |yml: &mut File| {
        // Format and fix project directly
        // Only clean if the current package isn't the CI package, since that isn't possible
        // (removing the executable while executing the program)
        if package != package_name_currently_executing {
            execute_command(fmt_and_fix, yml, whitespace, "clean", package, None, true);
            execute_command(
                fmt_and_fix,
                yml,
                whitespace,
                "build",
                package,
                Some(vec!["--jobs=1"]),
                true,
            );
        }
    };

    clean_and_build(yml);
    execute_command(fmt_and_fix, yml, whitespace, "fmt", package, None, false);
    execute_command(
        fmt_and_fix,
        yml,
        whitespace,
        "fix",
        package,
        Some(vec![
            "--all-features",
            "--allow-dirty",
            "--allow-staged",
            "--jobs=1",
        ]),
        false,
    );
    execute_command(
        fmt_and_fix,
        yml,
        whitespace,
        "test",
        package,
        // Test threads is 1, because in tests sometimes tables be added, keyspace are being recreated,
        // if that all goes down at the same time, error will be thrown
        Some(vec!["--verbose", "--", "--test-threads=1"]),
        true,
    );

    // Clean and rebuild again, it's needed because else clippy doesn't pick up anything
    clean_and_build(yml);
    execute_command(
        fmt_and_fix,
        yml,
        whitespace,
        "clippy",
        package,
        Some(vec!["--", "-D", "warnings"]),
        true,
    );

    writeln!(yml).unwrap();

    log::info!("Done verifying package");
}

pub fn execute_command(
    fmt_and_fix: bool,
    yml: &mut File,
    whitespace: &str,
    command: &str,
    package: &str,
    extra_command: Option<Vec<&str>>,
    for_ci: bool,
) {
    let mut args = vec![
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

    // TODO: Not sure how to get rid of the double command initialization, but args returns a &mut
    // and I am not sure how reuse the same command
    let formatted = format!("{:#?}", Command::new("cargo").args(&args));
    // Remove the quotes
    let no_quotes = formatted.replace("\"", "");
    log::debug!("Executing command: {}", no_quotes);

    if for_ci {
        writeln!(yml, "{}- name: {} {}", whitespace, command, package).unwrap();
        writeln!(yml, "{}  run: {}", whitespace, no_quotes).unwrap();
    }

    if fmt_and_fix {
        let envs: HashMap<_, _> = std::env::vars().collect();

        let output = Command::new("cargo")
            .args(&args)
            .envs(envs)
            .output()
            .unwrap();

        if !output.stderr.is_empty() && !output.status.success() {
            panic!("{}", String::from_utf8(output.stderr).unwrap());
        }

        assert!(output.status.success());
    }
}
