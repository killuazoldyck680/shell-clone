use std::io::{self, Write, Read};



fn main() {
    loop {
        print!("$ ");

    io::stdout().flush().unwrap(); 

    let mut passed_string = String::new();

     io::stdin().read_line(&mut passed_string);

     let new_string = passed_string.trim();

     if new_string.starts_with("exit") {
        break;
     } else if new_string.starts_with("echo") {
        new_string.drain(0..4)

        let echo_string =  new_string.trim();
     }
     
     else {
        println!("{new_string}: command not found")

     }

    


    }

    
}
