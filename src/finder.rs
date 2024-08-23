use crate::cli;

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// # Get all root directories
/// - On Windows, list all available drives
/// - On Unix systems, just use the root directory
pub fn get_root_dirs() -> Vec<PathBuf> {
    if cfg!(windows) {
        // On Windows, list all available drives
        (b'A'..=b'Z')
            .filter_map(|drive| {
                let path = PathBuf::from(format!("{}:\\", drive as char));
                if path.exists() {
                    Some(path)
                } else {
                    None
                }
            })
            .collect()
    } else {
        // On Unix-like systems, just use the root directory
        vec![PathBuf::from("/")]
    }
}

/// # Traverse the directory tree and search for all files with the given filename
pub fn search_files(dir: &Path, filename: &str) -> io::Result<Vec<PathBuf>> {
    let mut results = Vec::new();
    search_files_internal(dir, filename, &mut results);
    Ok(results)
}

/// # Recursively traverse the current directory and search for all files with the given filename
/// Arguments are:
/// - The directory to search in
/// - The filename to search for
/// - And a mutable reference to a vector to store the results in
fn search_files_internal(dir: &Path, filename: &str, results: &mut Vec<PathBuf>) {
    match fs::read_dir(dir) {
        Ok(entries) => {
            // Iterate over all entries in the directory
            for entry in entries.filter_map(Result::ok) {
                // Get the path of the current entry
                let path = entry.path();

                // If the current entry is a directory, recursively search in it
                // Otherwise, check if the current entry is a file and if it matches the filename
                if path.is_dir() {
                    // println!("Looking at: {}", path.display());
                    search_files_internal(&path, filename, results);
                } else if path.is_file() {
                    if path.file_name().and_then(|n| n.to_str()) == Some(filename) {
                        // If the file matches the filename, add it to the results vector
                        results.push(path);
                    }
                }
            }
        }
        Err(e) => {
            if e.kind() == io::ErrorKind::PermissionDenied {
                cli::print_red(&format!("Inaccessible directory: {}", dir.display()));
            } else {
                cli::print_red(&format!(
                    "Error reading directory -> Error: {} -> Directory: {}",
                    e,
                    dir.display()
                ));
            }
        }
    }
}
