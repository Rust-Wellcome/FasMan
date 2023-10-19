/* pub mod remap_head {
    use std::{error::Error, fs::File, result};
    use std::iter::Zip;
    use clap::ArgMatches;
    use colored::Colorize;
    use crate::map_headers;

    pub fn pull_map_from_tsv (map: &str) -> Zip<std::vec::IntoIter<std::string::String>, std::vec::IntoIter<std::string::String>> {
        // open tsv

        // take tsv as two lists

        // zip together like map_headers

        // emit
    }

    pub fn remapping_headers (arguments: std::option::Option<&ArgMatches>) -> Result<(), Box<dyn Error>> {
        let file: &String = arguments.unwrap().get_one::<String>("fasta-file").unwrap();
        let map_file: &String = arguments.unwrap().get_one::<String>("map-file").unwrap();
        let output: &String = arguments.unwrap().get_one::<String>("output-directory").unwrap();
        
        println!("Mapping headers for file: {}", file);
        println!("Replace headers with string: {}", map_file);
    
        let valid: Result<Vec<String>, Box<dyn Error>> = map_headers::map_headers::validate_fasta(&file);
        let valid_fasta = match &valid {
            Ok(thing) => {
                println!("Fasta is Valid!")
            }
            Err(_) => {
                println!("")
            }
        };

        let new_map = pull_map_from_tsv();

        let new_fasta: String = format!("{output}_OH.fasta");

        let mapped = map_headers::map_headers::create_mapped_fasta(file, &new_fasta, new_map);

        println!("{}\n{}\n\t{}\n\t{}", "FASTA HAS BEEN MAPPED AND REWRITTEN".green(), "FOUND HERE:".green(), &new_fasta.green(), &output_file.green());



        Ok(())
    }
} */