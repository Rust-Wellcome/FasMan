#![allow(non_snake_case)]

use std::fs::File;
use std::env;
use std::io::{BufReader, BufRead, Error};
use colored::Colorize;
use clap::{Command, command, Arg};
//use serde::{Serialize, Deserialize};

mod yamlvalidator;
use crate::yamlvalidator::yamlvalidator::validateyaml;

mod mapheaders;
use crate::mapheaders::mapheaders::mapfastahead;


fn splitbycount(file: &str, chunk: &u16, _sep: &str) -> Result<(), std::io::Error> {
    println!("Splitting file: {}", file);
    println!("Splitting bycount: {}", chunk);
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
                global_counter += 1;
            }
        } else {
            counter = 0;
            println!("CHUNK");
        }
    }
    println!("Total number of pairs: {:?}", global_counter);
    Ok(())
}

fn splitbysize(file: &str, _sep: &str) -> Result<(), std::io::Error> {
    println!("Splitting file: {}", file);

    Ok(())
}

fn main() -> Result<(), Error> {
    let match_result = command!()
    .about("A program for fasta manipulation and yaml validation ~ Used in TreeVal project")
    .subcommand(
        Command::new("validateyaml")
            .about("Subcommand for validating the users TreeVal yaml file")
            .arg(
                Arg::new("yaml")
                    .required(true)
                    .help("Path to the TreeVal yaml file generated by the user")
            )
            .arg(
                Arg::new("verbose")
                    .short('v')
                    .value_parser(clap::value_parser!(bool))
                    .default_value("false")
                    .required(false)
                    .help("Print explainers as to why validation fails, if it does fail")
            )
    )
    .subcommand(
        Command::new("splitbycount")
            .about("Subcommand for splitting fasta files by number of sequence-header pairs, e.g., 100 pairs per file")
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
            .about("Subcommand for splitting fasta files by user given size (in MegaBytes) into n (fasta_size / user_given_size) files")
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
            .about("Subcommand for stripping out headers and replacing with a standardised automatic or user-given string, this also returns a dict of old:new headers")
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

    println!{
        "{}\n{}\n{}",
        "WELLCOME TO TreeVal Data Prepper".bold().purple(),
        "This has been made to help prep data for use in the Treeval and curationpretext pipelines",
        "ONLY THE yamlvalidator IS SPECIFIC TO TREEVAL, THE OTHER COMMANDS CAN BE USED FOR ANY OTHER PURPOSE YOU WANT".purple()
    };
    println!("OPERATING SYSTEM: {}", env::consts::OS.purple()); // Prints the current OS.
    let mut path_sep = "/";
    match env::consts::OS {
        "windows" => path_sep = "\\",
        _ => ()
    };

    println!("RUNNING : {:?} : SUBCOMMAND", match_result.subcommand_name().unwrap());

    // Should really be using this: https://docs.rs/clap/latest/clap/struct.ArgMatches.html#method.subcommand
    match match_result.subcommand_name() {
        Some("splitbycount") => {
            let arguments = match_result.subcommand_matches("splitbycount");
            let fasta_file = arguments.unwrap().get_one::<String>("fasta-file").unwrap();
            let fasta_count = arguments.unwrap().get_one::<u16>("count").unwrap();
            println!("Fasta file for processing: {:?}", fasta_file);
            println!("{:?}", &fasta_count);
            println!("Number of sequence-header pairs per file: {:?}", fasta_count);
            let _ = splitbycount(fasta_file, &fasta_count, path_sep);
        },
        Some("splitbysize") => {
            let arguments = match_result.subcommand_matches("splitbysize");
            let fasta_file = arguments.unwrap().get_one::<String>("fasta-file").unwrap();
            println!("Fasta file for processing: {:?}", arguments.unwrap().get_one::<String>("fasta-file").unwrap());
            println!("Size to chunk fasta into: {:?}", arguments.unwrap().get_one::<u16>("mem-size").unwrap());
            let _ = splitbysize(fasta_file, path_sep);
        }, 
        Some("mapheaders") => {
            let arguments = match_result.subcommand_matches("mapheaders");
            let fasta_file = arguments.unwrap().get_one::<String>("fasta-file").unwrap();
            println!("Fasta file for processing: {:?}", arguments.unwrap().get_one::<String>("fasta-file").unwrap());
            println!("Replace headers with string: {:?}", arguments.unwrap().get_one::<String>("replace-with").unwrap());
            let _ = mapheaders(fasta_file, path_sep);
        },
         Some("validateyaml") => {
            let arguments = match_result.subcommand_matches("validateyaml");
            let yaml_file = arguments.unwrap().get_one::<String>("yaml").unwrap();
            let verbose_flag = arguments.unwrap().get_one::<bool>("verbose").unwrap();
            let _ = validateyaml(yaml_file, verbose_flag, path_sep);
        },
        _ => {
            println!{"NOT A COMMAND"}
        },
    };
    Ok(())
}