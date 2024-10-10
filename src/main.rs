use std::time::Instant;
use clap::{Arg, Command};
use colored::*;
use std::path::Path;

mod finder;

fn main() {
    // Define the CLI arguments using `clap`
    let matches = Command::new("File Finder")
        .version("1.0")
        .author("Your Name")
        .about("Searches for a file in a given directory")
        .arg(Arg::new("directory")
            .short('d')
            .long("directory")
            .value_name("DIRECTORY")
            .help("The directory to search in")
            .takes_value(true)
            .required(true))
        .arg(Arg::new("filename")
            .short('f')
            .long("filename")
            .value_name("FILENAME")
            .help("The name of the file to search for")
            .takes_value(true)
            .required(true))
        .arg(Arg::new("benchmark")
            .short('b')
            .long("benchmark")
            .help("Show the time taken for the search"))
        .get_matches();

    // Get the directory and filename arguments
    let directory = matches.value_of("directory").unwrap();
    let filename = matches.value_of("filename").unwrap();
    let benchmark = matches.is_present("benchmark");

    // Start the benchmark timer if the flag is set
    let start_time = Instant::now();

    // Perform the search
    let results = finder::search_files(Path::new(directory), filename);

    // Output results
    if results.is_empty() {
        println!("{}", "No files found.".red());
    } else {
        println!("{}", "Files found:".green());
        for result in results {
            println!("{}", result.display().to_string().yellow());
        }
    }

    // Show benchmark if the flag was set
    if benchmark {
        let duration = start_time.elapsed();
        println!(
            "{}: {} seconds",
            "Search completed in".blue(),
            format!("{:.2}", duration.as_secs_f64()).yellow()
        );
    }
}
