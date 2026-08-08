use std::env;
use std::fs::metadata;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;

const BUILTINS: [&str; 5] = ["type", "echo", "exit", "pwd", "cd"];

fn parse_command(command: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();

    let mut in_single_quotes = false;
    let mut in_double_quotes = false;

    for c in command.chars() {
        match c {
            '\'' if !in_double_quotes => {
                in_single_quotes = !in_single_quotes;
            }

            '"' if !in_single_quotes => {
                in_double_quotes = !in_double_quotes;
            }

            ' ' if !in_single_quotes && !in_double_quotes => {
                if !current.is_empty() {
                    parts.push(std::mem::take(&mut current));
                }
            }

            _ => {
                current.push(c);
            }
        }
    }

    if !current.is_empty() {
        parts.push(current);
    }

    parts
}


fn find_exe(cmd: &str) -> Option<PathBuf> {
    let path = env::var("PATH").unwrap_or_default();

    for dir in path.split(":") {
        let candidate = PathBuf::from(dir).join(cmd);
        if let Ok(metadata) = metadata(&candidate) {
            if metadata.is_file() && (metadata.permissions().mode() & 0o111 != 0) {
                return Some(candidate);
            }
        }
    }

    None
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = input.trim();

        if command.is_empty() {
            continue;
        }

        let parts = parse_command(command);

        match parts[0].as_str() {
            "exit" => {
                if parts.len() == 1 || (parts.len() == 2 && parts[1] == "0") {
                    break;
                }
            }

            "echo" => {
                println!("{}", parts[1..].join(" "));
            }
            "pwd" => {
                println!("{}", env::current_dir().unwrap().display());
            }

            "cd" => {
                let path = if parts.len() < 2 || parts[1] == "~" {
                    env::var("HOME").unwrap()
                } else {
                    parts[1].to_string()
                };

                if let Err(_) = env::set_current_dir(&path) {
                    println!("cd: {}: No such file or directory", path);
                }
            }

            "type" => {
                if parts.len() != 2 {
                    continue;
                }
                let cmd = parts[1].as_str();
                if BUILTINS.contains(&cmd) {
                    println!("{cmd} is a shell builtin");
                } else if let Some(path) = find_exe(cmd) {
                    println!("{cmd} is {}", path.display());
                } else {
                    println!("{cmd}: not found");
                }
            }

            _ => {
                let cmd = parts[0].as_str();
                if find_exe(cmd).is_some() {
                    //if let Some(path) = find_exe(cmd) {
                    Command::new(cmd)
                        .args(parts.iter().skip(1))
                        .status()
                        .expect("failed to execute the command");
                } else {
                    println!("{cmd}: command not found");
                }
            }
        }
    }
}
