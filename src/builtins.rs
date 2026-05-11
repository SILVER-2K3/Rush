use crate::env_manager::EnvManager;
use std::env;

pub enum BuiltinResult {
    Success,
    Failure(String),
    NotBuiltin,
}

pub fn run(cmd: &str, args: &[&str], env: &EnvManager) -> BuiltinResult {
    match cmd {
        "cd" => builtin_cd(args, env),
        "pwd" => builtin_pwd(),
        "echo" => builtin_echo(args),
        _ => BuiltinResult::NotBuiltin,
    }
}

fn builtin_cd(args: &[&str], env: &EnvManager) -> BuiltinResult {
    let path = if args.is_empty() {
        // DO NOT use || operator (or) just use "if args.is_empty() || args.get(0).map(|s| *s) == Some("~")" but Nah
        match env.get("HOME") {
            Some(home) => home.clone(),
            None => return BuiltinResult::Failure("HOME not set".to_string()),
        }
    } else if args[0] == "~" {
        match env.get("HOME") {
            Some(home) => home.clone(),
            None => return BuiltinResult::Failure("HOME not set".to_string()),
        }
    } else {
        args[0].to_string()
    };

    match env::set_current_dir(&path) {
        Ok(_) => BuiltinResult::Success,
        Err(e) => BuiltinResult::Failure(e.to_string()),
    }
}

fn builtin_pwd() -> BuiltinResult {
    match env::current_dir() {
        Ok(path) => {
            println!("{}", path.display());
            BuiltinResult::Success
        }
        Err(e) => BuiltinResult::Failure(e.to_string()),
    }
}

fn builtin_echo(args: &[&str]) -> BuiltinResult {
    println!("{}", args.join(" "));
    BuiltinResult::Success
}
