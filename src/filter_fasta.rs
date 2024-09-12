pub mod filter_fasta_mod {
    use clap::ArgMatches;
    use noodles::fasta;
    use std::error::Error;
    use std::fs::File;
    use std::{
        fs,
        io::{BufRead, BufReader},
        path::Path,
    };

    fn open_fasta<'a>(
        exclusions: Vec<&str>,
        fasta: &'a str,
        out_file: &str,
    ) -> std::result::Result<&'a str, Box<dyn Error>> {
        // Open and read fasta
        let reader: Result<fasta::Reader<Box<dyn BufRead>>, std::io::Error> =
            fasta::reader::Builder.build_from_path(fasta);

        // Create new file
        let file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(out_file)?;
        let mut writer = fasta::Writer::new(file);

        match reader {
            Ok(fasta) => {
                // on Ok reading append record to new fasta if
                // not in user given list of headers
                let mut binding = fasta;
                for result in binding.records() {
                    let record = result?;
                    let record_name = std::str::from_utf8(record.name())?;
                    if !exclusions.contains(&record_name) {
                        writer.write_record(&record)?;
                    } else {
                        println!("Found record to exclude: {:?}", &record_name);
                    }
                }
                Ok("Removed Exclusionary List")
            }
            Err(_) => Err("Error: Fasta is not valid check file!".into()),
        }
    }

    fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
        let file = File::open(filename).expect("no such file");
        let buf = BufReader::new(file);
        buf.lines()
            .map(|l| l.expect("Could not parse line"))
            .collect()
    }

    pub fn filter_fasta(arguments: std::option::Option<&ArgMatches>) {
        let fasta = arguments.unwrap().get_one::<String>("fasta").unwrap();
        let outfile = arguments.unwrap().get_one::<String>("output").unwrap();
        let exclude_list = arguments.unwrap().get_one::<String>("filter_list").unwrap();
        let exclude_file = arguments.unwrap().get_one::<String>("filter_file").unwrap();

        if exclude_file != "None" {
            let exclusion_list = lines_from_file(exclude_file);
            let exclusion_list_parsed = exclusion_list.iter().map(|p| p.as_str()).collect();
            let _x = open_fasta(exclusion_list_parsed, fasta, outfile);
        }

        if exclude_list != "None" {
            let list_to_exclude = exclude_list.split(',').collect::<Vec<&str>>();
            let _y = open_fasta(list_to_exclude, fasta, outfile);
        }
    }
}
