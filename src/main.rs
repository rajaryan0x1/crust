#[warn(unused_imports)]
use std::env;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::fs::metadata;

const BUILTINS: [&str; 3] = ["type", "echo", "exit"];

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
                if parts.len() == 2 {
                    let cmd = parts[1];

                    if BUILTINS.contains(&cmd) {
                        println!("{cmd} is a shell builtin");
                    } else {
                        let path = env::var("PATH").unwrap_or_default();
                        let mut found = false;

                        for dir in path.split(':') {
                            let mut candidate = PathBuf::from(dir);
                            candidate.push(cmd);

                            if let Ok(metadata) = metadata(&candidate) {
                                if metadata.is_file() && (metadata.permissions().mode() & 0o111 != 0) {
                                    println!("{cmd} is {}", candidate.display());
                                    found = true;
                                    break;

                                }

                            }

                        }

                        if !found {
                            println!("{cmd}: not found");
                        }
                    }
                }
            }

            _ => {
                println!("{command}: command not found");
            }
        }
    }
}
