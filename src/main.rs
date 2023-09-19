use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use clap::{command, Arg};



fn main() -> Result<(), Error> {
    let match_result = command!()
    .arg(
        Arg::new("fasta-file")
            .aliases(["fasta"])
            .required(true)
            .help("A path to a valid fasta file.")
    )
    .arg(
        Arg::new("cdna_count")
            .short('a')
            .long("cdna")
            .aliases(["cdna"])
            .help("How many cdna sequences per file")
    )
    .arg(
        Arg::new("rna_count")
            .short('b')
            .long("rna")
            .aliases(["rna"])
            .help("How many rna sequences per file")
    )
    .arg(
        Arg::new("cds_count")
            .short('c')
            .long("cds")
            .aliases(["cds"])
            .help("How many cds sequences per file")
    )
    .arg(
        Arg::new("pep_count")
            .short('d')
            .long("pep")
            .aliases(["pep"])
            .help("How many pep sequences per file")
    ).get_matches();

    println!("{:?}", match_result);

    let path = "test.fasta";
    let chunk = 2;
    let mut counter = 0;

    //let mut output = File::create(path)?;
    //write!(output, "Rust\n💖\nFun")?;

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        if counter != chunk {
            if line?.starts_with('>') {
                println!("header");
            } else {
                println!("Sequence");
                counter += 1;
            }
        } else {
            counter = 0;
            println!("CHUNK");
        }
    }

    Ok(())
}