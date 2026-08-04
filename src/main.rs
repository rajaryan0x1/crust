use std::io::{self, Write};

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
                if parts.len()==1 ||parts.len() == 2 && parts[1] == "0" {
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
                        println!("{} is a shell builtin", cmd);
                    } else {
                        println!("{}: not found", cmd);
                    }
                }
            }

            _ => {
                println!("{}: command not found", command);
            }
        }
    }
}
