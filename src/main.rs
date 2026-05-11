mod shell;
mod executor;
mod parser;
mod builtins;
mod env_manager;

use shell::Shell;

fn main() {
    let mut shell = Shell::new();
    shell.run();
}