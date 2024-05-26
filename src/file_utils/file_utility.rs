use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::Error;
use itertools::Itertools;

#[allow(dead_code)]
struct Records<T> {
    lines: Vec<T>,
}

impl Records<String> {
    pub fn size(&self) -> usize {
        self.lines.len()
    }
}

#[allow(dead_code)]
struct BatchFileReader {}

#[allow(dead_code)]
pub trait DefaultReader {
    fn default() -> Self;
}

impl DefaultReader for BatchFileReader {
    fn default() -> Self {
        BatchFileReader {}
    }
}

impl BatchFileReader {
    /*
     * Reads a specific number of lines from a file from the top
     */
    pub fn read_lines(
        &mut self,
        file_path: &str,
        num_lines: usize,
    ) -> Result<Records<String>, Error> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut internal_buffer = Vec::<String>::new();

        // Error unwrapping: https://tinyurl.com/brt9fphk
        // take() function https://tinyurl.com/6vx7m3k6
        for line in reader.lines().take(num_lines) {
            let result = line.expect("Error in reading file"); // This will panic if errored
            internal_buffer.push(result.clone())
        }

        Ok(Records {
            lines: internal_buffer,
        })
    }

    /**
     * Reads a file batch by batch, and applies a function Fn for each chunk
     */
    pub fn read_file_by_batch(
        &mut self,
        file_path: &str,
        batch_size: usize,
        f: &dyn Fn(Records<String>) -> (),
    ) -> Result<(), Error> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        for chunk in &reader.lines().map_while(Result::ok).chunks(batch_size) {
            f(Records {
                lines: chunk.collect(),
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_FILE_PATH: &str = "test_data/synthetic/tiny.fa";

    #[test]
    fn read_lines() {
        let mut BatchFileReader = BatchFileReader::default();
        let records = BatchFileReader.read_lines(TEST_FILE_PATH, 3).unwrap();
        assert_eq!(3, records.lines.len());
    }

    fn print_function(input: Records<String>) -> () {
        assert_eq!(true, input.size() <= 3);
    }

    #[test]
    fn read_file_batch() {
        let mut BatchFileReader = BatchFileReader::default();
        BatchFileReader
            .read_file_by_batch(TEST_FILE_PATH, 3, &print_function)
            .unwrap();
    }
}
