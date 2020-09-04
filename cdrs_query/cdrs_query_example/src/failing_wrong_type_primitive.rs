use proc_query::control;

fn main() {
    let b = "";

    control!("select * from test_table where b = ? and c = ?", b, b);
}