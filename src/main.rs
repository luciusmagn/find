mod cli;
mod finder;

use std::path::PathBuf;
use std::process;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let (options, filename) = cli::parse_args();
    let filename = filename.unwrap_or_else(|| {
        // If no file name was given, print help message and exit the program
        cli::color_print(
            cli::Color::Red,
            &"No file name was given - show help message:".to_string(),
        );
        cli::white_space(1);
        cli::print_help();
        process::exit(0);
    });

    if options.show_help {
        cli::print_help();
        process::exit(0);
    }

    let root_dirs = finder::get_root_dirs();
    let mut results = Vec::new();

    // Traverse all root directories and search for all the occurences of the file
    for root in root_dirs {
        match finder::search_files(&root, &filename) {
            Ok(mut files) => results.append(&mut files),
            Err(e) => eprintln!("Error searching {}: {}", root.display(), e),
        }
    }

    print_results(&results, &filename);

    // If time was requested, print the time elapsed
    if options.show_time {
        let elapsed = start.elapsed();
        cli::white_space(1);
        cli::color_print(cli::Color::Green, &format!("Elapsed time: {:.2?}", elapsed));
    }
}

fn print_results(results: &Vec<PathBuf>, filename: &String) {
    if results.is_empty() {
        // If no files were found
        cli::color_print(
            cli::Color::Blue,
            &format!("No files found matching '{}'", filename),
        );
    } else {
        // If files were found print all the occurrences
        cli::white_space(2);
        cli::color_print(
            cli::Color::Green,
            &format!("Found {} occurrences of '{}':", results.len(), filename),
        );
        for path in results {
            cli::color_print(cli::Color::Blue, &format!("{}", path.display()));
        }
    }
}
