mod shell;
mod executor;
mod parser;

use shell::Shell;

fn main() {
    let mut shell = Shell::new();
    shell.run();
}