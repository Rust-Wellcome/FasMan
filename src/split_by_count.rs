pub mod split_by_count_mod {
    use crate::generics::{sanitise_header, write_fasta};
    use clap::ArgMatches;
    use compare::{natural, Compare};
    use noodles::fasta::{self, Record};
    use std::cmp::Ordering;
    use std::{fs::File, io::BufReader, path::Path};

    #[allow(clippy::needless_return)]
    fn fix_head(records: Record, sanitise: bool, data_type: &str) -> Record {
        // Taker a Record and sanitise the header
        // recombine into a new Record
        if sanitise {
            let header = sanitise_header(records.definition(), data_type);
            let definition = fasta::record::Definition::new(header, None);
            let seq = records.sequence().to_owned();
            return fasta::Record::new(definition, seq);
        } else {
            return records.to_owned();
        };
    }

    pub fn split_file_by_count(arguments: std::option::Option<&ArgMatches>) {
        let sanitise: &bool = arguments.unwrap().get_one::<bool>("sanitise").unwrap();
        let fasta_file = arguments.unwrap().get_one::<String>("fasta-file").unwrap();
        let path_obj = Path::new(fasta_file);
        let grab_name = path_obj.file_name().unwrap();
        let actual_list: Vec<&str> = grab_name.to_str().unwrap().split('.').collect();
        let actual_name = actual_list[0];

        let data_type = arguments.unwrap().get_one::<String>("data_type").unwrap();

        let outpath = arguments
            .unwrap()
            .get_one::<String>("output-directory")
            .unwrap();

        let new_outpath = format!("{}/{}/{}/", outpath, actual_name, data_type);
        let fasta_count = arguments.unwrap().get_one::<u16>("count").unwrap();
        println!(
            "Fasta file for processing: {:?}\nNumber of records per file: {:?}",
            fasta_file, fasta_count
        );

        // Header counter
        let mut counter: u16 = 0;
        let mut file_counter: u16 = 1;

        // Remove the file suffix from the file name
        let file_name: Vec<&str> = actual_name.split('.').collect();

        // Open the fasta file
        let mut reader = File::open(fasta_file)
            .map(BufReader::new)
            .map(fasta::Reader::new)
            .unwrap();

        // Create a Record List
        let mut record_list: Vec<Record> = Vec::new();
        for result in reader.records() {
            let record = result.unwrap();
            counter += 1;

            let final_rec = fix_head(record, *sanitise, data_type);
            record_list.push(final_rec);

            let cmp = natural();
            let compared = cmp.compare(&counter, fasta_count);
            if compared == Ordering::Equal {
                let file_name = format!(
                    "{}_f{}_c{}-a{}.fa",
                    file_name[0],
                    file_counter,
                    &fasta_count,
                    &record_list.len()
                );

                let _ = write_fasta(&new_outpath, file_name, record_list);
                file_counter += 1;
                counter = 0;
                record_list = Vec::new();
            }
        }

        let file_name = format!(
            "{}_f{}_c{}-a{}.fa",
            file_name[0],
            file_counter,
            &fasta_count,
            &record_list.len()
        );
        let _ = write_fasta(&new_outpath, file_name, record_list);
    }
}
