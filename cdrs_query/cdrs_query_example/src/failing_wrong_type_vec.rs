use proc_query::control;

fn main() {
    let a = 1;

    control!("select * from test_table where b = 1 and c in ?", a);
}
