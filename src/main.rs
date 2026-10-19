use std::io::{self, Write, Read};
use std::process::Command;



fn main() {
    loop {
        print!("$ ");

    io::stdout().flush().unwrap(); 

    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let input = input.trim();

    if input.is_empty() {
        continue;
    }

    let parts: Vec<&str> = input.split_whitespace().collect();
    let command = parts[0];
    let args = &parts[1..];

    match command {
        "exit" => {
            break;
        }

        "echo" => {
            println!("{}", args.join(" "));
        }

        "type" => {
            if let Some(target) = args.first() {
                match *target {
                     "echo" |  "type" | "exit"   => {
            println!("{type_string} is a shell builtin")
           } 

           _ => {
    let path = std::env::var("PATH").unwrap_or_default();
    let delimiter = if cfg!(windows) { ';' } else { ':' };
    let path_dir = path.split(delimiter);
    let mut found = false;

    for dir in path_dir {
        let full_path = format!("{dir}/{type_string}");
        let file_path = std::path::Path::new(&full_path);

        if file_path.exists() {
            // Enforce execute permissions ONLY on Unix/Linux (CodeCrafters runner).
            // On Windows (local testing), this block is skipped entirely.
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Ok(metadata) = std::fs::metadata(file_path) {
                    if metadata.permissions().mode() & 0o111 == 0 {
                        continue; // Skip files without execute permissions
                    }
                } else {
                    continue;
                }
            }

            println!("{type_string} is {full_path}");
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
        }

        
    }

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
           
         }


     }
     
     else {
        println!("{new_string}: command not found")

     }

    


    }

    
}
