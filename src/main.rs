#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {    
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        const BUILTINS: [&str;3] = ["type" , "echo" , "exit"];
        let command = input.trim();
        if command == "exit" {
            break;
        }
        if command.starts_with("type"){
            let parts : Vec<&str> = command.split_whitespace().collect();
            if parts.len() == 2 {
                let cmd = parts[1];

                if BUILTINS.contains(&cmd) {
                    println!("{} is a shell builtin",cmd);
                } else {
                    println!("{}: not found" , cmd);
                }
            }

            continue;
        }
        if command.starts_with("echo"){
            println!("{}" , &command[5..]);
        }else {
            println!("{}: command not found" , command);
        }
    }
}
