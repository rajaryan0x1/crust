#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    loop {

    
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = input.trim();
        if command == "exit" {
            break;
        }

        if command.starts_with("echo"){
            println!("{}" , &command[5..]);
        }else {
            println!("{}: command not found" , command);
        }
    }
}
