use std::env;
use std::fs::{OpenOptions, metadata};
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::{Command, Stdio};

struct ParsedCommand {
    args: Vec<String>,
    stdout_file: Option<String>,
}

const BUILTINS: [&str; 5] = ["type", "echo", "exit", "pwd", "cd"];

fn parse_command(command: &str) -> ParsedCommand {
    let mut parts = Vec::new();
    let mut current = String::new();

    let mut in_single_quotes = false;
    let mut in_double_quotes = false;
    let mut escaped = false;

    for c in command.chars() {
        if escaped {
            current.push(c);
            escaped = false;
            continue;
        }

        match c {
            '\\' if !in_single_quotes => {
                escaped = true;
            }

            '\'' if !in_double_quotes => {
                in_single_quotes = !in_single_quotes;
            }

            '"' if !in_single_quotes => {
                in_double_quotes = !in_double_quotes;
            }

            c if c.is_whitespace() && !in_single_quotes && !in_double_quotes => {
                if !current.is_empty() {
                    parts.push(std::mem::take(&mut current));
                }
            }

            _ => {
                current.push(c);
            }
        }
    }

    if escaped {
        current.push('\\');
    }

    if !current.is_empty() {
        parts.push(current);
    }

    let mut args = Vec::new();
    let mut stdout_file = None;

    let mut i = 0;

    while i < parts.len() {
        match parts[i].as_str() {
            ">" | "1>" => {
                if i + 1 < parts.len() {
                    stdout_file = Some(parts[i + 1].clone());
                    i += 2;
                    continue;
                }
            }

            _ => {}
        }

        args.push(parts[i].clone());
        i += 1;
    }

    ParsedCommand { args, stdout_file }
}

fn find_exe(cmd: &str) -> Option<PathBuf> {
    let path = env::var("PATH").unwrap_or_default();

    for dir in path.split(':') {
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

        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }

        let command = input.trim();

        if command.is_empty() {
            continue;
        }

        let parts = parse_command(command);

        if parts.args.is_empty() {
            continue;
        }

        match parts.args[0].as_str() {
            "exit" => {
                if parts.args.len() == 1 || (parts.args.len() == 2 && parts.args[1] == "0") {
                    break;
                }
            }

            "echo" => {
                println!("{}", parts.args[1..].join(" "));
            }

            "pwd" => {
                println!("{}", env::current_dir().unwrap().display());
            }

            "cd" => {
                let path = if parts.args.len() < 2 || parts.args[1] == "~" {
                    env::var("HOME").unwrap()
                } else {
                    parts.args[1].to_string()
                };

                if let Err(_) = env::set_current_dir(&path) {
                    println!("cd: {}: No such file or directory", path);
                }
            }

            "type" => {
                if parts.args.len() != 2 {
                    continue;
                }

                let cmd = parts.args[1].as_str();

                if BUILTINS.contains(&cmd) {
                    println!("{cmd} is a shell builtin");
                } else if let Some(path) = find_exe(cmd) {
                    println!("{cmd} is {}", path.display());
                } else {
                    println!("{cmd}: not found");
                }
            }

            _ => {
                let cmd = parts.args[0].as_str();

                if let Some(path) = find_exe(cmd) {
                    let mut command = Command::new(path);

                    command.args(parts.args.iter().skip(1));

                    // Redirect stdout if > or 1> was specified.
                    if let Some(output_file) = parts.stdout_file {
                        let file = OpenOptions::new()
                            .write(true)
                            .create(true)
                            .truncate(true)
                            .open(output_file)
                            .expect("failed to open output file");

                        command.stdout(Stdio::from(file));
                    }

                    // stderr is NOT redirected.
                    command.status().expect("failed to execute the command");
                } else {
                    println!("{cmd}: command not found");
                }
            }
        }
    }
}
