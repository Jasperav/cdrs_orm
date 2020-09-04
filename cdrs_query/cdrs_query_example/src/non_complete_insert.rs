use proc_query::control;

fn main() {
    let a = 1;

    control!("insert into test_table (b, c, d, a) values (2, 3, 4, 5)");
}