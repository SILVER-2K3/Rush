pub fn parser(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false; //tracks if the string is in quotes or not 

    for ch in input.chars() {
        match ch {
            '"' => in_quotes = !in_quotes, //flips the switch
            ' ' if !in_quotes => {
                if !current.is_empty() { // 'current' HAS something
                    args.push(current.clone());
                    current = String::new();
                }
            }
            _ => current.push(ch),
        }
    }

    // push the last word since there's no trailing space to trigger it inside the loop
    if !current.is_empty() {
        args.push(current);
    }
    args
}
