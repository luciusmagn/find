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
/// Arguments are:
/// - The directory to search in
/// - The filename to search for
pub fn search_files(dir: &Path, filename: &str) -> io::Result<Vec<PathBuf>> {
    let mut results = Vec::new();

    // Recursively traverse the directory
    match fs::read_dir(dir) {
        Ok(entries) => {
            // Iterate over all entries in the directory
            for entry in entries.filter_map(Result::ok) {
                // Get the path of the current entry
                let path = entry.path();

                // If the current entry is a directory, recursively search in it
                // Otherwise, check if the current entry is a file and if it matches the filename
                if path.is_dir() {
                    if let Ok(mut sub_results) = search_files(&path, filename) {
                        results.append(&mut sub_results);
                    }
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
                cli::color_print(
                    cli::Color::Red,
                    &format!("Inaccessible directory: {}", dir.display()),
                );
            } else {
                cli::color_print(
                    cli::Color::Red,
                    &format!("Error: {} | Reading: {}", e, dir.display()),
                );
            }
        }
    }

    Ok(results)
}
