use std::env;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::fs::metadata;
use std::process::Command;

const BUILTINS: [&str; 4] = ["type", "echo", "exit" , "pwd"];

fn find_exe(cmd : &str) ->Option<PathBuf> {
    let path = env::var("PATH").unwrap_or_default();

    for dir in path.split(":"){
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

        let parts: Vec<&str> = command.split_whitespace().collect();

        match parts[0] {
            "exit" => {
                if parts.len() == 1 || (parts.len() == 2 && parts[1] == "0") {
                    break;
                }
            }

            "echo" => {
                println!("{}", parts[1..].join(" "));
            }

            "type" => {

                if parts.len() != 2 {
                    continue;
                }
                let cmd = parts[1];
                if BUILTINS.contains(&cmd) {
                    println!("{cmd} is a shell builtin");
                } else if let Some(path) = find_exe(cmd) {
                    println!("{cmd} is {}" , path.display());
                } else {
                    println!("{cmd}: not found");
                }
            }

            _ => {
                let cmd = parts[0];
                if find_exe(cmd).is_some() {
                //if let Some(path) = find_exe(cmd) {
                    Command::new(cmd)
                        .args(&parts[1..])
                        .status()
                        .expect("failed to execute the command");
                } else {
                    println!("{cmd}: command not found");
                }
            }
        }
    }
}
