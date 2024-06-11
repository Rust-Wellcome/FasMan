pub mod tpf_fasta_mod {
    use clap::ArgMatches;
    use noodles::core::Position;
    use noodles::fasta;
    use noodles::fasta::record::Sequence;
    use noodles::fasta::repository::adapters::IndexedReader;
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::{fs::read_to_string, fs::File, str};

    use crate::generics::validate_fasta;

    #[derive(Debug, Clone, PartialEq, Eq)]
    /// STRUCT: Tpf Struct creates a simple object which descibes each line of an input TPF.
    struct Tpf {
        ori_scaffold: String,
        start_coord: usize,
        end_coord: usize,
        new_scaffold: String,
        orientation: String,
    }

    /// Implementing a Disaplay method for Tpf - This was used for printing the original scaffold as well as the original start and end co-ords.
    impl std::fmt::Display for Tpf {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            write!(
                fmt,
                "\t{} -- {} -- {}",
                self.ori_scaffold, self.start_coord, self.end_coord
            )
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    /// STRUCT: NewFasta is a struct that wraps around struct-Tpf as well as the corresponding sequence for that new segment.
    struct NewFasta {
        tpf: Tpf,
        sequence: String,
    }

    #[derive(Debug)]
    /// STRUCT: MyRecord is a Fasta record Struct - Should be replaced by the much better Noodles::record
    struct MyRecord {
        name: String,
        sequence: Vec<String>,
    }

    /// FUNCTION: parse_tpf generates a Vector (akin to a list in python) of Tpf structs which descibes the changes made to a genomic assembly from a program called PretextView and agp-tpf-utils
    fn parse_tpf(path: &String) -> Vec<Tpf> {
        // Instantiate a List of Tpf objects
        let mut all_tpf: Vec<Tpf> = Vec::new();
        for line in read_to_string(path).unwrap().lines() {
            // If line starts with '?' parse line, lines
            // without this are gaps
            if line.starts_with('?') {
                // Parse data into TpF object
                let line_replaced = line.replace('\t', " ");
                let line_list: Vec<&str> = line_replaced.split_whitespace().collect();
                let scaff_data: Vec<&str> = line_list[1].split(':').collect();
                let scaff_coords: Vec<&str> = scaff_data[1].split('-').collect();
                let data = Tpf {
                    ori_scaffold: scaff_data[0].to_owned(),
                    start_coord: scaff_coords[0].to_owned().parse::<usize>().unwrap(),
                    end_coord: scaff_coords[1].to_owned().parse::<usize>().unwrap(),
                    new_scaffold: line_list[2].to_owned().replace("RL", "SUPER"),
                    orientation: line_list[3].to_owned(),
                };
                all_tpf.push(data);
            }
        }
        all_tpf
    }

    /// FUNCTION: subset_vec_tpf subsets the Vector of Tpf dependant on the tuple of scaffold and size of scaffold. Tpf coords must be inside the size..
    fn subset_vec_tpf<'a>(
        tpf: &'a Vec<Tpf>,
        fasta: (&std::string::String, &usize),
    ) -> Vec<&'a Tpf> {
        let mut subset_tpf: Vec<&Tpf> = Vec::new();
        for i in tpf {
            if i.ori_scaffold == *fasta.0 {
                subset_tpf.push(i)
            }
        }
        subset_tpf
    }

    /// FUNCTION: check_orientation ensures that the orientation of an input sequence is always in the PLUS orientation. If in MINUS, then the sequence is complemented and reversed.
    fn check_orientation(
        parsed: std::option::Option<noodles::fasta::record::Sequence>,
        orientation: String,
    ) -> String {
        if orientation == "MINUS" {
            let start = Position::try_from(1).unwrap();
            let parse_orientation = parsed.unwrap();
            let compliment: Sequence = parse_orientation
                .complement()
                .collect::<Result<_, _>>()
                .unwrap();
            let seq = compliment.get(start..).unwrap();
            str::from_utf8(seq).unwrap().chars().rev().collect()
        } else {
            let start = Position::try_from(1).unwrap();
            let parse_orientation = parsed.unwrap();
            let seq = parse_orientation.get(start..).unwrap();
            str::from_utf8(seq).unwrap().chars().collect()
        }
    }

    /// FUNCTION: parse_seq takes an input sequence from the input fasa file and parses it based on a subset Vector of Tpf (e.g scaffold_1)
    fn parse_seq(
        sequence: std::option::Option<noodles::fasta::record::Sequence>,
        tpf: Vec<&Tpf>,
    ) -> Vec<NewFasta> {
        let mut subset_tpf: Vec<NewFasta> = Vec::new();

        let new_seq = sequence.unwrap(); // Option(Sequence ()) -> Sequence ()
        for &i in &tpf {
            let start = Position::try_from(i.start_coord).unwrap();
            let end = Position::try_from(i.end_coord).unwrap();
            let parsed = new_seq.slice(start..=end);
            let the_sequence = check_orientation(parsed, i.orientation.to_owned());
            let data = NewFasta {
                tpf: i.to_owned(),
                sequence: the_sequence,
            };
            subset_tpf.push(data);
        }
        subset_tpf
    }

    /// FUNCTION: get_uniques takes the Vec<Tpf> and moves through it to collect a Vector of unique new_scaffold names
    fn get_uniques(tpf_list: &Vec<Tpf>) -> Vec<String> {
        let mut uniques: Vec<String> = Vec::new();
        for i in tpf_list {
            if !uniques.contains(&i.new_scaffold) {
                uniques.push(i.new_scaffold.to_owned())
            }
        }
        uniques
    }

    /// FUNCTION: save_to_fasta saved the now parsed data, Vec<Tpf> is in the order of the original TPF as we do not directly modify this Vec
    /// The unique list from get_uniques is used at the iterator and output data per unique at a time. This is wholey inefficient however,
    /// If uniques is 10 long and fasta is 100, then this is 1000 scans through the list in total.
    /// There is a file1 and file2 as output, file2 is a human readable file detailing what Tpf records (scaffold5:1000-100000,scaffold1,+) were output to each new scaffold (scaffold1)
    /// The MyRecord struct collects these trimmings per unique header and they are saved to file by being joined by a user definable length of N's
    fn save_to_fasta(
        fasta_data: Vec<NewFasta>,
        tpf_data: Vec<Tpf>,
        output: &String,
        n_length: usize,
    ) {
        let _data_file = File::create(output);
        let mut file = OpenOptions::new()
            .write(true)
            .open(output)
            .expect("creation failed");

        let _debugger = File::create("debug.txt");
        let mut file2 = OpenOptions::new()
            .write(true)
            .open("debug.txt")
            .expect("creation failed");

        let uniques = get_uniques(&tpf_data);

        for x in uniques {
            println!("NOW WRITING DATA FOR: {:?}", &x);
            let stringy = format!(">{x}\n");
            file.write_all(stringy.as_bytes())
                .expect("Unable to write to file");

            file2
                .write_all(stringy.as_bytes())
                .expect("Unable to write to file");

            let mut data: MyRecord = MyRecord {
                name: "".to_string(),
                sequence: Vec::new(),
            };

            x.clone_into(&mut data.name);
            for tpf in &tpf_data {
                if tpf.new_scaffold == x {
                    for fasta in &fasta_data {
                        if fasta.tpf == *tpf {
                            let stringy = format!("\t{}\n", tpf);
                            file2
                                .write_all(stringy.as_bytes())
                                .expect("Unable to write to file");
                            data.sequence.push(fasta.sequence.to_owned());
                        }
                    }
                }
            }

            let line_len: usize = 60;
            let fixed = data.sequence;
            let n_string = "N".repeat(n_length);
            let fixed2 = fixed.join(&n_string); //.join required a borrowed str
            let fixed3 = fixed2
                .as_bytes()
                .chunks(line_len)
                .map(str::from_utf8)
                .collect::<Result<Vec<&str>, _>>()
                .unwrap();

            for i in fixed3 {
                let formatted = i.to_owned() + "\n";
                file.write_all(formatted.as_bytes()).unwrap();
            }
        }
    }

    #[allow(clippy::needless_borrow)]
    #[allow(clippy::let_and_return)]
    /// PUB FUNCTION: curate_fasta is the main of this module/
    /// First a fasta file is validated and returns a list of scaffold header and length of scaffold
    /// Stacker is used to increase the size of the stack incase we run out of memory
    /// The input fasta is then converted into an indexed_reader and returns as a queryable object
    /// For each header in the input fasta, subset the Vec<Tpf>
    /// Query for the sequence in that header and parse it per subset Vec<Tpf> If this doesn't exist then exit program,
    /// one or both files are wrong.
    /// Now collect all data and output to file.
    pub fn curate_fasta(arguments: std::option::Option<&ArgMatches>) {
        let fasta_file: &String = arguments.unwrap().get_one::<String>("fasta").unwrap();
        let tpf_file: &String = arguments.unwrap().get_one::<String>("tpf").unwrap();
        let n_length: &usize = arguments.unwrap().get_one::<usize>("n_length").unwrap();
        let output: &String = arguments.unwrap().get_one::<String>("output").unwrap();
        println!("LET'S GET CURATING THAT FASTA!");

        stacker::maybe_grow(32 * 1024, 1024 * 5120, || {
            match validate_fasta(fasta_file) {
                Ok(fasta_d) => {
                    let tpf_data = parse_tpf(&tpf_file);

                    let reader =
                        fasta::indexed_reader::Builder::default().build_from_path(fasta_file);
                    let fasta_repo = match reader {
                        Ok(data) => {
                            let adapter = IndexedReader::new(data);
                            let repository = fasta::Repository::new(adapter);
                            repository
                        }
                        Err(_) => todo!(), // Probably just panic!
                    };

                    let mut new_fasta_data: Vec<NewFasta> = Vec::new();
                    for i in fasta_d {
                        let subset_tpf = subset_vec_tpf(&tpf_data, (&i.0, &i.1));
                        let sequence = fasta_repo.get(&i.0).transpose();

                        match sequence {
                            Ok(data) => {
                                let subset_results = parse_seq(data, subset_tpf);
                                new_fasta_data.extend(subset_results);
                            }
                            Err(e) => panic!("{:?}", e),
                        };
                    }
                    save_to_fasta(new_fasta_data, tpf_data, output, n_length.to_owned())
                }
                Err(e) => panic!("Something is wrong with the file! | {}", e),
            }
        })
    }
}
