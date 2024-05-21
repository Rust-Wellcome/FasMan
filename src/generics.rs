use noodles::fasta;
use noodles::fasta::record::Definition;
use std::error::Error;
use std::{collections::HashMap, fmt, io::BufRead, result, str};

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct EmptyVec;
impl Error for EmptyVec {}

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Can't Display Empty Vec")
    }
}

pub fn validate_fasta(
    path: &str,
) -> result::Result<HashMap<std::string::String, usize>, Box<dyn Error>> {
    // Simply validate the fasta is valid by reading though and ensure there are
    // valid record formats through out the file
    let reader: Result<fasta::Reader<Box<dyn BufRead>>, std::io::Error> =
        fasta::reader::Builder.build_from_path(path);
    let mut fasta_map = HashMap::new();

    match &reader {
        Ok(_fasta) => {
            let mut binding: fasta::Reader<Box<dyn BufRead>> =
                reader.expect("NO VALID HEADER / SEQUENCE PAIRS");
            for result in binding.records() {
                let record = result?;
                fasta_map.insert(record.name().to_owned(), record.sequence().len());
            }
            Ok(fasta_map)
        }
        Err(_) => Err("Error: Fasta is not valid check file!".into()),
    }
}

#[allow(clippy::iter_kv_map)]
pub fn only_keys<K, V>(map: HashMap<K, V>) -> impl Iterator<Item = K> {
    // Take a HashMap and return a Key only Vec
    map.into_iter().map(|(k, _v)| k)
}

fn get_gene_symbol(header: String) -> Result<String, Box<dyn std::error::Error>> {
    let header_list: Vec<&str> = header.split(" ").collect();
    let record_header = header_list[0];
    Ok(record_header[1..].to_owned())
    // let re = Regex::new(r"gene=([A-Z]\w+)").unwrap();

    // let first_run = re.captures(&header).ok_or("None")?;

    // if first_run[0] == "None".to_owned() {
    //     let re = Regex::new(r"symbol:(\S+)").unwrap();
    //     let second_run = re.captures(&header).ok_or("None")?;
    //     if second_run[0] == "None".to_owned() {
    //         let re = Regex::new(r"(\(\S+\)) gene").unwrap();
    //         let third_run = re.captures(&header).ok_or("None")?;
    //         if third_run[0] == "None".to_owned() {
    //             Ok("NOCAPTUREDRESULT".to_string())
    //         } else {
    //             Ok(third_run[0].to_string())
    //         }
    //     } else {
    //         Ok(second_run[0].to_string())
    //     }
    // } else {
    //     Ok(first_run[0].to_string())
    // }
}

pub fn sanitise_header(old_header: &Definition) -> String {
    let x = get_gene_symbol(old_header.to_string());

    // Yeah i dont know either...
    match x {
        Ok(c) => c,
        Err(e) => {
            format!("Regex isnt good enough to capture header id: {}", e)
        }
    }
}
