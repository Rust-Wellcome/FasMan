pub mod mapping_headers {
    use clap::ArgMatches;
    use colored::Colorize;
    use std::error::Error;
    use std::fmt;
    use std::fs::File;
    use std::io::{BufRead, BufReader, BufWriter, Write};
    use std::iter::Zip;

    use crate::generics::only_keys;
    use crate::generics::validate_fasta;

    #[allow(dead_code)]
    #[derive(Debug, Clone)]
    struct EmptyVec;
    impl Error for EmptyVec {}

    impl fmt::Display for EmptyVec {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Can't Display Empty Vec")
        }
    }

    #[allow(clippy::explicit_counter_loop)]
    pub fn create_mapping(
        name_vec: Vec<std::string::String>,
        new_name: &str,
    ) -> Zip<std::vec::IntoIter<std::string::String>, std::vec::IntoIter<std::string::String>> {
        // Generate a new mapping for the Fasta
        //
        let mut new_heads: Vec<String> = Vec::new();
        let mut head_counter: i32 = 0;
        let name_vec_clone = name_vec.clone();

        for _x in name_vec {
            new_heads.push(format!("{}_{}", new_name, head_counter));
            head_counter += 1;
        }

        let mapped_heads: Zip<std::vec::IntoIter<String>, std::vec::IntoIter<String>> =
            name_vec_clone.into_iter().zip(new_heads);

        mapped_heads
    }

    pub fn save_mapping(
        output: &str,
        mapped: Zip<
            std::vec::IntoIter<std::string::String>,
            std::vec::IntoIter<std::string::String>,
        >,
    ) {
        // Save the header mapping to file
        let f: File = File::create(output).expect("Unable to create file");
        let mut f: BufWriter<File> = BufWriter::new(f);
        for map_pair in mapped {
            let line: String = format!("{}\t{}\n", map_pair.0, map_pair.1);
            f.write_all(&line.into_bytes())
                .expect("Unable to write data");
        }
    }

    #[allow(unused_mut)]
    pub fn create_mapped_fasta(
        input: &str,
        output: &str,
        mapped: Zip<
            std::vec::IntoIter<std::string::String>,
            std::vec::IntoIter<std::string::String>,
        >,
    ) {
        // Swap out the old with the new
        // skip all else.
        // This could be re-written now that I know more about noodles
        let file_reader: File = File::open(input).expect("CAN'T OPEN FILE");
        let buff_reader: BufReader<File> = BufReader::new(file_reader);
        let mut new_fasta: File = File::create(output).unwrap();

        for line in buff_reader.lines() {
            let l: &str = &line.as_ref().unwrap()[..];
            if l.starts_with('>') {
                let mut to_replace = l.replace('>', "");
                let mut mapped_heads: Zip<std::vec::IntoIter<String>, std::vec::IntoIter<String>> =
                    mapped.clone();
                let mut map: Option<(String, String)> =
                    mapped_heads.find(|x: &(String, String)| x.0 == to_replace);
                let mut new_head: String = map.expect("").1;
                let fmt_head: String = format!(">{}\n", new_head);
                let _ = new_fasta.write_all(&fmt_head.into_bytes());
            } else {
                let mut seq = line.expect("");
                let fmt_seq = format!("{}\n", seq);
                let _ = new_fasta.write_all(&fmt_seq.into_bytes());
            }
        }
    }

    pub fn map_fasta_head(
        arguments: std::option::Option<&ArgMatches>,
    ) -> Result<(), Box<dyn Error>> {
        // Generate a mapped.txt with the old and new headers
        // Generate a mapped.fasta with the new headers
        let file: &String = arguments.unwrap().get_one::<String>("fasta-file").unwrap();
        let replacer: &String = arguments
            .unwrap()
            .get_one::<String>("replace-with")
            .unwrap();
        let output: &String = arguments
            .unwrap()
            .get_one::<String>("output-directory")
            .unwrap();

        println!("Mapping headers for file: {}", file);
        println!("Replace headers with string: {:?}", &replacer);

        match validate_fasta(file) {
            Ok(names) => {
                // Vec of scaffold names from validate_fasta
                // return only the headers, not the lengths
                let new_names = Vec::from_iter(only_keys(names));

                // Generate a Zip of the a=old and new names
                let new_map: Zip<std::vec::IntoIter<String>, std::vec::IntoIter<String>> =
                    create_mapping(new_names, replacer);

                // Save the mapping to file
                let map_to_save: Zip<std::vec::IntoIter<String>, std::vec::IntoIter<String>> =
                    new_map.clone();
                let output_file = format!("{}mapped-heads.tsv", output);
                save_mapping(&output_file, map_to_save);

                // Generate a new fasta with the mapped headers
                let new_fasta: String = format!("{output}mapped.fasta");
                create_mapped_fasta(file, &new_fasta, new_map);

                println!(
                    "{}\n{}\n\t{}\n\t{}",
                    "FASTA HAS BEEN MAPPED AND REWRITTEN".green(),
                    "FOUND HERE:".green(),
                    &new_fasta.green(),
                    &output_file.green()
                );
            }

            Err(e) => panic!("Something is wrong with the file! | {}", e),
        };

        Ok(())
    }
}
