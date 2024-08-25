use std::env;

#[derive(PartialEq)]
pub enum Color {
    Red,
    Blue,
    Green,
}

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

/// # Print text in the given color
pub fn color_print(color: Color, text: &String) {
    match color {
        Color::Red => eprintln!("\x1b[31m{}\x1b[0m", text),
        Color::Blue => println!("\x1b[34m{}\x1b[0m", text),
        Color::Green => println!("\x1b[32m{}\x1b[0m", text),
    }
}
