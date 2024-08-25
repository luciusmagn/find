mod cli;
mod finder;

use std::process;
use std::time::Instant;

fn main() {
    // Start time
    let start = Instant::now();

    // Parse command line arguments for file name
    let filename = cli::parse_args().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    // Get all root directories
    let root_dirs = finder::get_root_dirs();
    let mut results = Vec::new();

    // Traverse all root directories and search for all the occurences of the file
    for root in root_dirs {
        match finder::search_files(&root, &filename) {
            Ok(mut files) => results.append(&mut files),
            Err(e) => eprintln!("Error searching {}: {}", root.display(), e),
        }
    }

    if results.is_empty() {
        // If no files were found
        cli::color_print(
            cli::Color::Blue,
            &format!("No files found matching '{}'", filename),
        );
    } else {
        // If files were found print all the occurrences
        white_space(2);
        cli::color_print(
            cli::Color::Green,
            &format!("Found {} occurrences of '{}':", results.len(), filename),
        );
        for path in results {
            cli::color_print(cli::Color::Blue, &format!("{}", path.display()));
        }
    }

    // Time elapsed
    let elapsed = start.elapsed();
    white_space(1);
    cli::color_print(cli::Color::Green, &format!("Elapsed time: {:.2?}", elapsed));
}

fn white_space(n: usize) {
    for _ in 0..n {
        println!("");
    }
}
