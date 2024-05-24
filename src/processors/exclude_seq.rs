use clap::ArgMatches;
use noodles::fasta;
use std::error::Error;
use std::{fs, io::BufRead, str};

fn open_fasta<'a>(
    exclusions: Vec<&str>,
    fasta: &'a str,
    out_file: &str,
) -> std::result::Result<&'a str, Box<dyn Error>> {
    let reader: Result<fasta::Reader<Box<dyn BufRead>>, std::io::Error> =
        fasta::reader::Builder.build_from_path(fasta);
    let file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(out_file)?;
    let mut writer = fasta::Writer::new(file);

    match reader {
        Ok(fasta) => {
            let mut binding = fasta;
            for result in binding.records() {
                let record = result?;
                if !exclusions.contains(&record.name()) {
                    writer.write_record(&record)?;
                } else {
                    println!("Found record to exclude: {:?}", &record.name());
                }
            }
            Ok("Removed Exclusionary List")
        }
        Err(_) => Err("Error: Fasta is not valid check file!".into()),
    }
}

pub fn filter_fasta(fasta: &String, outfile: &String, exclude: &String) {
    let list_to_exclude = exclude.split(',').collect::<Vec<&str>>();
    let _x = open_fasta(list_to_exclude, fasta, outfile);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

