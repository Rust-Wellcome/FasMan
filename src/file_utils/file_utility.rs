use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::Error;

struct Records {
    lines: Vec<String>
}

struct FileReader {
    buffer: Vec<String> // TODO: Make use of this internal buffer.
}

pub trait Default {
    fn default() -> Self;
}

impl Default for FileReader {
    fn default() -> Self {
        FileReader {
            buffer: Vec::<String>::new()
        }
    }
}

impl FileReader {

    /*
     * Reads a specific number of lines from a file
     */
    pub fn read_file(&mut self, file_path: &str, num_lines: usize) -> Result<Records, Error> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        // This buffer will be stored in heap, and will popped off when read_file function goes out of scope.
        let mut internal_buffer = vec![];   

        // Error unwrapping: https://tinyurl.com/brt9fphk
        // take() function https://tinyurl.com/6vx7m3k6
        for line in reader.lines().take(num_lines) {
            let result = line.expect("Error in reading file"); // This will panic if errored
            internal_buffer.push(result);
        };

        Ok(Records { lines: internal_buffer })
    }

}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    #[test]
    fn read_first_line() {
        let mut fileReader = FileReader::default();
        match fileReader.read_file("test_data/synthetic/tiny.fa", 3) {
            Ok(records) => {
                assert_eq!(3, records.lines.len())
            }
            Err(error) => {
                panic!("{:?}", error)
            }
        }
    }
}