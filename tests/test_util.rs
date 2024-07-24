use std::{fs, io::ErrorKind};

pub fn are_files_identical(file_path1: &str, file_path2: &str) -> std::io::Result<bool> {
    match (fs::read(file_path1), fs::read(file_path2)) {
        (Ok(contents1), Ok(contents2)) => Ok(contents1 == contents2),
        (Err(e), _) | (_, Err(e)) => {
            if e.kind() == ErrorKind::NotFound {
                Err(e)
            } else {
                // Handle other errors (e.g., permissions issues)
                Err(e)
            }
        }
    }
}