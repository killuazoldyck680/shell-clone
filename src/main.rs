use std::fmt::format;
use std::io::{self, Write};
use std::ops::ControlFlow::Break;
use std::process::Command;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};


fn find_executables(input: &str) -> Vec<String> {
    let path = std::env::var("PATH").ok().unwrap_or_default();

    let mut matches = Vec::new();

    if "echo".starts_with(input) {
    matches.push("echo".to_string());
}
if "exit".starts_with(input) {
    matches.push("exit".to_string());
}


    for dir in std::env::split_paths(&path) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
               if let Ok(name) = entry.file_name().into_string() {
                if name.starts_with(input) {
                   matches.push(name);
                 

                    

                  
                   
                   
                }
               } 
            }
        }

       
    }
      matches.sort();
                    matches.dedup();
                  
    matches
}

fn find_completions(input: &str) -> Vec<String> {
    match input.rfind(' ') {
        Some(space_index) => {
            let arg = &input[space_index + 1..];

            let (dir_path, prefix) = match arg.rfind('/') {
                Some(slash_index) => {
                    let dp = &arg[..=slash_index];
                    let pfx = &arg[slash_index + 1..];
                    (dp, pfx)
                }
                None => (".", arg),
            };

            std::fs::read_dir(dir_path)
                .into_iter()
                .flatten()
                .filter_map(|entry| entry.ok())
                .filter_map(|entry| {
                    let name = entry.file_name().to_str()?.to_string();
                    if name.starts_with(prefix) {
                       let full_path =  if dir_path == "." {
                            name.clone()
                        } else {
                            format!("{}{}", dir_path, name)
                        };

                        let is_directory = std::fs::metadata(&full_path).map(|m| m.is_dir()).unwrap_or(false);

                        if is_directory {
                            if dir_path == "." {
                                Some(format!("{}/", name))
                            } else {
                                Some(format!("{}{}/", dir_path, name))
                            }
                        } else {
                            if dir_path == "." {
                                Some(name)
                            } else {
                                Some(full_path)
                            }
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<String>>()
        }

        None => {
            return find_executables(input);
        }
    }
}

fn main() {
     let mut last_tab_input: Option<String> = None;


    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
       
        crossterm::terminal::enable_raw_mode().unwrap();

        let mut input = String::new();

        loop {
            
           let event =  crossterm::event::read().unwrap() ;

           match event {
            Event::Key(key_event) => {
            //  Only process key press events on Windows to prevent duplication.
            if key_event.kind != KeyEventKind::Press {
                continue;
            }

            if key_event.modifiers.contains(KeyModifiers::CONTROL) && key_event.code == KeyCode::Char('c') {
                println!("^C");
                crossterm::terminal::disable_raw_mode().unwrap();
                std::process::exit(0);
            }
                match key_event.code {
                    crossterm::event::KeyCode::Char(c) => {input.push(c);
                    print!("{}", c);
                    io::stdout().flush().unwrap();

                    last_tab_input = None;
                    }

               crossterm::event::KeyCode::Backspace =>{ 
                  if !input.is_empty() {
                    input.pop();

                    print!("\x08 \x08");
                    io::stdout().flush().unwrap();

                    last_tab_input = None;
                }
                 }

               crossterm::event::KeyCode::Tab => {
                let matches = find_completions(&input);
               
               if matches.is_empty() {
                 print!("\x07");
            io::stdout().flush().unwrap();
               } else if matches.len() == 1 {
                    let cmd = &matches[0]; 
            
                if let Some(space_idx) = input.rfind(' ') {
                    input.truncate(space_idx + 1);

                    input.push_str(cmd);
                    if !cmd.ends_with('/') {
                        input.push(' ' );
                    }

                    input.push(' ');

                    print!("\r\x1b[K$ {}", input);
                } else {
                    let characters = &cmd[input.len()..];
        input.push_str(characters);
        input.push_str(" ");
        print!("{} ", characters);
                }

                io::stdout().flush().unwrap();
    last_tab_input = None;
    return;
               } else {
                if let Some(shortest) = matches.iter().min_by_key(|s| s.len()) {
                    
                   let character =  shortest.chars().enumerate();
                      let mut lcp_len = 0;
                   for (i,c) in character {
                   if  matches.iter().all(|s| s.chars().nth(i) == Some(c)) {
                    lcp_len +=1;
                   } else {
                       break;
                   }

                  
                   
                   }

                   let lcp = &shortest[..lcp_len];
                    if lcp > input.as_str() {
                    
                    let extra = &lcp[input.len()..];

                    input.push_str(extra);

                   print!("{}", extra);

                   io::stdout().flush().unwrap();
                        last_tab_input = None;
                        return;
                   


                   } else if lcp == input {
                       
                   }
                    eprintln!("\n[DEBUG] input: '{}', lcp: '{}', lcp_len: {}", input, lcp, lcp_len);
               

                  
                }

                

                   if last_tab_input.as_ref() == Some(&input) {
                    println!();
println!("{}", matches.join("  "));
print!("$ {}", input); 
io::stdout().flush().unwrap();
last_tab_input = None;
                   } else {
                        print!("\x07");
            io::stdout().flush().unwrap();

            last_tab_input = Some(input.clone());
                   }
               }



             
            }

              crossterm::event:: KeyCode::Enter => {
                println!();

              if let Some((cmd, args)) = input.split_once(' ') {
        // Handled when there is a space (e.g., "echo hello")
        if cmd == "echo" {
            println!("{}", args);
        }
    } else {
        // Handled when there are no spaces (e.g., "exit" or bare "echo")
        let trimmed = input.trim();
        if trimmed == "exit" {
            crossterm::terminal::disable_raw_mode().unwrap();
            break;
        } else if trimmed == "echo" {
            println!();
        }
    }
              input.clear();  

              last_tab_input = None;
                
            }
               _ => {}
                }
            }   
            
            _ => {}
           }
           

                   }
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

                    Some("2>>") => {
                       stderr_redirect_file = Some(std::fs::OpenOptions::new().write(true).create(true).append(true).open(&file).expect("Failed to open file for appending"))
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
