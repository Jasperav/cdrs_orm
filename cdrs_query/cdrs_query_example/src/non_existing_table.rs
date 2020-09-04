use proc_query::control;

fn main() {
    control!("select * from idontexist");
}