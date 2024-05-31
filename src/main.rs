#![allow(non_snake_case)]

use std::io::Error;

use fasta_manipulation::run;

// https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#separation-of-concerns-for-binary-projects
fn main() -> Result<(), Error> {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    } else {
        Ok(())
    }
}
