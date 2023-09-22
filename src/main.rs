use std::fs::File;
use std::env;
use std::io::{BufReader, BufRead, Error};
use clap::{Command, command, Arg};


fn splitbycount(file: &str, chunk: &u16) -> Result<(), std::io::Error> {
    println!("hello");
    //if bycount {
    //    println!("Count to split by: {}", bycount.unwrap().get_one::<String>("count").unwrap());
   // }
    // returns bool bycount.unwrap().contains_id("count");
    
    // ---
    let  chunk_val = chunk.clone();
    let mut counter = 0;
    let mut global_counter = 0;

    //let mut output = File::create(path)?;
    //write!(output, "Rust\n💖\nFun")?;

    let input = File::open(file)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        if counter != chunk_val {
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

fn splitbysize() -> Result<(), std::io::Error> {
    println!("hello");

    Ok(())
}

fn mapheaders() -> Result<(), std::io::Error> {
    println!("hello");

    Ok(())
}

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
                    .value_parser(clap::value_parser!(u16))
                    .help("How many sequences per file")
            )
    )
    .subcommand(
        Command::new("splitbysize")
            .about("Command for splitting fasta files by user given size (in MegaBytes) into n (fasta_size / user_given_size) files")
            .arg(
                Arg::new("fasta-file")
                    .short('f')
                    .aliases(["fasta"])
                    .required(true)
                    .help("A path to a valid fasta file.")
            )
            .arg(
                Arg::new("mem-size")
                    .short('s')
                    .long("mem-size")
                    .required(true)
                    .value_parser(clap::value_parser!(u16))
                    .help("Size in MB that a fasta file is to be chunked into")
            )
    )
    .subcommand(
        Command::new("mapheaders")
            .about("Command for stripping out headers and replacing with a standardised automatic or user-given string, this also returns a dict of old:new headers")
            .arg(
                Arg::new("fasta-file")
                    .short('f')
                    .aliases(["fasta"])
                    .required(true)
                    .help("A path to a valid fasta file.")
            )
            .arg(
                Arg::new("replace-with")
                    .short('r')
                    .aliases(["replacement"])
                    .required(false)
                    .default_value("FMMH")
                    .help("The new header format, appended with a numerical value. Without being set the new header will default to 'FM_{numberical}'")
            )
    )
    .get_matches();

    //println!("{}", &match_result.get_one::<String>("count").unwrap_or(&"DEFAULT".to_string()));

    let args: Vec<String> = env::args().collect();

    if &args[1].to_string() == "splitbycount" {
        let arguments = match_result.subcommand_matches("splitbycount");
        let fasta_file = arguments.unwrap().get_one::<String>("fasta-file").unwrap();
        let fasta_count = arguments.unwrap().get_one::<u16>("count").unwrap();
        println!("Fasta file for processing: {:?}", fasta_file);
        println!("{:?}", &fasta_count);
        println!("Number of sequence-header pairs per file: {:?}", fasta_count);
        splitbycount(fasta_file, &fasta_count);
    } else if &args[1].to_string() == "splitbysize" {
        let arguments = match_result.subcommand_matches("splitbysize");
        println!("Fasta file for processing: {:?}", arguments.unwrap().get_one::<String>("fasta-file").unwrap());
        println!("Size to chunk fasta into: {:?}", arguments.unwrap().get_one::<u16>("mem-size").unwrap());
        splitbysize();
    } else if &args[1].to_string() == "mapheaders" {
        let arguments = match_result.subcommand_matches("mapheaders");
        println!("Fasta file for processing: {:?}", arguments.unwrap().get_one::<String>("fasta-file").unwrap());
        println!("Replace headers with string: {:?}", arguments.unwrap().get_one::<String>("replace-with").unwrap());
        mapheaders();
    };

    Ok(())
}