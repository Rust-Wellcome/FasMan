pub mod remapping_headers {
    use crate::map_headers;
    use clap::ArgMatches;
    use colored::Colorize;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::iter::Zip;

    use crate::generics::validate_fasta;

    pub fn pull_map_from_tsv(
        map_file: &str,
    ) -> Zip<std::vec::IntoIter<std::string::String>, std::vec::IntoIter<std::string::String>> {
        let file_reader: File = File::open(map_file).expect("CAN'T OPEN FILE");
        let buff_reader: BufReader<File> = BufReader::new(file_reader);

        let mut old_head: Vec<String> = Vec::new();
        let mut new_head: Vec<String> = Vec::new();

        for line in buff_reader.lines() {
            match line {
                Ok(string) => {
                    let mut old_new = string.split('\t');
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

        let mapped_heads: Zip<std::vec::IntoIter<String>, std::vec::IntoIter<String>> =
            new_head.into_iter().zip(old_head);

        return mapped_heads;
    }

    pub fn remapping_head(arguments: std::option::Option<&ArgMatches>) {
        let file: &String = arguments.unwrap().get_one::<String>("fasta-file").unwrap();
        let map_file: &String = arguments.unwrap().get_one::<String>("map-file").unwrap();
        let output: &String = arguments
            .unwrap()
            .get_one::<String>("output-directory")
            .unwrap();

        println!("Mapping headers for file: {}", file);
        println!("Replace headers with string: {}", map_file);

        match validate_fasta(file) {
            Ok(_thing) => {
                let new_map: Zip<std::vec::IntoIter<String>, std::vec::IntoIter<String>> =
                    pull_map_from_tsv(map_file);

                let new_fasta: String = format!("{output}_OH.fasta");

                map_headers::mapping_headers::create_mapped_fasta(file, &new_fasta, new_map);

                println!(
                    "{}\n{}\n\t{}\n",
                    "FASTA HAS BEEN RE-APPED AND REWRITTEN".green(),
                    "FOUND HERE:".green(),
                    &new_fasta.green()
                );
            }
            Err(_) => {
                println!("NOT A VALID FASTA")
            }
        };
    }
}
