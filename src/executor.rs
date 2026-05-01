use std::process::Command;

pub fn execute(cmd: &str, args: &[&str]) {
    match Command::new(cmd).args(args).spawn() {
        Ok(mut child) => {
            child.wait().unwrap(); //wait() so that child process finishes first and then prints promp
        }

        Err(_) => {
            println!("{}:command not found", cmd);
        }
    }
}
