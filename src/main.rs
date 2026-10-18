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
     } else if new_string.starts_with("echo ") {
        

        let echo_string =  &new_string[5..].trim();

        println!("{echo_string}");
     } else if new_string.starts_with("type ") {
         let type_string = &new_string[5..].trim();

         if type_string == "echo" | "type" |"exit" {
            println!("{type_string}"is a shell builtin)
         }


     }
     
     else {
        println!("{new_string}: command not found")

     }

    


    }

    
}
