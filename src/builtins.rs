use std::env;

pub enum BuiltinResult{
    Success,
    Failure(String),
    NotBuiltin,
}

pub fn run(cmd: &str, args: &[&str]) -> BuiltinResult{
    match cmd {
        "cd" => builtin_cd(args),
        _ => BuiltinResult::NotBuiltin,
    }
}


fn builtin_cd(args: &[&str]) -> BuiltinResult{
    let path = args.get(0).unwrap_or(&"~");

    match env::set_current_dir(path) {
        Ok(_) => BuiltinResult::Success,
        Err(e)  => BuiltinResult::Failure(e.to_string()),
    }
}