use std::io::{self, Write};
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

        let mut args = Vec::new();

        let mut current_arg = String::new();
        
        let mut in_single_quote = false;


        for c in input.chars() {
            
        if in_single_quote {
            if c == '\'' {
                in_single_quote = false;
            } else {
                current_arg.push(c);
            }
        } else {
            if c == '\'' {
                in_single_quote = true;
            } else if c.is_whitespace()  {
                if current_arg.is_empty() {
                
            }

                args.push(current_arg);

                current_arg = String::new();

                 
            }  else {
                current_arg.push(c);
            }
        }
        }

        if !current_arg.is_empty() { args.push(current_arg); }

        if args.is_empty() {
            continue;
        }

        let command = args[0].as_str();
        let args = &args[1..];

        
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
                        "echo" | "type" | "exit" | "pwd" | "cd" => {
                            println!("{} is a shell builtin", target);
                        } 
                        _ => {
                            let path = std::env::var("PATH").unwrap_or_default();
                            let delimiter = if cfg!(windows) { ';' } else { ':' };
                            let path_dir = path.split(delimiter);
                            let mut found = false;

                            for dir in path_dir {
                                let full_path = format!("{dir}/{target}");
                                let file_path = std::path::Path::new(&full_path);

                                if file_path.exists() {
                                    #[cfg(unix)]
                                    {
                                        use std::os::unix::fs::PermissionsExt;
                                        if let Ok(metadata) = std::fs::metadata(file_path) {
                                            if metadata.permissions().mode() & 0o111 == 0 {
                                                continue; 
                                            }
                                        } else {
                                            continue;
                                        }
                                    }

                                    println!("{} is {}", target, full_path);
                                    found = true;
                                    break;
                                }
                            }

                            if !found {
                                println!("{target}: not found");
                            }
                        }
                    }
                }
            }

            "pwd" => {
               let cur_dir =  std::env::current_dir().expect("Failed to get current directory");

               println!("{}", cur_dir.display());


            }

            "cd" => {
                if let Some(target) = args.first() {
        // Resolve ~ to HOME (Unix/Mac) or USERPROFILE (Windows), or keep the original target
        let resolved_target = if *target == "~" {
            if let Ok(home) = std::env::var("HOME") {
                home
            } else if let Ok(profile) = std::env::var("USERPROFILE") {
                profile
            } else {
                println!("cd: ~: Home directory not set");
                return;
            }
        } else {
            target.to_string()
        };
                    let cur_dir = std::env::set_current_dir(&resolved_target);

                    match cur_dir {
                  Ok(()) => {}

                  Err(_) => {
                    println!("cd: {target}: No such file or directory")
                  }  
                }
                }

                
                
            }

            // This is the separate fallback arm for external programs
            _ => {
                // Replace your external program lookup block (in the `_ =>` match arm) with this:

let path = std::env::var("PATH").unwrap_or_default();
let delimiter = if cfg!(windows) { ';' } else { ':' };
let mut executable_path = None;

for dir in path.split(delimiter) {
    let mut full_path = format!("{dir}/{command}");
    let file_path = std::path::Path::new(&full_path);
    let mut exists = file_path.exists();

    // Windows local fallback: check for .exe if the base file doesn't exist
    if !exists && cfg!(windows) {
        let exe_path = format!("{dir}/{command}.exe");
        if std::path::Path::new(&exe_path).exists() {
            full_path = exe_path;
            exists = true;
        }
    }

    if exists {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = std::fs::metadata(&full_path) {
                if metadata.permissions().mode() & 0o111 == 0 {
                    continue;
                }
            } else {
                continue;
            }
        }
        executable_path = Some(full_path);
        break;
    }
}

if let Some(exec_path) = executable_path {
    match Command::new(&exec_path).args(args).status() {
        Ok(_status) => {}
        Err(_) => {
            println!("{}: failed to execute", command);
        }
    }
} else {
    println!("{}: not found", command);
}
            }
        }
    }
}