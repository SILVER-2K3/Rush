use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::process::Command;

pub struct Shell;

impl Shell {
    pub fn new() -> Self {
        Shell
    }

    pub fn run(&mut self) {
        loop {
            print!("$> ");
            stdout().flush().unwrap();

            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            if input == "exit" {
                break;
            }

            let mut parts = input.split_whitespace();

            let command = parts.next();

            match command {
                Some(cmd) => {
                    let args: Vec<&str> = parts.collect();

                    match Command::new(cmd).args(&args).spawn() {
                        Ok(mut child) => {
                            child.wait().unwrap();
                        }

                        Err(_) => {
                            println!("{}:command not found", cmd);
                        }
                    }
                }
                None => {}
            }
        }
    }
}
