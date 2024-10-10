use std::path::Path;
use std::time::Instant;

use clap::Parser;

mod finder;

/// Find files by name in a specified directory
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The name of the file to search for
    #[arg(short, long)]
    filename: String,

    /// The directory to search
    #[arg(short, long)]
    dir: String,
    
    /// Show the time elapsed for the search
    #[arg(short, long)]
    time: bool,
}

fn main() {
    let args = Args::parse();
    let filename = &args.filename;
    let dir = &args.dir;
    let time = args.time;

    let start = Instant::now();
    let results = finder::search_files(&Path::new(dir), filename);
    let elapsed = start.elapsed();

    if time {
        println!("Time elapsed: {:?}", elapsed);
    }

    for result in results {
        println!("{}", result.display());
    }
}
