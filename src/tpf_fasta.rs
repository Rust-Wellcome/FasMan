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
    struct Tpf {
        ori_scaffold: String,
        start_coord: usize,
        end_coord: usize,
        new_scaffold: String,
        orientation: String,
    }

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
    struct NewFasta {
        tpf: Tpf,
        sequence: String,
    }

    #[derive(Debug)]
    struct MyRecord {
        name: String,
        sequence: Vec<String>,
    }

    fn parse_tpf(path: &String) -> Vec<Tpf> {
        let mut all_tpf: Vec<Tpf> = Vec::new();
        for line in read_to_string(path).unwrap().lines() {
            if line.starts_with('?') {
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

    fn subset_vec_tpf<'a>(
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

    fn parse_seq(
        sequence: std::option::Option<noodles::fasta::record::Sequence>,
        tpf: Vec<&Tpf>,
    ) -> Vec<NewFasta> {
        let mut subset_tpf: Vec<NewFasta> = Vec::new();
        //
        // Take the input sequence and scaffold name
        // Parse the input sequence based on the data contained in
        // the TPF. Which is already a subset based on scaff name
        //

        let new_seq = sequence.unwrap(); // Option(Sequence ()) -> Sequence ()
        for &i in &tpf {
            let start = Position::try_from(i.start_coord).unwrap();
            let end = Position::try_from(i.end_coord).unwrap();
            //let region = Region::new(&i.new_scaffold, start.unwrap()..=end.unwrap());
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

    fn get_uniques(tpf_list: &Vec<Tpf>) -> Vec<String> {
        let mut uniques: Vec<String> = Vec::new();

        for i in tpf_list {
            if !uniques.contains(&i.new_scaffold) {
                uniques.push(i.new_scaffold.to_owned())
            }
        }
        uniques
    }

    fn save_to_fasta(fasta_data: Vec<NewFasta>, tpf_data: Vec<Tpf>, output: &String) {
        //
        // TPF is in the input TPF order, this will continue to be the case until
        // the script is modified and the Tpf struct gets modified in place for some reason
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

        // This is inefficient as we are scanning through the fasta_data, uniques number of times
        // If uniques is 10 long and fasta is 100, then this is 1000 scans through in total.
        let mut no_more: Vec<String> = Vec::new();
        for x in uniques {
            println!("NOW WRITING DATA FOR: {:?}", &x);
            // X = "SUPER_1"
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

            no_more.push(x.to_owned());
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
            let fixed2 = fixed.join("NNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNN");
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
            println!("NO LONG SCANNING FOR: {:?}", &no_more)
        }
    }

    #[allow(clippy::needless_borrow)]
    #[allow(clippy::let_and_return)]
    pub fn curate_fasta(arguments: std::option::Option<&ArgMatches>) {
        //
        // Generate a curated fasta file based on the input TPF file
        // which was generated by Pretext and the agp_to_tpf script.
        // This new fasta file contains a new scaffold naming as well
        // as pieced together sequences generated by the splitting of
        // data in Pretext.
        //
        let fasta_file: &String = arguments.unwrap().get_one::<String>("fasta").unwrap();
        let tpf_file: &String = arguments.unwrap().get_one::<String>("tpf").unwrap();
        let output: &String = arguments.unwrap().get_one::<String>("output").unwrap();
        println!("LET'S GET CURATING THAT FASTA!");
        stacker::maybe_grow(32 * 1024, 1024 * 5120, || {
            match validate_fasta(fasta_file) {
                Ok(fasta_d) => {
                    let tpf_data = parse_tpf(&tpf_file);
                    //let _validated = varify_validity(&tpf_data, &fasta_d);

                    //
                    // Start indexed reader of the input fasta
                    // if valid then use the data
                    //
                    let reader =
                        fasta::indexed_reader::Builder::default().build_from_path(fasta_file);
                    let fasta_repo = match reader {
                        Ok(data) => {
                            let adapter = IndexedReader::new(data);
                            let repository = fasta::Repository::new(adapter);
                            repository
                        }
                        Err(_) => todo!(),
                    };

                    //
                    // For unique scaffold in the fasta file iter through and
                    // parse sequence for each line in the tpf
                    // The tpf will contain multiple enteries for each scaffold, minimum of one entry.
                    //
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
                    save_to_fasta(new_fasta_data, tpf_data, output)
                }
                Err(e) => panic!("Something is wrong with the file! | {}", e),
            }
        })
    }
}
