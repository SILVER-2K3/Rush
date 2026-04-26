use std::io::stdin;
use std::io::stdout;
use  std::io::Write;

pub struct Shell;

impl Shell {
    pub fn new() -> Self {
        Shell
    }

    pub fn run(&mut self){
        loop{
            print!("$> ");
            stdout().flush().unwrap();

            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            let input =  input.trim();
            if  input == "exit" {
                break;
            }
            println!("input was {}", input); 
        }
    } 
}