use std::collections::HashMap;
use std::env::args;
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

fn find_completions(arg: &str) -> Vec<String> {
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
                let full_path = if dir_path == "." {
                    name.clone()
                } else {
                    format!("{}{}", dir_path, name)
                };

                let is_directory = std::fs::metadata(&full_path)
                    .map(|m| m.is_dir())
                    .unwrap_or(false);

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
fn main() {
    let mut last_tab_input: Option<String> = None;

    let mut collector = HashMap::new();

    let mut job_counter = 0;

    struct JobStorage {
        number: u64,
        process_id: u32,
        original_command_string: String,
        done_command_string: String,
        status: String,
        child: std::process::Child,

    }

    let mut job_entries: Vec<JobStorage> = Vec::new();




    loop {
        for job in job_entries.iter_mut() {
        if let Ok(Some(status)) = job.child.try_wait() {
            if status.success() {
                job.status = "Done".to_string();
            }
        }
    }

    let snapshot: Vec<(u64, String, String)> = job_entries
        .iter()
        .filter(|job| job.status == "Done")
        .map(|job| {
            
            (job.number, job.status.clone(),job.done_command_string.clone(), )
        })
        .collect();

    job_entries.retain(|job| job.status != "Done");

    let mut remaining_numbers: Vec<u64> = job_entries.iter().map(|job| job.number).collect();
    remaining_numbers.sort_by(|a, b| b.cmp(a));
    let highest = remaining_numbers.first().copied();
    let second_highest = remaining_numbers.get(1).copied();

    for (num, status, cmd) in snapshot {
        let marker = if Some(num) == highest {
            "+"
        } else if Some(num) == second_highest {
            "-"
        } else {
            " "
        };

        println!("[{}]{}  {:<24}  {}", num, marker, status, cmd);
    }





        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();

        crossterm::terminal::enable_raw_mode().unwrap();

        let mut input = String::new();

        loop {
            let event = crossterm::event::read().unwrap();

            match event {
                Event::Key(key_event) => {
                    //  Only process key press events on Windows to prevent duplication.
                    if key_event.kind != KeyEventKind::Press {
                        continue;
                    }

                    if key_event.modifiers.contains(KeyModifiers::CONTROL)
                        && key_event.code == KeyCode::Char('c')
                    {
                        println!("^C");
                        crossterm::terminal::disable_raw_mode().unwrap();
                        std::process::exit(0);
                    }
                    match key_event.code {
                        crossterm::event::KeyCode::Char(c) => {
                            input.push(c);
                            print!("{}", c);
                            io::stdout().flush().unwrap();

                            last_tab_input = None;
                        }

                        crossterm::event::KeyCode::Backspace => {
                            if !input.is_empty() {
                                input.pop();

                                print!("\x08 \x08");
                                io::stdout().flush().unwrap();

                                last_tab_input = None;
                            }
                        }

 crossterm::event::KeyCode::Tab => {
    let tokens = input.split_whitespace().collect::<Vec<&str>>();

    if !tokens.is_empty() {
        let command_name = tokens[0];

            if let Some(script_path) = collector.get(command_name) {
           let (arg2, arg3) =  if input.ends_with(' ') {
                let a3 = if tokens.len() > 1 { *tokens.last().unwrap() } else { "" };
                ("", a3)
            } else {
                let a2 = *tokens.last().unwrap();
                let a3 = if tokens.len() >= 2 { tokens[tokens.len() - 2] } else { "" };
                (a2, a3) 
            };

            let val = input.len().to_string();
       let output = std::process::Command::new(&script_path)
                .arg(command_name)
                .arg(arg2)
                .arg(arg3)
                .env("COMP_LINE", &input)
                .env("COMP_POINT", val)
                .output()
                .expect("Error");
        

             let stdout_str = String::from_utf8_lossy(&output.stdout);


            let mut candidates = stdout_str.lines().filter(|s| !s.is_empty()).collect::<Vec<&str>>();
           
            

            candidates.sort();

            if candidates.is_empty() {
                print!("\x07");
                io::stdout().flush().unwrap();

                last_tab_input = None;
            } else if candidates.len() == 1 {
                if !arg2.is_empty() {
                    input.truncate(input.len() - arg2.len());
                }
                    input.push_str(candidates[0]);

                    input.push(' ');
                    print!("\r\x1b[K$ {}", input);

                    io::stdout().flush().unwrap();
                    last_tab_input = None;
                
            } else {
                let first = candidates[0];

                let last = candidates[candidates.len() - 1];

               let prefix_len =  first.chars().zip(last.chars()).take_while(|(a,b)| a == b).map(|(c,_)| c.len_utf8()).sum::<usize>();

                let lcp = &first[..prefix_len];

                if lcp.len() > arg2.len() {
                    if !arg2.is_empty() {
                    input.truncate(input.len() - arg2.len());

                   
                }

                 input.push_str(lcp);
                        print!("\r\x1b[K$ {}", input);
                        io::stdout().flush().unwrap();
                        last_tab_input = None;


                } else {
                    if last_tab_input.as_ref() == Some(&input) {
                println!();

                println!("{}", candidates.join("  "));
                print!("\r\x1b[K$ {}", input);

                io::stdout().flush().unwrap();
                last_tab_input = None;
               } else {
                   print!("\x07");
                   io::stdout().flush().unwrap();

                   last_tab_input = Some(input.clone());
               }
                }

                

               
            }

            continue;
            

       
       


    }
    }

  
     

    
   
    
    


    let trimmed_input = input.trim_end();
    let space_idx = trimmed_input.rfind(' ');
    
    let mut matches = match space_idx {
        Some(idx) => {
            find_completions(&trimmed_input[idx + 1..])
        }

        None => {
            find_executables(trimmed_input)
        }
    };

    matches.sort();

    if matches.is_empty() {
        print!("\x07");
        io::stdout().flush().unwrap();

        last_tab_input = None;
    } else if matches.len() == 1 {
        let cmd = &matches[0];

        if let Some(space_idx) = input.rfind(' ') {
            input.truncate(space_idx + 1);

            input.push_str(cmd);
            if !cmd.ends_with('/') {
                input.push(' ');
            }

            print!("\r\x1b[K$ {}", input);
        } else {
            let characters = &cmd[input.len()..];
            input.push_str(characters);
            if !cmd.ends_with('/') {
                input.push(' ');
                print!("{} ", characters);
            } else {
                print!("{}", characters);
            }
        }

        io::stdout().flush().unwrap();
        last_tab_input = None;
    } else {
        if let Some(shortest) = matches.iter().min_by_key(|s| s.len()) {
            let character = shortest.chars().enumerate();
            let mut lcp_len = 0;
            for (i, c) in character {
                if matches.iter().all(|s| s.chars().nth(i) == Some(c)) {
                    lcp_len += 1;
                } else {
                    break;
                }
            }

            let lcp = &shortest[..lcp_len];

            let current_token = if let Some(space_idx) = trimmed_input.rfind(' ') {
                trimmed_input[space_idx + 1..].to_string()
            } else {
                trimmed_input.to_string()
            };

            let token_basename = if let Some(slash_idx) = current_token.rfind('/') {
                current_token[slash_idx + 1..].to_string()
            } else {
                current_token
            };

            // Mutually exclusive: either we extend the LCP or we fall back to double-tab behavior
            if lcp > token_basename.as_str() {
                let extra = &lcp[token_basename.len()..];

                input.push_str(extra);

                // Cleanly redraw the prompt line so the terminal stays fully synchronized
                print!("\r\x1b[K$ {}", input);

                io::stdout().flush().unwrap();
                last_tab_input = None;
            } else {
                // Fallback: double-tab tracking and listing
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
    }
}
                       crossterm::event::KeyCode::Enter => {
                            println!();
                            break; 
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

        let mut args: Vec<String> = Vec::new();

        let mut current_arg = String::new();

        let mut in_single_quote = false;

        let mut in_double_quote = false;

        let mut is_escaped = false;

        let mut redirect_file: Option<std::fs::File> = None;

        let mut stderr_redirect_file: Option<std::fs::File> = None;

        
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

        let mut is_background = false;

        if args.last().map(|s| s.as_str()) == Some("&") {
            args.pop();

            is_background = true;
            
        }

      if args.is_empty() { continue; }  

        if let Some(pipe_idx) = args.iter().position(|s| s == "|") {
            let path = std::env::var("PATH").unwrap_or_default();
                let delimiter = if cfg!(windows) { ';' } else { ':' };

                let mut left_exec = None;

                let mut right_exec = None;


            let left_slice = &args[..pipe_idx];

            let right_slice = &args[pipe_idx + 1..];

            if left_slice.is_empty() || right_slice.is_empty() {
                println!("Error");
                continue;
            }

            let left_arg = &left_slice[1..];

            let right_arg = &right_slice[1..];

            let left_command = left_slice[0].as_str();

            let right_command = right_slice[0].as_str();

            if (left_command.contains('/') || left_command.contains('\\')) && std::path::Path::new(left_command).is_file() {
     left_exec = Some(left_command.to_string());
}
                if left_exec.is_none() {
                     for dir in path.split(delimiter) {
                    let mut full_path = format!("{dir}/{left_command}");
                    let file_path = std::path::Path::new(&full_path);
                    let mut exists = file_path.exists();

                    
                    if !exists && cfg!(windows) {
                        let exe_path = format!("{dir}/{left_command}.exe");
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
                        left_exec = Some(full_path);
                        break;
                    }
                }

                }




                 if (right_command.contains('/') || right_command.contains('\\')) && std::path::Path::new(right_command).is_file() {
     right_exec = Some(right_command.to_string());
}
                if right_exec.is_none() {
                     for dir in path.split(delimiter) {
                    let mut full_path = format!("{dir}/{right_command}");
                    let file_path = std::path::Path::new(&full_path);
                    let mut exists = file_path.exists();

                    
                    if !exists && cfg!(windows) {
                        let exe_path = format!("{dir}/{right_command}.exe");
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
                        right_exec = Some(full_path);
                        break;
                    }
                }

                }

                let left_child = None;

                let pipe_stdin = None;

                match left_exec {
                    Some(left_path) => {
                    let new_left_command  =     Command::new(left_path).args(left_arg).stdout(std::process::Stdio::piped());

                    match new_left_command.spawn() {
                        Ok(mut left_child) => {
                           let pipe_stdin =  std::process::Stdio::from(left_child.stdout.take().unwrap());
                           
                           
                        }

                        Err(_) => {
                            continue;
                        }
                    }


                    
                    }
                    None => {
                        println!("{left_command}: not found");
                        continue;
                    }
                }

                
                match right_exec {
                    Some(right_path) => {
                    let new_right_command  =     Command::new(right_path).args(right_arg).stdin();
                    }

                    None => {
                        println!("{right_command}: not found");
                        continue;
                    }
                }


        }


        
        if args.len() >= 2 {
            if args[args.len() - 2] == ">"
                || args[args.len() - 2] == "1>"
                || args[args.len() - 2] == "2>"
                || args[args.len() - 2] == "1>>"
                || args[args.len() - 2] == ">>"
                || args[args.len() - 2] == "2>>"
            {
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
                                redirect_file = Some(
                                    std::fs::File::create(&file).expect("Failed to create file"),
                                );
                            }
                            Some("2>") => {
                                stderr_redirect_file = Some(
                                    std::fs::File::create(&file).expect("Failed to create file"),
                                );
                            }

                            Some(">>") | Some("1>>") => {
                                redirect_file = Some(
                                    std::fs::OpenOptions::new()
                                        .write(true)
                                        .create(true)
                                        .append(true)
                                        .open(&file)
                                        .expect("Failed to open file for appending"),
                                );
                            }

                            Some("2>>") => {
                                stderr_redirect_file = Some(
                                    std::fs::OpenOptions::new()
                                        .write(true)
                                        .create(true)
                                        .append(true)
                                        .open(&file)
                                        .expect("Failed to open file for appending"),
                                )
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

            "echo" => match redirect_file {
                Some(ref mut file) => {
                    writeln!(file, "{}", args.join(" ")).expect("Failed to write to file")
                }
                None => {
                    println!("{}", args.join(" "));
                }
            },

            "type" => {
                if let Some(target) = args.first() {
                    match target.as_str() {
                        "echo" | "type" | "exit" | "pwd" | "cd" | "complete" | "jobs" => {
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

            "complete" => {
                
                if !args.is_empty() {
                    let first = args
                    .get(0).unwrap();
                     
                    if first.as_str() == "-p" {
                        let second = args.get(1).unwrap();

                       match collector.get(second) {
                        Some(script_path) => {
                            println!("complete -C '{script_path}' {second}");
                        }

                        None => {
                            println!("complete: {second}: no completion specification");
                        }
                       }
                    } else if first.as_str() == "-C" {
                        let script_path = args.get(1).unwrap().to_string();

                        let target = args.get(2).unwrap().to_string();

                        collector.insert(target, script_path);
                    } else if first.as_str() == "-r" {
                       if let Some(second) = args.get(1) {
                        collector.remove(second);
                       }
                    }
                    
                    
                    else {

                         let second = args.get(1).unwrap();

                        println!("complete: {second}: no completion specification");
                    }
                }
            }

            "jobs" => {
    for job in job_entries.iter_mut() {
        if let Ok(Some(status)) = job.child.try_wait() {
            if status.success() {
                job.status = "Done".to_string();
            }
        }
    }

    let snapshot: Vec<(u64, String, String)> = job_entries
        .iter()
        .map(|job| {
            let cmd = if job.status == "Done" {
                job.done_command_string.clone()
            } else {
                job.original_command_string.clone()
            };
            (job.number, job.status.clone(), cmd)
        })
        .collect();

    job_entries.retain(|job| job.status != "Done");

    let mut remaining_numbers: Vec<u64> = job_entries.iter().map(|job| job.number).collect();
    remaining_numbers.sort_by(|a, b| b.cmp(a));
    let highest = remaining_numbers.first().copied();
    let second_highest = remaining_numbers.get(1).copied();

    for (num, status, cmd) in snapshot {
        let marker = if Some(num) == highest {
            "+"
        } else if Some(num) == second_highest {
            "-"
        } else {
            " "
        };

        println!("[{}]{}  {:<24}  {}", num, marker, status, cmd);
    }
}

            
            _ => {
               
                let path = std::env::var("PATH").unwrap_or_default();
                let delimiter = if cfg!(windows) { ';' } else { ':' };
                let mut executable_path = None;

               if (command.contains('/') || command.contains('\\')) && std::path::Path::new(command).is_file() {
    executable_path = Some(command.to_string());
}
                if executable_path.is_none() {
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

                }

               
                if let Some(exec_path) = executable_path {
                    let mut cmd = Command::new(&exec_path);
                    cmd.args(args);

                    // Handle standard output redirection (> or 1>)
                    match redirect_file {
                        Some(ref file) => {
                            cmd.stdout(std::process::Stdio::from(
                                file.try_clone().expect("Failed to clone file"),
                            ));
                        }
                        None => {

                            cmd.stdout(std::process::Stdio::inherit());

                            
                            

                        }
                    }

                    // Handle standard error redirection (2>)
                    match stderr_redirect_file {
                        Some(ref file) => {
                            cmd.stderr(std::process::Stdio::from(
                                file.try_clone().expect("Failed to clone file"),
                            ));
                        }
                        None => {
                            cmd.stderr(std::process::Stdio::inherit());

                        }
                    }

                    match cmd.spawn() {
                        Ok(mut child) => {
                            if is_background {
                             let result =    if job_entries.is_empty() {
                                    1
                                } else {
                                   job_entries.iter().map(|job| job.number).max().unwrap_or(0) + 1
                                };
                                println!("[{}] {}", result, child.id());

                             let (command_with_amp, command_without_amp) = if args.is_empty() {
    (format!("{command} &"), format!("{command}"))
} else {
    let joined_args = args.join(" ");
    (
        format!("{command} {joined_args} &"),
        format!("{command} {joined_args}"),
    )
};
                                let entry = JobStorage {
                                    original_command_string: command_with_amp,
                                    done_command_string: command_without_amp,
                                    status : "Running".to_string(),
                                    number: result,
                                    process_id: child.id(), child
                                };

                                job_entries.push(entry);

                            } else {
                                let _ = child.wait();
                            }
                        }

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
