pub mod remap_head {
    use std::{error::Error, fs::File};
    use std::io::{BufRead, BufReader};
    use std::iter::Zip;
    use clap::ArgMatches;
    use colored::Colorize;
    use crate::map_headers;

    pub fn pull_map_from_tsv (map_file: &str) -> Zip<std::vec::IntoIter<std::string::String>, std::vec::IntoIter<std::string::String>> {
        let file_reader: File = File::open(map_file).expect("CAN'T OPEN FILE");
        let buff_reader: BufReader<File> = BufReader::new(file_reader);

        let mut old_head: Vec<String> = Vec::new();
        let mut new_head: Vec<String> = Vec::new();

        for line in buff_reader.lines() {
            let old_new = match line {
                Ok(string) => {
                    let mut old_new = string.split("\t");
                    let x = old_new.next().unwrap();
                    let y = old_new.next().unwrap();
                    old_head.push(x.to_string());
                    new_head.push(y.to_string());
                }
                Err(_) => {
                    print!("")
                }
            };
        }

        let mapped_heads: Zip<std::vec::IntoIter<String>, std::vec::IntoIter<String>> = new_head.to_owned().into_iter().zip(old_head);
        print!("{:?}", mapped_heads);

        return mapped_heads

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

        let new_map:Zip<std::vec::IntoIter<String>, std::vec::IntoIter<String>> = pull_map_from_tsv(map_file);

        let new_fasta: String = format!("{output}_OH.fasta");

        let _ = map_headers::map_headers::create_mapped_fasta(file, &new_fasta, new_map);

        println!("{}\n{}\n\t{}\n", "FASTA HAS BEEN RE-APPED AND REWRITTEN".green(), "FOUND HERE:".green(), &new_fasta.green());

        Ok(())
    }
}