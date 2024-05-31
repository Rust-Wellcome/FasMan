#![allow(non_snake_case)]

use fasta_manipulation::run;

// https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#separation-of-concerns-for-binary-projects
fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    } else {
        println!("Done!");
    }
}
