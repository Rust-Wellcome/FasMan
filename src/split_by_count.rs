pub mod split_by_count_mod {
    use clap::ArgMatches;
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    pub fn split_file_by_count(arguments: std::option::Option<&ArgMatches>, _sep: &str) {
        let fasta_file = arguments.unwrap().get_one::<String>("fasta-file").unwrap();
        let fasta_count = arguments.unwrap().get_one::<u16>("count").unwrap();
        println!("Fasta file for processing: {:?}", fasta_file);
        println!("{:?}", &fasta_count);
        println!(
            "Number of sequence-header pairs per file: {:?}",
            fasta_count
        );

        let chunk_val = *fasta_count;
        let mut counter = 0;
        let mut global_counter = 0;

        let input = File::open(fasta_file).expect("CANT OPEN FASTA");
        let buffered = BufReader::new(input);

        for line in buffered.lines() {
            if counter != chunk_val {
                if line.expect("NO LINES IN FASTA").starts_with('>') {
                    println!("header");
                } else {
                    println!("Sequence");
                    counter += 1;
                    global_counter += 1;
                }
            } else {
                counter = 0;
                println!("CHUNK");
            }
        }
        println!("Total number of pairs: {:?}", global_counter);
    }
}
