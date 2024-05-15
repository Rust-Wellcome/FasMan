pub mod split_by_count_mod {
    use clap::ArgMatches;
    use compare::{natural, Compare};
    use noodles::fasta::record::Definition;
    use noodles::fasta::{self, Record};
    use std::cmp::Ordering::{self, Equal};
    use std::fs::OpenOptions;
    use std::{
        fs::{create_dir_all, File},
        io::{stdout, BufRead, BufReader, Write},
        path::Path,
    };

    fn sanitise_headers(head: &Definition) -> String {
        return head.to_string();
    }

    fn fix_head(records: Record, sanitise: bool) -> Record {
        let clean_headers = true;
        if clean_headers {
            let header = sanitise_headers(records.definition());

            let definition = fasta::record::Definition::new(header, None);
            let seq = records.sequence().to_owned();
            return fasta::Record::new(definition, seq);
        } else {
            return records.to_owned();
        };
    }

    fn write_fasta(outdir: &String, fasta_record: &Vec<Record>) {
        println!("{}", outdir);

        let _data_file = File::create(&outdir);
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(outdir)
            .expect("creation failed");

        let mut writer = fasta::Writer::new(file);
        for i in fasta_record {
            writer.write_record(&i).unwrap();
        }
    }

    pub fn split_file_by_count(arguments: std::option::Option<&ArgMatches>) {
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
        create_dir_all(new_outpath.clone()).unwrap();
        let fasta_count = arguments.unwrap().get_one::<u16>("count").unwrap();
        println!("Fasta file for processing: {:?}", fasta_file);
        println!("{:?}", &fasta_count);
        println!(
            "Number of sequence-header pairs per file: {:?}",
            fasta_count
        );

        let mut counter: u16 = 0;
        let mut file_counter = 1;
        let clean_headers = true;

        let file_name: Vec<&str> = actual_name.split(".").collect();

        let mut reader = File::open(fasta_file)
            .map(BufReader::new)
            .map(fasta::Reader::new)
            .unwrap();

        let mut record_list: Vec<Record> = Vec::new();
        for result in reader.records() {
            let record = result.unwrap();
            counter += 1;

            let final_rec = fix_head(record, clean_headers);
            record_list.push(final_rec);

            let cmp = natural();
            let compared = cmp.compare(&counter, fasta_count);
            if compared == Ordering::Equal {
                let full_outpath = format!(
                    "{}{}_f{}_c{}-a{}.fa",
                    new_outpath,
                    file_name[0],
                    file_counter,
                    &fasta_count,
                    &record_list.len()
                );

                write_fasta(&full_outpath, &record_list);
                file_counter += 1;
                counter = 0;
                record_list = Vec::new();
            }
        }
        let full_outpath = format!(
            "{}{}_f{}_c{}-a{}.fa",
            new_outpath,
            file_name[0],
            file_counter,
            &fasta_count,
            &record_list.len()
        );
        write_fasta(&full_outpath, &record_list);
    }
}
