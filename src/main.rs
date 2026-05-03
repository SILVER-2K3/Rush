mod shell;
mod executor;
mod parser;
mod builtins;

use shell::Shell;

fn main() {
    let mut shell = Shell::new();
    shell.run();
}