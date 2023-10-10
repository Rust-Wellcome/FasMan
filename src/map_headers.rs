pub mod map_headers {
    use std::io::{BufWriter, Write, BufRead, BufReader};
    use std::error::Error;
    use std::iter::Zip;
    use std::fs::File;
    use std::io::prelude::*;
    use std::fs;

    use clap::ArgMatches;
    use noodles::fasta;
    use colored::Colorize;
    use std::fmt;

    #[derive(Debug, Clone)]
    struct EmptyVec;
    impl Error for EmptyVec {}

    impl fmt::Display for EmptyVec {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Can't Display Empty Vec")
        }
    }


    pub fn validate_fasta(path: &str) -> Result<Vec<std::string::String>, Box<dyn Error>> {
        let reader = fasta::reader::Builder.build_from_path(path);
        let result = match &reader {
            Ok(names) => {
                let mut binding = reader.expect("NO VALID HEADER / SEQUENCE PAIRS");
                let result: fasta::reader::Records<'_, Box<dyn BufRead>> = binding.records();
                let names: Vec<_> = result.flatten().map(|res| res.name().to_owned()).collect();
                return Ok(names)
            },
            Err(_) => {
                return Err(format!("Error...").into())
            }
        };
    }


    pub fn create_mapping(name_vec: Vec<std::string::String>, new_name: &str) -> Zip<std::vec::IntoIter<std::string::String>, std::vec::IntoIter<std::string::String>> {
        let mut new_heads: Vec<String> = Vec::new();
        let mut head_counter: i32 = 0;
        let name_vec_clone = name_vec.clone();

        for x in name_vec{
            new_heads.push(format!("{}_{}", new_name, head_counter));
            head_counter += 1;
        }

        let mapped_heads = name_vec_clone.to_owned().into_iter().zip(new_heads);

        return mapped_heads
    }

    pub fn save_mapping(output: &str, mapped: Zip<std::vec::IntoIter<std::string::String>, std::vec::IntoIter<std::string::String>>) {
        let f = File::create(output).expect("Unable to create file");
        let mut f = BufWriter::new(f);
        for map_pair in mapped {
            let line = format!("{}\t{}\n", map_pair.0, map_pair.1);
            f.write_all(&line.into_bytes()).expect("Unable to write data");
        };
    }

    pub fn create_mapped_fasta(input: &str, output: &str, mapped: Zip<std::vec::IntoIter<std::string::String>, std::vec::IntoIter<std::string::String>>) {
        let file_reader = File::open(input).expect("CAN'T OPEN FILE");
        let buff_reader = BufReader::new(file_reader);
        let mut new_fasta = File::create(output).unwrap();

        for line in buff_reader.lines() {
            let l = &line.as_ref().unwrap()[..];
            if l.starts_with(">") {
                let mut to_replace = l.replace(">","");
                let mut mapped_heads = mapped.clone();
                let mut map = mapped_heads.find(|x| x.0 == to_replace);
                let mut new_head = map.expect("").1;
                let fmt_head = format!(">{}\n", new_head);
                new_fasta.write_all(&fmt_head.into_bytes());
            } else {
                let mut seq = line.expect("");
                let fmt_seq = format!("{}\n", seq);
                new_fasta.write_all(&fmt_seq.into_bytes());
            }
        }
    }

    pub fn map_fasta_head(arguments: std::option::Option<&ArgMatches>) -> Result<(), Box<dyn Error>> {
        let file: &String = arguments.unwrap().get_one::<String>("fasta-file").unwrap();
        let replacer: &String = arguments.unwrap().get_one::<String>("replace-with").unwrap();
        let output: &String = arguments.unwrap().get_one::<String>("output-directory").unwrap();
        
        println!("Mapping headers for file: {}", file);
        println!("Replace headers with string: {:?}", &replacer);
    
        let _ = validate_fasta(file);

        let name_vec = validate_fasta(file);
        
        let names = match name_vec {
            Ok(names) => names,
            Err(_e) => return Err(EmptyVec.into())
        };

        let new_map = create_mapping(names, replacer);

        let map_to_save = new_map.clone();
        let output_file = format!("{}mapped-heads.tsv", output);

        save_mapping(&output_file, map_to_save);

        let new_fasta = format!("{output}mapped.fasta");

        let _ = create_mapped_fasta(file, &new_fasta, new_map);

        println!("{}\n{}\n\t{}\n\t{}", "FASTA HAS BEEN MAPPED AND REWRITTEN".green(), "FOUND HERE:".green(), &new_fasta.green(), &output_file.green());
        Ok(())
    }
    
}