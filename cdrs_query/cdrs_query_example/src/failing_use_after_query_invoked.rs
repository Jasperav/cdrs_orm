use proc_query::control;

// Checks if the argument isn't cloned but moved
fn main() {
    let argument = "".to_string();

    control!("select * from test_table where b = ? and c = ?", argument, argument);
}