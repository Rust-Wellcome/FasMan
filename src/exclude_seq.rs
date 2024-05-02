pub mod exclude_seq_mod {
    use clap::ArgMatches;
    use noodles::fasta;
    use std::error::Error;
    use std::{fs, io::BufRead, str};

    fn open_fasta<'a>(
        exclusions: Vec<&str>,
        fasta: &'a str,
        out_file: &str,
    ) -> std::result::Result<(&'a str), Box<dyn Error>> {
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
            Err(_) => return Err("Error: Fasta is not valid check file!".into()),
        }
    }

    pub fn filter_fasta(arguments: std::option::Option<&ArgMatches>) {
        let fasta = arguments.unwrap().get_one::<String>("fasta").unwrap();
        let exclude = arguments.unwrap().get_one::<String>("filter_list").unwrap();
        let outfile = arguments.unwrap().get_one::<String>("output").unwrap();
        let list_to_exclude = exclude.split(",").collect::<Vec<&str>>();
        let _x = open_fasta(list_to_exclude, fasta, outfile);
    }
}
