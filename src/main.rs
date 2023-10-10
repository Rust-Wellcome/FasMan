#![allow(non_snake_case)]

use std::env;
use std::io::Error;
use colored::Colorize;
use clap::{Command, command, Arg};

mod yaml_validator;
use crate::yaml_validator::yaml_validator::validate_yaml;

mod map_headers;
use crate::map_headers::map_headers::map_fasta_head;

mod split_by_size;
use crate::split_by_size::split_by_size::split_file_by_size;

mod split_by_count;
use crate::split_by_count::split_by_count::split_file_by_count;

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
            .arg(
                Arg::new("output")
                    .short('o')
                    .aliases(["out"])
                    .required(false)
                    .default_value("./")
                    .help("Output the log to file")
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
                Arg::new("output-directory")
                    .short('o')
                    .aliases(["out"])
                    .required(false)
                    .default_value("./")
                    .help("The output directory that files will be placed in")
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
            .arg(
                Arg::new("output-directory")
                    .short('o')
                    .aliases(["out"])
                    .required(false)
                    .default_value("./")
                    .help("The output directory that files will be placed in")
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
                Arg::new("output-directory")
                    .short('o')
                    .aliases(["out"])
                    .required(false)
                    .default_value("./")
                    .help("The output directory which will contain the mapped-heads.txt as well as the *mapped.fasta")
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
        "windows" => {
            path_sep = "\\";
            println!("Changing path separators, because windows...")
        },
        "macos" => println!("Supported: Basically linux"),
        "linux" => println!("Supported: Linux"),
        _ => ()
    };

    println!("RUNNING : {:?} : SUBCOMMAND", match_result.subcommand_name().unwrap());

    match match_result.subcommand_name() {
        Some("splitbycount") => {
            let arguments = match_result.subcommand_matches("splitbycount");
            let _ = split_file_by_count(arguments, path_sep);
        },
        Some("splitbysize") => {
            let arguments: Option<&clap::ArgMatches> = match_result.subcommand_matches("splitbysize");
            let _ = split_file_by_size(arguments, path_sep);
        }, 
        Some("mapheaders") => {
            let arguments: Option<&clap::ArgMatches> = match_result.subcommand_matches("mapheaders");
            let _ = map_fasta_head(arguments);
        },
        Some("validateyaml") => {
            let arguments = match_result.subcommand_matches("validateyaml");
            let _ = validate_yaml(arguments, path_sep);
        },
        _ => {
            println!{"NOT A COMMAND"}
        },
    };
    Ok(())
}