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

        let mut in_double_quote = false;

        let mut is_escaped = false;

        let mut redirect_file: Option<std::fs::File> = None;

        let mut stderr_redirect_file : Option<std::fs::File> = None;


        let mut chars = input.chars().peekable();
        while let Some(c) = chars.next() {
            if in_single_quote {
                if c == '\'' {
                    in_single_quote = false;
                } else {
                    current_arg.push(c);
                }
            } else if in_double_quote {
                if c == '"' {
                    in_double_quote = false;
                } else if c == '\\' {
                    if let Some(&next_char) = chars.peek() {
                        if next_char == '"' || next_char == '\\' {
                            chars.next();
                            current_arg.push(next_char);
                        } else {
                            current_arg.push(c);
                        }
                    } else {
                        current_arg.push(c);
                    }
                } else {
                    current_arg.push(c);
                }
            } else if is_escaped {
                current_arg.push(c);

                is_escaped = false;
            } else if c == '\'' {
                in_single_quote = true;
            } else if c == '"' {
                in_double_quote = true;
            } else if c == '\\' {
                is_escaped = true;
            } else if c.is_whitespace() {
                if !current_arg.is_empty() {
                    args.push(current_arg);
                    current_arg = String::new();
                } else {
                }
            } else {
                current_arg.push(c);
            }
        }

        if !current_arg.is_empty() {
            args.push(current_arg);
        }

        if args.is_empty() {
            continue;
        }

        if args.len() >= 2 {
    if args[args.len() - 2] == ">" || args[args.len() - 2] == "1>" || args[args.len() - 2] == "2>" || args[args.len() -2] == "1>>" || args[args.len() - 2] == ">>" || args[args.len() - 2] == "2>>"{
        let target_filename = args.pop();
        let redirection_operator = args.pop();

        match target_filename {
            Some(file) => {
                // 1. Create parent directories and file once
                if let Some(parent) = std::path::Path::new(&file).parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
               
                // 2. Route the file handle to the correct stream variable
                match redirection_operator.as_deref() {
                    Some(">") | Some("1>") => {
                        redirect_file = Some(std::fs::File::create(&file).expect("Failed to create file"));

                    }
                    Some("2>") => {
                        stderr_redirect_file = Some(std::fs::File::create(&file).expect("Failed to create file"));

                    }

                    Some(">>") | Some("1>>") => {
                        redirect_file = Some(std::fs::OpenOptions::new().write(true).create(true).append(true).open(&file).expect("Failed to open file for appending"));

                        
                    }
                    _ => {}
                }
            }
            None => {}
        }
    }
}
        let command = args[0].as_str();
        let args = &args[1..];
        

        
        match command {
            "exit" => {
                break;
            }

            "echo" => {
                match redirect_file {
                   Some(ref mut file) => {
                    writeln!(file, "{}", args.join(" ")).expect("Failed to write to file")
                   }
                    None =>  {println!("{}", args.join(" ")); }
                }

                
            }

            "type" => {
                if let Some(target) = args.first() {
                    match target.as_str() {
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
                let cur_dir = std::env::current_dir().expect("Failed to get current directory");

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
                   let mut cmd = Command::new(&exec_path);
                   cmd.args(args);

                   // Handle standard output redirection (> or 1>)
                   match redirect_file {
                       Some(ref file) => {
                        cmd.stdout(std::process::Stdio::from(file.try_clone().expect("Failed to clone file")));
                       }
                       None => {}
                   }

                   // Handle standard error redirection (2>)
                   match stderr_redirect_file {
                       Some(ref file) => {
                        cmd.stderr(std::process::Stdio::from(file.try_clone().expect("Failed to clone file")));
                       }
                       None => {}
                   }

                   match cmd.status() {
                       Ok(_status) => {}

                       Err(_) => {
                        println!("{}: failed to execute", command)
                       }
                   }
                } else {
                    println!("{}: not found", command);
                }
            }
        }
    }
}
