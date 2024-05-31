use log::info;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::errors::file_error::FileError;
use itertools::Itertools;

#[allow(dead_code)]
struct Records<T> {
    items: Vec<T>,
}

#[allow(dead_code)]
impl Records<String> {
    pub fn size(&self) -> usize {
        self.items.len()
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

#[allow(dead_code)]
impl BatchFileReader {
    /*
     * Reads a specific number of lines from a file from the top
     */
    pub fn read_lines(
        &mut self,
        file_path: &str,
        num_lines: usize,
    ) -> Result<Records<String>, FileError> {
        info!("Reading lines in file.");
        let file = File::open(file_path);

        let result = match file {
            Ok(file) => file,
            Err(error) => {
                info!("Error in file handler: {:?}", error);
                return Err(error.into());
            }
        };

        let reader = BufReader::new(result);
        let mut internal_buffer = Vec::<String>::new();

        // Error unwrapping: https://tinyurl.com/brt9fphk
        // take() function https://tinyurl.com/6vx7m3k6
        for line in reader.lines().take(num_lines) {
            let result = line.expect("Error in reading file"); // This will panic if errored
            internal_buffer.push(result.clone())
        }

        Ok(Records {
            items: internal_buffer,
        })
    }

    /**
     * Reads a file batch by batch, and applies a function Fn for each chunk
     * Function pointers documentation: https://doc.rust-lang.org/book/ch19-05-advanced-functions-and-closures.html#function-pointers
     * f is a closure pushed into the stack of read_file_by_batch that is similar to an anonymous function in Java/JavaScript/C#
     * https://doc.rust-lang.org/book/ch13-01-closures.html#moving-captured-values-out-of-closures-and-the-fn-traits
     * Note that f is not intended to mutate the captured Records value, and should not return anything (i.e., move the captured Record value out of the closure).
     */
    pub fn read_file_by_batch(
        &mut self,
        file_path: &str,
        batch_size: usize,
        f: &dyn Fn(Records<String>),
    ) -> Result<(), FileError> {
        info!("Reading file by chunk.");

        let file = File::open(file_path);

        let result = match file {
            Ok(file) => file,
            Err(error) => {
                info!("Error in file handler: {:?}", error);
                return Err(error.into());
            }
        };

        let reader = BufReader::new(result);

        // map_while() Creates an iterator that both yields elements based on a predicate and maps.
        // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map_while
        for chunk in &reader.lines().map_while(Result::ok).chunks(batch_size) {
            f(Records {
                items: chunk.collect(),
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
        let mut batch_file_reader = BatchFileReader::default();
        let records = batch_file_reader.read_lines(TEST_FILE_PATH, 3).unwrap();
        assert_eq!(3, records.items.len());
    }

    // You can create the closure in one place and then call the closure elsewhere to evaluate it in a different context.
    // Reference: https://doc.rust-lang.org/book/ch13-01-closures.html
    fn assert_function(input: Records<String>) {
        assert!(input.size() <= 3);
    }

    #[test]
    fn read_file_batch() {
        let mut batch_file_reader = BatchFileReader::default();
        batch_file_reader
            .read_file_by_batch(TEST_FILE_PATH, 3, &assert_function)
            .unwrap_or_else(|e| panic!("Error: {:?}", e));
    }
}
