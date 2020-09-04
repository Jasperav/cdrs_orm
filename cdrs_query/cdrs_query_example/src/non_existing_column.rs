use proc_query::control;

fn main() {
    let a = 1;

    control!("select * from test_table where b = ? and c = 2 and f = 2", a);
}