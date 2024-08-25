use std::env;

#[derive(PartialEq)]
pub enum Color {
    Red,
    Blue,
    Green,
}

pub struct ParsedArgs {
    pub options: Options,
    pub filename: Option<String>,
}

pub struct Options {
    pub show_time: bool,
}

/// # Parse command line arguments
/// Expects exactly one argument, the file name to search for
pub fn parse_args() -> Result<ParsedArgs, &'static str> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return Err("No file name given");
    }

    let mut options = Vec::new();
    let mut filename = None;

    for arg in args {
        if arg.starts_with("-") {
            options.push(arg);
        } else if filename.is_none() {
            filename = Some(arg);
        } else {
            return Err("Multiple file names given");
        }
    }

    Ok(ParsedArgs {
        options: Options {
            show_time: options.contains(&"-t".to_string()),
        },
        filename,
    })
}

/// # Print text in the given color
pub fn color_print(color: Color, text: &String) {
    match color {
        Color::Red => eprintln!("\x1b[31m{}\x1b[0m", text),
        Color::Blue => println!("\x1b[34m{}\x1b[0m", text),
        Color::Green => println!("\x1b[32m{}\x1b[0m", text),
    }
}
