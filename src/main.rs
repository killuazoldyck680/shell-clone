use std::io::{self, Write, Read};



fn main() {
    loop {
        print!("$ ");

    io::stdout().flush().unwrap();

    let mut passed_string = String::new();

    let new_string = io::stdin().read_line(&mut passed_string).trim();

    println!({command}: command not found)



    }

    
}
