use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let current_word = args.get(2).map(|s| s.as_str()).unwrap_or("");

    for candidate in ["checkout", "cherry-pick"] {
        if candidate.starts_with(current_word) {
            println!("{}", candidate);
        }
    }
}