use std::env;
use std::process;

#[derive(PartialEq)]
pub enum Color {
    Red,
    Blue,
    Green,
}

pub struct Options {
    pub show_help: bool,
    pub show_time: bool,
}

/// # Parse command line arguments
/// Expects exactly one argument, the file name to search for
pub fn parse_args() -> (Options, Option<String>) {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut options = Vec::new();
    let mut filename = None;

    for arg in args {
        if arg.starts_with("-") {
            options.push(arg);
        } else if filename.is_none() {
            filename = Some(arg);
        }
    }

    if options.contains(&"-h".to_string())
        || options.contains(&"--help".to_string()) && filename.is_some()
    {
        color_print(
            Color::Red,
            &"Cannot use --help with a file name".to_string(),
        );
        print_help();
        process::exit(0);
    }

    (
        Options {
            show_help: options.contains(&"-h".to_string())
                || options.contains(&"--help".to_string())
                || options.is_empty(),
            show_time: options.contains(&"-t".to_string())
                || options.contains(&"--time".to_string())
                || options.is_empty(),
        },
        filename,
    )
}

pub fn print_help() {
    color_print(
        Color::Blue,
        &"Usage, depending on if the program is run as a binary or as a library:".to_string(),
    );
    color_print(
        Color::Blue,
        &"    .finder-rs [options] <filename>".to_string(),
    );
    color_print(
        Color::Blue,
        &"    cargo run -- [options] <filename>".to_string(),
    );
    white_space(1);
    color_print(Color::Green, &"Options:".to_string());
    color_print(
        Color::Green,
        &"    -h, --help      Print this help message".to_string(),
    );
    color_print(
        Color::Green,
        &"    -t, --time      Show time elapsed".to_string(),
    );
}

/// # Print text in the given color
pub fn color_print(color: Color, text: &String) {
    match color {
        Color::Red => eprintln!("\x1b[31m{}\x1b[0m", text),
        Color::Blue => println!("\x1b[34m{}\x1b[0m", text),
        Color::Green => println!("\x1b[32m{}\x1b[0m", text),
    }
}

pub fn white_space(n: usize) {
    for _ in 0..n {
        println!("");
    }
}
