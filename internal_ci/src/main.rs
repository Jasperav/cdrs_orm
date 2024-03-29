use std::env::current_dir;
use std::io::Write;

fn main() {
    dotenv::dotenv().unwrap();

    let workflows = current_dir().unwrap().join("../.github").join("workflows");
    let (packages, images) = cdrs_ci::setup(&workflows);
    let template_string = String::from_utf8_lossy(include_bytes!("template.yml")).to_string();
    let mut fmt_and_fix = true;

    for (container, image) in images.iter() {
        let mut yml = cdrs_ci::write_template(&workflows, container, image, &template_string);
        let whitespace = "      ";

        for package in packages.iter() {
            if package == "test_derived_equals" {
                writeln!(yml, "{}- name: Install cargo expand", whitespace).unwrap();
                writeln!(yml, "{}  run: cargo install cargo-expand", whitespace).unwrap();
            }

            cdrs_ci::write_tests(&mut yml, whitespace, package, fmt_and_fix, "internal_ci");
        }

        fmt_and_fix = false;
    }
}
