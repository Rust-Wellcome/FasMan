use crate::generics::sanitise_header;
use clap::ArgMatches;
use compare::{natural, Compare};
use noodles::fasta::{self, Record};
use std::cmp::Ordering;
use std::fs::OpenOptions;
use std::{
    fs::{create_dir_all, File},
    io::BufReader,
    path::Path,
};

#[allow(clippy::needless_return)]
fn fix_head(records: Record, sanitise: bool) -> Record {
    if sanitise {
        let header = sanitise_header(records.definition());
        let definition = fasta::record::Definition::new(header, None);
        let seq = records.sequence().to_owned();
        return fasta::Record::new(definition, seq);
    } else {
        return records.to_owned();
    };
}

fn write_fasta(outdir: &String, fasta_record: &Vec<Record>) {
    println!("{}", outdir);

    let _data_file = File::create(outdir);
    let file = OpenOptions::new()
        .append(true)
        .open(outdir)
        .expect("creation failed");

    let mut writer = fasta::Writer::new(file);
    for i in fasta_record {
        writer.write_record(i).unwrap();
    }
}

pub fn split_file_by_count(fasta_file: &String, output_directory: &String, data_type: &String, sanitise: &bool, fasta_count: &u16) {
    let path_obj = Path::new(fasta_file);
    let grab_name = path_obj.file_name().unwrap();
    let actual_list: Vec<&str> = grab_name.to_str().unwrap().split('.').collect();
    let actual_name = actual_list[0];

    let new_outpath = format!("{}/{}/{}/", output_directory, actual_name, data_type);
    create_dir_all(new_outpath.clone()).unwrap();
    println!(
        "Fasta file for processing: {:?}\nNumber of records per file: {:?}",
        fasta_file, fasta_count
    );

    let mut counter: u16 = 0;
    let mut file_counter: u16 = 1;

    let file_name: Vec<&str> = actual_name.split('.').collect();

    let mut reader = File::open(fasta_file)
        .map(BufReader::new)
        .map(fasta::Reader::new)
        .unwrap();

    let mut record_list: Vec<Record> = Vec::new();
    for result in reader.records() {
        let record = result.unwrap();
        counter += 1;

        let final_rec = fix_head(record, *sanitise);
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

