#![allow(non_snake_case)]

use clap::{command, Arg, Command};
use colored::Colorize;
use std::env;
use std::io::Error;

mod yaml_validator;
use crate::yaml_validator::yaml_validator_mod::validate_yaml;

mod map_headers;
use crate::map_headers::mapping_headers::map_fasta_head;

mod remap_head;
use crate::remap_head::remapping_headers::remapping_head;

mod split_by_size;
use crate::split_by_size::split_by_size_mod::split_file_by_size;

mod split_by_count;
use crate::split_by_count::split_by_count_mod::split_file_by_count;

mod generics;
//use crate::generics::validate_fasta;

mod tpf_fasta;
use crate::tpf_fasta::tpf_fasta_mod::curate_fasta;

mod filter_fasta;
use crate::filter_fasta::filter_fasta_mod::filter_fasta;

fn main() -> Result<(), Error> {
    let split_options = ["pep", "cds", "cdna", "rna", "other"];
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
                Arg::new("output_to_file")
                    .short('f')
                    .value_parser(clap::builder::BoolishValueParser::new())
                    .default_value(std::ffi::OsStr::new("true"))
                    .help("Output the log to file")
            )
            .arg(
                Arg::new("output_to_stdout")
                    .short('s')
                    .value_parser(clap::builder::BoolishValueParser::new())
                    .default_value(std::ffi::OsStr::new("true"))
                    .help("Output the log to file")
            )
            .arg(
                Arg::new("output_to_pipeline")
                    .short('p')
                    .value_parser(clap::builder::BoolishValueParser::new())
                    .default_value(std::ffi::OsStr::new("true"))
                    .help("Output the log to file")
            )
    )
    .subcommand(
        Command::new("splitbycount")
            .about("Subcommand for splitting fasta files by number of sequence-header pairs, e.g., 100 pairs per file")
            .arg(
                Arg::new("fasta-file")
                    .short('f')
                    .required(true)
                    .help("A path to a valid fasta file.")
            )
            .arg(
                Arg::new("output-directory")
                    .short('o')
                    .default_value("./")
                    .help("The output directory that files will be placed in | outfile will be formatted like {input_file_prefix}_f{file_count}_c{requested_chunk_count}-a{actual_chunk_count}.fa")
            )
            .arg(
                Arg::new("data_type")
                    .short('d')
                    .value_parser(clap::builder::PossibleValuesParser::new(split_options))
                    .help("The data type of the input data")
            )
            .arg(
                Arg::new("sanitise")
                    .short('s')
                    .value_parser(clap::value_parser!(bool))
                    .help("Do we need to sanitise the headers of the input fasta")
            )
            .arg(
                Arg::new("count")
                    .short('c')
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
                    .required(true)
                    .help("A path to a valid fasta file.")
            )
            .arg(
                Arg::new("mem-size")
                    .short('m')
                    .required(true)
                    .value_parser(clap::value_parser!(usize))
                    .help("Size in MB that a fasta file is to be chunked into")
            )
            .arg(
                Arg::new("data_type")
                    .short('d')
                    .value_parser(clap::builder::PossibleValuesParser::new(split_options))
                    .help("The data type of the input data")
            )
            .arg(
                Arg::new("sanitise")
                    .short('s')
                    .value_parser(clap::value_parser!(bool))
                    .help("Do we need to sanitise the headers of the input fasta")
            )
            .arg(
                Arg::new("output-directory")
                    .short('o')
                    .default_value("./")
                    .help("The output directory that files will be placed in")
            )
    )
    .subcommand(
        Command::new("geneset_csvs")
            .about("Subcommand to generate csv files that condense geneset directories generated by splitbycount/splitbysize. Mainly for use in TreeVal")
            .arg(
                Arg::new("geneset_dir")
                    .short('d')
                    .required(true)
                    .help("The path to the top level directory of your geneset directory.")
            )
            .arg(
                Arg::new("specifiy_clade")
                    .short('c')
                    .required(true)
                    .default_value("ALL")
                    .help("Specify the clade folder to refresh")
            )
    )
    .subcommand(
        Command::new("mapheaders")
            .about("Subcommand for stripping out headers and replacing with a standardised automatic or user-given string, this also returns a dict of old:new headers")
            .arg(
                Arg::new("fasta-file")
                    .short('f')
                    .required(true)
                    .help("A path to a valid fasta file.")
            )
            .arg(
                Arg::new("output-directory")
                    .short('o')
                    .default_value("./")
                    .help("The output directory which will contain the mapped-heads.txt as well as the *mapped.fasta")
            )
            .arg(
                Arg::new("replace-with")
                    .short('r')
                    .default_value("FMMH")
                    .help("The new header format, appended with a numerical value. Without being set the new header will default to 'FMMH_{numberical}'")
            )
    )
    .subcommand(
        Command::new("remapheaders")
            .about("Subcommand for stripping out previously mapped headers and replacing with the old headers")
            .arg(
                Arg::new("fasta-file")
                    .short('f')
                    .required(true)
                    .help("A path to a valid fasta file.")
            )
            .arg(
                Arg::new("output-directory")
                    .short('o')
                    .default_value("./new")
                    .help("The output directory which will contain the mapped-heads.txt as well as the *mapped.fasta")
            )
            .arg(
                Arg::new("map-file")
                    .short('m')
                    .required(true)
                    .help("The original mapped header field, a TSV of old-header, new-header")
            )
    )
    .subcommand(
        Command::new("profile")
        .about("Profile an input fasta file and return various statistics")
        .arg(
            Arg::new("fasta-file")
                .short('f')
                .required(true)
                .help("The input fasta file for profiling")
        )
        .arg(
            Arg::new("output-dir")
                .short('o')
                .default_value("FasMan-out")
                .help("The input fasta file for profiling")
        )
    )
    .subcommand(
        Command::new("curate")
        .about("Convert an tpf file and original fasta file into a fasta file - useful for curation")
        .arg(
            Arg::new("fasta")
                .short('f')
                .required(true)
                .help("The input fasta file for re-organising")
        )
        .arg(
            Arg::new("tpf")
                .short('t')
                .required(true)
                .help("The TPF file used to re-organise the input fasta")
        )
        .arg(
            Arg::new("sort")
                .short('s')
                .value_parser(clap::value_parser!(bool))
                .default_value("false")
                .help("Size sort the output or leave as order in AGP")
        )
        .arg(
            Arg::new("output")
                .short('o')
                .default_value("new.fasta")
                .help("The output name of the new fasta file")
        )
        .arg(
            Arg::new("n_length")
                .value_parser(clap::value_parser!(usize))
                .default_value("200")
                .help("Length that the N (gap) string should be.")
        )
    )
    .subcommand(
        Command::new("subset")
        .about("Subset a fasta file in a random manner by percentage of file")
        .arg(
            Arg::new("fasta-file")
                .short('f')
                .required(true)
                .help("The input fasta file for profiling")
        )
        .arg(
            Arg::new("random")
                .short('r')
                .value_parser(clap::value_parser!(bool))
                .help("Random subset of input file. Default skims the first X given percent")
        )
        .arg(
            Arg::new("percent")
                .short('p')
                .value_parser(clap::value_parser!(u16))
                .default_value("50")
                .help("Percentage of the original file entries that should be retained")
        )
    )
    .subcommand(
        Command::new("filterfasta")
            .about("Filter a given list of sequences from fasta file")
            .arg(
                Arg::new("fasta")
                    .required(true)
                    .help("A fasta file for processing")
            )
            .arg(
                Arg::new("output")
                    .short('o')
                    .default_value("FiilteredFasta.fa")
                    .help("The outfile naming")
            )
            .arg(
                Arg::new("filter_list")
                    .short('l')
                    .required(false)
                    .default_value("None")
                    .help("A string comma-separated list of sequence names to exclude from the final fasta")
            )
            .arg(
                Arg::new("filter_file")
                    .short('f')
                    .required(false)
                    .default_value("None")
                    .help("A txt file (such as names.lst) with a sequence header per line to exclude from a final fasta file")
            )
    )
    .subcommand(
        Command::new("mergehaps")
        .about("Merge haplotypes / multi fasta files together")
        .arg(
            Arg::new("fasta-1")
                .short('p')
                .required(true)
                .help("The input fasta file for re-organising")
        )
        .arg(
            Arg::new("fasta-2")
                .short('s')
                .required(true)
                .help("The second input fasta file")
        )
        .arg(
            Arg::new("naming")
                .short('s')
                .default_value("PRI/HAP")
                .help("A '/' separated list with an item per file, these are the namings of the new scaffolds in the merged output")
        )
        .arg(
            Arg::new("output")
                .short('o')
                .default_value("merged")
                .help("Output file prefix")
        )
    )
    .get_matches();

    println! {
        "{}\n{}\n{}\nRUNNING SUBCOMMAND: |\n-- {}\nRUNNING ON: |\n-- {}",
        "WELCOME TO Fasta Manipulator".bold(),
        "This has been made to help prep data for use in the Treeval and curationpretext pipelines".bold(),
        "ONLY THE yamlvalidator IS SPECIFIC TO TREEVAL, THE OTHER COMMANDS CAN BE USED FOR ANY OTHER PURPOSE YOU WANT".purple(),
        match_result.subcommand_name().unwrap(),
        env::consts::OS
    };

    match match_result.subcommand_name() {
        // Should really be pulled out into it's own program
        // Validator for YAML file for TreeVal and potentially CurationPretext
        Some("validateyaml") => validate_yaml(match_result.subcommand_matches("validateyaml")),

        // FASTA Manipulator modules
        Some("splitbysize") => split_file_by_size(match_result.subcommand_matches("splitbysize")),
        Some("splitbycount") => {
            split_file_by_count(match_result.subcommand_matches("splitbycount"))
        }
        //Some("subset") => subset(match_result.subcommand_matches("subset"))
        //Some("profile") => profile(match_result.subcommand_matches("profile"))
        Some("mapheaders") => {
            _ = map_fasta_head(match_result.subcommand_matches("mapheaders"));
        }
        Some("remapheaders") => remapping_head(match_result.subcommand_matches("remapheaders")),
        Some("filterfasta") => filter_fasta(match_result.subcommand_matches("filterfasta")),

        // FASTA + TPF = NEW_FASTA
        Some("curate") => curate_fasta(match_result.subcommand_matches("curate")),

        _ => {
            unreachable!()
        }
    };
    Ok(())
}
