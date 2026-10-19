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
                        "echo" | "type" | "exit" | "pwd" => {
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