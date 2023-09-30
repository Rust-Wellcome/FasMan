use std::fs;
use std::fs::File;
use std::env;
use std::io::{BufReader, BufRead, Error, ErrorKind};
use std::path::PathBuf;
use clap::{Command, command, Arg};
use serde::{Serialize, Deserialize};

//use regex::Regex;

#[derive(Debug, Serialize, Deserialize)]
struct TreeValYaml {
    assembly: Assembly,
    reference_file: String,
    assem_reads: AssemReads,
    alignment: Alignment,
    self_comp: SelfComp,
    intron: Intron,
    telomere: Telomere,
    synteny: Synteny,
    busco: Busco,
}
#[derive(Debug, Serialize, Deserialize)]
struct Assembly {
    level: String,
    sample_id: String,
    latin_name: String,
    classT: String,
    asmVersion: u16,
    gevalType: String
}

#[derive(Debug, Serialize, Deserialize)]
struct AssemReads {
    pacbio: String,
    hic: String,
    supplementary: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Alignment {
    data_dir: String,
    common_name: String,
    geneset: String
}

#[derive(Debug, Serialize, Deserialize)]
struct SelfComp {
    motif_len: u16,
    mummer_chunk: u16
}

#[derive(Debug, Serialize, Deserialize)]
struct Intron {
    size: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Telomere {
    teloseq: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Synteny {
    synteny_genome_path: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Busco {
    lineages_path: String,
    lineage: String
}


fn validatepaths(path: &str, sep: &str) {
    match fs::metadata(path) {
        Ok(_) => println!("PATH EXISTS: {}", path),
        Err(_) => println!("PATH DOESN'T EXIST | CHECK YAML!: {}", path),
    }
}

// Check if pacbio has fasta.gz files and cram has cram and crai
fn validatedata(path: &str, dtype: &str, sep: &str) {
        match fs::read_dir(&path) {
            Err(e) if e.kind() == ErrorKind::NotFound => {}
            Err(e) => panic!("DIRECTORY PATH DOESN'T EXIST: {e}"),
            Ok(data_files) => {
                if dtype == "pacbio" {
                    let files: Vec<PathBuf> = data_files.filter_map(|f| f.ok())
                        .filter(|d| match d.path().extension() {
                            None => false,
                            Some(ex) => ex == "fasta.gz"
                        })
                        .map(|f| f.path())
                        .collect();
                    println!("{:?}", &files);
            
                } else if dtype == "hic" {
                    let files: Vec<PathBuf> = data_files.filter_map(|f| f.ok())
                        .filter(|d| match d.path().extension() {
                            None => false,
                            Some(ex) => ex == "cram" || ex == "crai"
                        })
                        .map(|f| f.path())
                        .collect();
                    println!("{:?}", &files);
                    // IF COLLECT SIZE == 0 "NO FILES THOUGH"
            }
        }
    };

}


fn validateyaml(file: &str, _verbose: &bool, sep: &str) -> Result<(), std::io::Error> {
    println!{"Validating Yaml: {}", file};
    let input = fs::File::open(file).expect("Unable to read from file");
    let contents: TreeValYaml = serde_yaml::from_reader(input).expect("Unable to read from file");

    println!("RUNNING VALIDATE-YAML FOR SAMPLE: {}", contents.assembly.sample_id);
    println!("RUNNING VALIDATE-YAML ON FILE: {}", file);

    validatepaths(&contents.reference_file, &sep);
    validatepaths(&contents.alignment.data_dir, &sep);
    validatepaths(&contents.synteny.synteny_genome_path, &sep);
    validatepaths(&contents.busco.lineages_path, &sep);
    validatepaths(&contents.assem_reads.pacbio, &sep);
    validatedata(&contents.assem_reads.pacbio, "pacbio", &sep);
    validatepaths(&contents.assem_reads.hic, &sep);
    validatedata(&contents.assem_reads.hic, "hic", &sep);

    println!("CHECKING GENESET DIRECTORY RESOLVES");
    let genesets = contents.alignment.geneset.split(",");
    for set in genesets {
        let gene_alignment_path = contents.alignment.data_dir.clone() + &contents.assembly.classT + &sep + "csv_data" + &sep + &set + "-data.csv";
        validatepaths(&gene_alignment_path, &sep);
    };

    println!("CHECKING SYNTENY DIRECTORY RESOLVES");
    let synteny_full = contents.synteny.synteny_genome_path.clone() + &contents.assembly.classT + &sep;
    let synteny_path = fs::read_dir(&synteny_full)?;
    for path in synteny_path {
        let path = path?.path();
        if path.is_file() {
            let data = std::fs::read_to_string(path)?;
            println!("SYNTENIC GENOME FOUND: {}", data)
        } else {
            println!{"NO SYNTENIC GENOMES IN DIRECTORY: {}", std::fs::read_to_string(path)?}
        }
    };

    println!("CHECKING BUSCO DIRECTORY RESOLVES");
    let busco_path = contents.busco.lineages_path.clone()  + &sep + "lineages" + &sep + &contents.busco.lineage;
    validatepaths(&busco_path, &sep);

    Ok(())
}

fn splitbycount(file: &str, chunk: &u16, sep: &str) -> Result<(), std::io::Error> {
    println!("Splitting file: {}", file);
    println!("Splitting bycount: {}", chunk);
    //if bycount {
    //    println!("Count to split by: {}", bycount.unwrap().get_one::<String>("count").unwrap());
    // }
    // returns bool bycount.unwrap().contains_id("count");
    
    // ---
    let  chunk_val = chunk.clone();
    let mut counter = 0;
    let mut global_counter = 0;

    //let mut output = File::create(path)?;
    //write!(output, "Rust\n💖\nFun")?;

    let input = File::open(file)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        if counter != chunk_val {
            if line?.starts_with('>') {
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
    Ok(())
}

fn splitbysize(file: &str, sep: &str) -> Result<(), std::io::Error> {
    println!("Splitting file: {}", file);

    Ok(())
}

fn mapheaders(file: &str, sep: &str) -> Result<(), std::io::Error> {
    println!("Mapping headers for file: {}", file);

    Ok(())
}

fn main() -> Result<(), Error> {
    let match_result = command!()
    .about("A program for fasta manipulation and yaml validation ~ Used in TreeVal project")
    .subcommand(
        Command::new("validateyaml")
            .about("Subcommand for validating the users TreeVal yaml file")
            .arg(
                Arg::new("yaml")
                    .required(true)
                    .help("Path to the TreeVal yaml file generated by the user")
            )
            .arg(
                Arg::new("verbose")
                    .short('v')
                    .value_parser(clap::value_parser!(bool))
                    .default_value("false")
                    .required(false)
                    .help("Print explainers as to why validation fails, if it does fail")
            )
    )
    .subcommand(
        Command::new("splitbycount")
            .about("Subcommand for splitting fasta files by number of sequence-header pairs, e.g., 100 pairs per file")
            .arg(
                Arg::new("fasta-file")
                    .short('f')
                    .aliases(["fasta"])
                    .required(true)
                    .help("A path to a valid fasta file.")
            )
            .arg(
                Arg::new("count")
                    .short('c')
                    .long("file-count")
                    .aliases(["count"])
                    .value_parser(clap::value_parser!(u16))
                    .help("How many sequences per file")
            )
    )
    .subcommand(
        Command::new("splitbysize")
            .about("Subcommand for splitting fasta files by user given size (in MegaBytes) into n (fasta_size / user_given_size) files")
            .arg(
                Arg::new("fasta-file")
                    .short('f')
                    .aliases(["fasta"])
                    .required(true)
                    .help("A path to a valid fasta file.")
            )
            .arg(
                Arg::new("mem-size")
                    .short('s')
                    .long("mem-size")
                    .required(true)
                    .value_parser(clap::value_parser!(u16))
                    .help("Size in MB that a fasta file is to be chunked into")
            )
    )
    .subcommand(
        Command::new("mapheaders")
            .about("Subcommand for stripping out headers and replacing with a standardised automatic or user-given string, this also returns a dict of old:new headers")
            .arg(
                Arg::new("fasta-file")
                    .short('f')
                    .aliases(["fasta"])
                    .required(true)
                    .help("A path to a valid fasta file.")
            )
            .arg(
                Arg::new("replace-with")
                    .short('r')
                    .aliases(["replacement"])
                    .required(false)
                    .default_value("FMMH")
                    .help("The new header format, appended with a numerical value. Without being set the new header will default to 'FM_{numberical}'")
            )
    )
    .get_matches();

    //println!("{}", &match_result.get_one::<String>("count").unwrap_or(&"DEFAULT".to_string()));

    let args: Vec<String> = env::args().collect();

    println!("OPERATING SYSTEM: {}", env::consts::OS); // Prints the current OS.
    let mut path_sep = "/";
    match env::consts::OS {
        "windows" => path_sep = "\\",
        _ => println!("No path changes needed")
    }

    // Should replace this with a "match &*args[1]" Would require wrapping each subcommands entirely in a seperate function
    if &args[1].to_string() == "splitbycount" {
        let arguments = match_result.subcommand_matches("splitbycount");
        let fasta_file = arguments.unwrap().get_one::<String>("fasta-file").unwrap();
        let fasta_count = arguments.unwrap().get_one::<u16>("count").unwrap();
        println!("Fasta file for processing: {:?}", fasta_file);
        println!("{:?}", &fasta_count);
        println!("Number of sequence-header pairs per file: {:?}", fasta_count);
        splitbycount(fasta_file, &fasta_count, path_sep);
    } else if &args[1].to_string() == "splitbysize" {
        let arguments = match_result.subcommand_matches("splitbysize");
        let fasta_file = arguments.unwrap().get_one::<String>("fasta-file").unwrap();
        println!("Fasta file for processing: {:?}", arguments.unwrap().get_one::<String>("fasta-file").unwrap());
        println!("Size to chunk fasta into: {:?}", arguments.unwrap().get_one::<u16>("mem-size").unwrap());
        splitbysize(fasta_file, path_sep);
    } else if &args[1].to_string() == "mapheaders" {
        let arguments = match_result.subcommand_matches("mapheaders");
        let fasta_file = arguments.unwrap().get_one::<String>("fasta-file").unwrap();
        println!("Fasta file for processing: {:?}", arguments.unwrap().get_one::<String>("fasta-file").unwrap());
        println!("Replace headers with string: {:?}", arguments.unwrap().get_one::<String>("replace-with").unwrap());
        mapheaders(fasta_file, path_sep);
    } else if &args[1].to_string() == "validateyaml" {
        let arguments = match_result.subcommand_matches("validateyaml");
        let yaml_file = arguments.unwrap().get_one::<String>("yaml").unwrap();
        let verbose_flag = arguments.unwrap().get_one::<bool>("verbose").unwrap();
        validateyaml(yaml_file, verbose_flag, path_sep);
    };

    Ok(())
}