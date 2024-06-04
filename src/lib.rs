use clap::Parser;

use cli::{Cli, Commands};
use std::io::Error;

// Reference: https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
use crate::processors::exclude_seq::filter_fasta;
use crate::processors::map_headers::map_fasta_head;
use crate::processors::remap_head::remapping_head;
use crate::processors::split_by_count::split_file_by_count;
use crate::processors::split_by_size::split_file_by_size;
use crate::processors::tpf_fasta::curate_fasta;
use crate::processors::yaml_validator::validate_yaml;

mod cli;
mod errors;
mod file_utils;
mod generics;

mod processors;

pub fn run() -> Result<(), Error> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::YamlValidator {
            yaml,
            verbose,
            output,
        }) => validate_yaml(yaml, verbose, output),
        Some(Commands::SplitByCount {
            fasta_file,
            output_directory,
            data_type,
            sanitise,
            count,
        }) => split_file_by_count(fasta_file, output_directory, data_type, sanitise, count),
        Some(Commands::SplitBySize {
            fasta_file,
            mem_size,
            output_directory,
        }) => split_file_by_size(fasta_file, mem_size, output_directory),
        Some(Commands::MapHeaders {
            fasta_file,
            output_directory,
            replace_with,
        }) => _ = map_fasta_head(fasta_file, output_directory, replace_with),
        Some(Commands::ReMapHeaders {
            fasta_file,
            output_directory,
            map_file,
        }) => remapping_head(fasta_file, output_directory, map_file),
        Some(Commands::Curate {
            fasta,
            tpf,
            sort,
            output,
            n_length,
        }) => curate_fasta(fasta, tpf, sort, output, n_length),
        Some(Commands::FilterFasta {
            fasta,
            output,
            filter_list,
        }) => filter_fasta(fasta, output, filter_list),
        Some(Commands::GenesetCSVS { .. }) => {
            todo!()
        }
        Some(Commands::Profile { .. }) => {
            todo!()
        }
        Some(Commands::Subset { .. }) => {
            todo!()
        }
        Some(Commands::Mergehaps { .. }) => {
            todo!()
        }
        None => {
            println!("No command provided");
        }
    }
    Ok(())
}
