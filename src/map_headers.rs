pub mod mapping_headers {
    use std::io::{BufWriter, Write, BufRead, BufReader};
    use std::error::Error;
    use std::iter::Zip;
    use std::fs::File;
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
        // Simply validate the fasta is valid by reading though and ensure there are
        // valid record formats through out the file
        let reader: Result<fasta::Reader<Box<dyn BufRead>>, std::io::Error> = fasta::reader::Builder.build_from_path(path);
        let result = match &reader {
            Ok(names) => {
                let mut binding: fasta::Reader<Box<dyn BufRead>> = reader.expect("NO VALID HEADER / SEQUENCE PAIRS");
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
        // Generate a new mapping for the Fasta
        //
        let mut new_heads: Vec<String> = Vec::new();
        let mut head_counter: i32 = 0;
        let name_vec_clone = name_vec.clone();

        for x in name_vec{
            new_heads.push(format!("{}_{}", new_name, head_counter));
            head_counter += 1;
        }

        let mapped_heads: Zip<std::vec::IntoIter<String>, std::vec::IntoIter<String>> = name_vec_clone.to_owned().into_iter().zip(new_heads);

        return mapped_heads
    }

    pub fn save_mapping(output: &str, mapped: Zip<std::vec::IntoIter<std::string::String>, std::vec::IntoIter<std::string::String>>) {
        let f: File = File::create(output).expect("Unable to create file");
        let mut f: BufWriter<File> = BufWriter::new(f);
        for map_pair in mapped {
            let line: String = format!("{}\t{}\n", map_pair.0, map_pair.1);
            f.write_all(&line.into_bytes()).expect("Unable to write data");
        };
    }

    pub fn create_mapped_fasta(input: &str, output: &str, mapped: Zip<std::vec::IntoIter<std::string::String>, std::vec::IntoIter<std::string::String>>) {
        let file_reader: File = File::open(input).expect("CAN'T OPEN FILE");
        let buff_reader: BufReader<File> = BufReader::new(file_reader);
        let mut new_fasta: File = File::create(output).unwrap();
        print!("{:?}", mapped);

        for line in buff_reader.lines() {
            let l: &str = &line.as_ref().unwrap()[..];
            if l.starts_with(">") {
                let mut to_replace = l.replace(">","");
                let mut mapped_heads: Zip<std::vec::IntoIter<String>, std::vec::IntoIter<String>> = mapped.clone();
                let mut map: Option<(String, String)> = mapped_heads.find(|x: &(String, String)| x.0 == to_replace);
                let mut new_head: String = map.expect("").1;
                let fmt_head: String = format!(">{}\n", new_head);
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

        let name_vec: Result<Vec<String>, Box<dyn Error>> = validate_fasta(file);

        let names: Vec<String> = match name_vec {
            Ok(names) => names,
            Err(_e) => return Err(EmptyVec.into())
        };

        let new_map: Zip<std::vec::IntoIter<String>, std::vec::IntoIter<String>> = create_mapping(names, replacer);

        let map_to_save: Zip<std::vec::IntoIter<String>, std::vec::IntoIter<String>> = new_map.clone();
        let output_file = format!("{}mapped-heads.tsv", output);

        save_mapping(&output_file, map_to_save);

        let new_fasta: String = format!("{output}mapped.fasta");

        let _ = create_mapped_fasta(file, &new_fasta, new_map);

        println!("{}\n{}\n\t{}\n\t{}", "FASTA HAS BEEN MAPPED AND REWRITTEN".green(), "FOUND HERE:".green(), &new_fasta.green(), &output_file.green());
        Ok(())
    }

}