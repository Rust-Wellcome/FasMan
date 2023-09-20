use std::fs::File;
use std::env;
use std::io::{BufReader, BufRead, Error};
use clap::{Command, command, Arg};



fn main() -> Result<(), Error> {
    let match_result = command!()
    .about("A program for fasta manipulation ~ Used in TreeVal project")
    .subcommand(
        Command::new("splitbycount")
            .about("Command for splitting fasta files by number of sequence-header pairs, e.g., 100 pairs per file")
            .arg(
                Arg::new("fasta-file")
                    .short('f')
                    .aliases(["fasta"])
                    .required(true)
                    .help("A path to a valid fasta file.")
            )
            .arg(
                Arg::new("count")
                    .short('c')
                    .long("file-count")
                    .aliases(["count"])
                    .help("How many sequences per file")
            )
    )
    .subcommand(
        Command::new("splitbysize")
            .about("Command for splitting fasta files by user given size (in MegaBytes) into n (fasta_size / user_given_size) files")
            .arg(
                Arg::new("fasta-file")
                    .aliases(["fasta"])
                    .required(true)
                    .help("A path to a valid fasta file.")
            )
            .arg(
                Arg::new("mem-size")
                    .required(true)
                    .help("Size in MB that a fasta file is to be chunked into")
            )
    )
    .get_matches();

    //println!("{}", &match_result.get_one::<String>("count").unwrap_or(&"DEFAULT".to_string()));


    let args: Vec<String> = env::args().collect();

    if &args[1].to_string() == "splitbycount" {
        let bycount = match_result.subcommand_matches("splitbycount");
        println!("Number of sequence-header pairs per file: {:?}", bycount.unwrap().get_one::<String>("count").unwrap());
    } else if &args[1].to_string() == "splitbysize" {
        let bysize = match_result.subcommand_matches("splitbysize");
        println!("Size to chunk fasta into: {:?}", bysize);
    };
    //if bycount {
    //    println!("Count to split by: {}", bycount.unwrap().get_one::<String>("count").unwrap());
   // }
    // returns bool bycount.unwrap().contains_id("count");
    
    // ---
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