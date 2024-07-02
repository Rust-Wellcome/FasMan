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
    pub struct Tpf {
        pub ori_scaffold: String,
        pub start_coord: usize,
        pub end_coord: usize,
        pub new_scaffold: String,
        pub orientation: String,
    }

    impl std::fmt::Display for Tpf {
        // This is how we want to print a Tpf object
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            write!(
                fmt,
                "\t{} -- {} -- {}",
                self.ori_scaffold, self.start_coord, self.end_coord
            )
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct NewFasta {
        pub tpf: Tpf,
        pub sequence: String,
    }

    #[derive(Debug)]
    struct MyRecord {
        name: String,
        sequence: Vec<String>,
    }

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

    pub fn subset_vec_tpf<'a>(
        tpf: &'a Vec<Tpf>,
        fasta: (&std::string::String, &usize),
    ) -> Vec<&'a Tpf> {
        //
        // Subset the Vec<TPF> based on a search through the fasta
        //
        let mut subset_tpf: Vec<&Tpf> = Vec::new();
        for i in tpf {
            if i.ori_scaffold == *fasta.0 {
                subset_tpf.push(i)
            }
        }
        subset_tpf
    }

    // The TPF will contain data in both PLUS (normal) and
    // MINUS (inverted), if MINUS then we need to invert again
    // and get the complement sequence
    // We then return the sequence of the record.
    pub fn check_orientation(
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

    pub fn parse_seq(
        sequence: std::option::Option<noodles::fasta::record::Sequence>,
        tpf: Vec<&Tpf>,
    ) -> Vec<NewFasta> {
        //
        // Take the input sequence and scaffold name
        // Parse the input sequence based on the data contained in
        // the TPF. Which is already a subset based on scaff name.
        //
        // for instance this Vec may only contain SCAFFOLD_1 TPF records
        // if the sequence is from a SCAFFOLD_1 component
        // as we move through the list, we are cutting the sequence at the
        // recorded positions and outputting the new sequence.
        //
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

    pub fn get_uniques(tpf_list: &Vec<Tpf>) -> Vec<String> {
        // Get a Vec of the uniques names in the TPF Vec
        let mut uniques: Vec<String> = Vec::new();

        for i in tpf_list {
            if !uniques.contains(&i.new_scaffold) {
                uniques.push(i.new_scaffold.to_owned())
            }
        }
        uniques
    }

    fn save_to_fasta(
        fasta_data: Vec<NewFasta>,
        tpf_data: Vec<Tpf>,
        output: &String,
        n_length: usize,
    ) {
        //
        // TPF is in the input TPF order, this will continue to be the case until
        // such time that the script starts modifying the TPF in place which
        // we don't want to happen. Once this happens the order will no
        // longer be guaranteed.
        //
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

        // This is inefficient as we are scanning through the fasta_data, uniques
        // ( equal to number of scaffolds) number of times
        // If uniques is 10 long and fasta is 100, then this is 1000 scans through in total.
        for x in uniques {
            println!("NOW WRITING DATA FOR: {:?}", &x);
            // X = "SUPER_1"
            let stringy = format!(">{x}\n");
            file.write_all(stringy.as_bytes())
                .expect("Unable to write to file");

            // file2 will collect what went where
            // no sequence data
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

            // Should be it's own function really
            // This actually writes the new fasta file
            // Joining the data together with user (default = 200)
            // N's (gap)

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
    pub fn curate_fasta(arguments: std::option::Option<&ArgMatches>) {
        //
        // Generate a curated fasta file based on the input TPF file
        // which was generated by Pretext and the agp_to_tpf scripts.
        // This new fasta file contains a new scaffold naming as well
        // as pieced together sequences generated by the splitting of
        // data in Pretext.
        //
        let fasta_file: &String = arguments.unwrap().get_one::<String>("fasta").unwrap();
        let tpf_file: &String = arguments.unwrap().get_one::<String>("tpf").unwrap();
        let n_length: &usize = arguments.unwrap().get_one::<usize>("n_length").unwrap();
        let output: &String = arguments.unwrap().get_one::<String>("output").unwrap();
        println!("LET'S GET CURATING THAT FASTA!");

        // Stacker is supposed to increase the stack size
        // once memory runs out
        stacker::maybe_grow(32 * 1024, 1024 * 5120, || {
            match validate_fasta(fasta_file) {
                // validate returns Vec of headers - basically indexes it
                Ok(fasta_d) => {
                    let tpf_data = parse_tpf(&tpf_file);

                    //
                    // Start indexed reader of the input fasta
                    // if valid then use the data
                    //
                    let reader =
                        fasta::indexed_reader::Builder::default().build_from_path(fasta_file);
                    let fasta_repo = match reader {
                        Ok(data) => {
                            let adapter = IndexedReader::new(data);

                            // Now read the fasta and return is as a queryable object
                            let repository = fasta::Repository::new(adapter);
                            repository
                        }
                        Err(_) => todo!(), // Probably just panic!
                    };

                    //
                    // For unique scaffold in the fasta file iter through and
                    // parse sequence for each line in the tpf
                    // The tpf will contain multiple enteries for each scaffold, minimum of one entry.
                    //
                    let mut new_fasta_data: Vec<NewFasta> = Vec::new();
                    for i in fasta_d {
                        // for header in fasta_d
                        // subset the tpf on header and length
                        // cross referencing with fasta_d
                        let subset_tpf = subset_vec_tpf(&tpf_data, (&i.0, &i.1));

                        // Query the fasta for scaffold = header
                        let sequence = fasta_repo.get(&i.0).transpose();

                        // if exists then get the seqeuence, return a tpf object
                        // containing the trimmed sequence
                        match sequence {
                            Ok(data) => {
                                let subset_results = parse_seq(data, subset_tpf);
                                new_fasta_data.extend(subset_results);
                            }
                            Err(e) => panic!("{:?}", e),
                        };
                    }
                    // Write it all out to fasta
                    save_to_fasta(new_fasta_data, tpf_data, output, n_length.to_owned())
                }
                Err(e) => panic!("Something is wrong with the file! | {}", e),
            }
        })
    }
}
