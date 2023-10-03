pub mod map_headers {
    use std::io::BufRead;

    use noodles::{fasta, csi::index::header};
    use noodles::fasta::{record::{Definition, Sequence}};
    use colored::Colorize;

    pub fn validate_fasta(path: &str) {
        let reader = fasta::reader::Builder.build_from_path(path);

        let mut binding = reader.expect("NO VALID HEADER / SEQUENCE PAIRS");
        let result = binding.records();

        let names: Vec<_> = result.flatten().map(|res| res.name()).collect();
        
        
        //let names: Result<Vec<_>, _> = result.map(|res| res.map(|r| r.name())).collect();



        //let mut header_vec: Vec<&str> = Vec::new();


        //let names: Vec<_> = records.map(|res| res.unwrap().name()).collect();
        //let head_set: Vec<_> = result.map(|res| res.map(|x| x.name())).collect();

        //let counter = result.count();
        //println!("{} {} {}", ">- REFERENCE H/S PAIRS:".green(), counter, "H/S PAIRS".green());
    
        //let names: Result<Vec<_>, _> = result.map(|res| res.map(|x| x.name())).collect();
    
        //let names: Vec<_> = result.map(|res| res.map(|r| r.name())).collect();
        println!("{:?}", names)
    }


    pub fn create_mapping() {

    }

    pub fn save_mapping() {

    }

    pub fn create_mapped_fasta() {

    }

    pub fn map_fasta_head(file: &str, _sep: &str) -> Result<(), std::io::Error> {
        println!("Mapping headers for file: {}", file);
    
        validate_fasta(file);

        Ok(())
    }
    
}