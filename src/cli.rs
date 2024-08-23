use std::env;

/// # Parse command line arguments
/// Expects exactly one argument, the file name to search for
pub fn parse_args() -> Result<String, &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        Err("Only one file name should be provided")
    } else {
        Ok(args[1].clone())
    }
}

pub fn print_red(text: &String) {
    eprintln!("\x1b[31m{}\x1b[0m", text);
}

pub fn print_blue(text: &String) {
    println!("\x1b[34m{}\x1b[0m", text);
}

pub fn print_green(text: &String) {
    println!("\x1b[32m{}\x1b[0m", text);
}
