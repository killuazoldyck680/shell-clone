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
         let type_string = new_string[5..].trim();

         match type_string {
            "echo" |  "type" | "exit"   => {
            println!("{type_string} is a shell builtin")
           } 

           _ => {
            let path  = std::env::var("PATH").unwrap_or_default();

            let path_dir = path.split(':');

            let mut found = false;

            for dir in path_dir {
                let full_path = format!("{dir}/{type_string}");

                if std::path::Path::new(&full_path).exists() {
                    println!("{type_string} is {full_path}");
                    #[cfg(unix)]
                    {
                        
                    }

                    found = true;
                    break;
                }
            }

            if !found {
                println!("{type_string}: not found");
            }


            
           }
         }


     }
     
     else {
        println!("{new_string}: command not found")

     }

    


    }

    
}
