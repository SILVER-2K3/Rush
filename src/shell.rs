use crate::builtins;
use crate::executor;
use crate::parser;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;

pub struct Shell;

impl Shell {
    pub fn new() -> Self {
        Shell
    }

    pub fn run(&mut self) {
        loop {
            print!("$> ");
            stdout().flush().unwrap(); //flushing stdout so that promp appears before read_line()

            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            if input == "exit" {
                break;
            }

            let args = parser::parser(input);

            // args.first() returns Option<&String>, map converts it to Option<&str> for executor
            let command = args.first().map(|s| s.as_str());

            match command {
                Some(cmd) => {
                    // skipping index 0 (coz thats the command duh), collecting remaining items as args by iterating through them one by one
                    let cmd_args: Vec<&str> = args[1..].iter().map(|s| s.as_str()).collect();

                    match builtins::run(cmd, &cmd_args) {
                        builtins::BuiltinResult::Success => {}
                        builtins::BuiltinResult::Failure(msg) => println!("rush: {}", msg),
                        builtins::BuiltinResult::NotBuiltin => {
                            executor::execute(cmd, &cmd_args);
                        }
                    }
                }
                None => {}
            }
        }
    }
}
