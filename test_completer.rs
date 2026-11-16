use std::env;

fn main() {
    let comp_line = env::var("COMP_LINE").unwrap_or_default();
    let comp_point = env::var("COMP_POINT").unwrap_or_default();

    if comp_line == "git ad" && comp_point == "6" {
        println!("add");
    }
}