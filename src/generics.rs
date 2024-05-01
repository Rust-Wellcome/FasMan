use noodles::fasta;
use std::error::Error;
use std::{
    fmt,
    io::BufRead,
    str,
    collections::HashMap,
    result,
};

#[derive(Debug, Clone)]
struct EmptyVec;
impl Error for EmptyVec {}

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Can't Display Empty Vec")
    }
}

pub fn validate_fasta(path: &str) -> result::Result<HashMap<std::string::String, usize>, Box<dyn Error>> {
    // Simply validate the fasta is valid by reading though and ensure there are
    // valid record formats through out the file
    let reader: Result<fasta::Reader<Box<dyn BufRead>>, std::io::Error> = fasta::reader::Builder.build_from_path(path);
    let mut fasta_map = HashMap::new();

    match &reader {
        Ok(_fasta) => {
            let mut binding: fasta::Reader<Box<dyn BufRead>> =
                reader.expect("NO VALID HEADER / SEQUENCE PAIRS");
            for result in binding.records() {
                let record = result?;
                fasta_map.insert(record.name().to_owned(), record.sequence().len());
            }
            return Ok(fasta_map)
        }
        Err(_) => return Err("Error: Fasta is not valid check file!".into()),
    }
}

pub fn only_keys<K, V>(map: HashMap<K, V>) -> impl Iterator<Item = K> {
    // Take a HashMap and return a Key only Vec
    map.into_iter().map(|(k, _v)| k)
}