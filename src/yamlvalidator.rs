pub mod yamlvalidator {
    use serde::{Serialize, Deserialize};
    use colored::Colorize;
    use std::fs;
    use std::path::PathBuf;
    use std::io::{ErrorKind};

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

    pub fn validatepaths(path: &str, field_id: &str) {
        match fs::metadata(path) {
            Ok(_) => println!("{}   \t{}\t{}", &field_id.green(), "| PATH EXISTS: ".green(), path.green()),
            Err(_) => println!("{}   \t{}\t{}", &field_id.red().bold(), "| CHECK YAML!:".red().bold(), path),
        }
    }
    
    //
    // FUNCTION: Check if pacbio has fasta.gz files, cram has cram and crai and synteny has fasta
    //           could make this much easier and consise by passing in a list of file types to check
    //           validatedata(path, [fa, fna, fasta])
    //
    pub fn validatedata(path: &str, dtype: &str, _sep: &str) {
        match fs::read_dir(&path) {
            Err(e) if e.kind() == ErrorKind::NotFound => {}
            Err(e) => panic!("{} {e}", "DIRECTORY PATH DOESN'T EXIST: ".red().bold()),
            Ok(data_files) => {
                if dtype == "pacbio" {
                    let files: Vec<PathBuf> = data_files.filter_map(|f| f.ok())
                        .filter(|d| match d.path().extension() {
                            None => false,
                            Some(ex) => ex == "fasta.gz"
                        })
                        .map(|f| f.path())
                        .collect();
    
                    if files.len() == 0 {
                        println!("{}", "NO PACBIO DATA FILES".red())
                    } else {
                        println!("{} {:?}", "YOUR FILES ARE:".green(), &files);
                    }
    
                } else if dtype == "hic" {
                    let files: Vec<PathBuf> = data_files.filter_map(|f| f.ok())
                        .filter(|d| match d.path().extension() {
                            None => false,
                            Some(ex) => ex == "cram" || ex == "crai"
                        })
                        .map(|f| f.path())
                        .collect();
    
                    if files.len() == 0 {
                        println!("{}", "NO HIC DATA FILES".red())
                    } else {
                        println!("{} {:?}", "YOUR FILES ARE:".green(), &files);
                    }
    
                } else if dtype == "synteny" {
                    let files: Vec<PathBuf> = data_files.filter_map(|f| f.ok())
                        .filter(|d| match d.path().extension() {
                            None => false,
                            Some(ex) => ex == "fa" || ex == "fasta" || ex == "fna"
                        })
                        .map(|f| f.path())
                        .collect();
    
                    if files.len() == 0 {
                        println!("{}", "NO SYNTENIC GENOMES".red())
                    } else {
                        println!("{} {:?}", "YOUR GENOMES ARE:".green(), &files);
                    }
                }
            }
        };
    
    }
    
    
    pub fn validateyaml(file: &str, _verbose: &bool, sep: &str) -> Result<(), std::io::Error> {
        println!{"Validating Yaml: {}", file.purple()};
    
        let input = fs::File::open(file).expect("Unable to read from file");
        let contents: TreeValYaml = serde_yaml::from_reader(input).expect("Unable to read from file");
    
        println!("RUNNING VALIDATE-YAML FOR SAMPLE: {}", contents.assembly.sample_id.purple());
    
        validatepaths(&contents.reference_file, "REFERENCE");
        validatepaths(&contents.alignment.data_dir, "GENESET");
        validatepaths(&contents.synteny.synteny_genome_path, "SYNTENY");
        validatepaths(&contents.busco.lineages_path, "BUSCO");
    
        validatepaths(&contents.assem_reads.pacbio, "PACBIO");
        validatedata(&contents.assem_reads.pacbio, "pacbio", &sep);
    
        validatepaths(&contents.assem_reads.hic, "HIC");
        validatedata(&contents.assem_reads.hic, "hic", &sep);
    
        println!("{}", "CHECKING GENESET DIRECTORY RESOLVES".blue());
        let genesets = contents.alignment.geneset.split(",");
        for set in genesets {
            let gene_alignment_path = contents.alignment.data_dir.clone() + &contents.assembly.classT + &sep + "csv_data" + &sep + &set + "-data.csv";
            validatepaths(&gene_alignment_path, "GENESET-CSV");
        };
    
        println!("{}", "CHECKING SYNTENY DIRECTORY RESOLVES".blue());
        let synteny_full = contents.synteny.synteny_genome_path.clone() + &contents.assembly.classT + &sep;
        validatepaths(&synteny_full, "SYNTENY-FASTA");
        validatedata(&synteny_full, "synteny", &sep);
    
    
        println!("{}", "CHECKING BUSCO DIRECTORY RESOLVES".blue());
        let busco_path = contents.busco.lineages_path.clone()  + &sep + "lineages" + &sep + &contents.busco.lineage;
        validatepaths(&busco_path, "BUSCO-DB");
        // NOW CHECK FOR FILES IN DIRECTORY?
        
        println!("{}\n{}\n{}\n{}\n{}", 
            "VALIDATION COMPLETE".purple(),
            "GENERAL INFORMATION:".purple().bold(),
            "Check the log to see what failed",
            "FULL : ONLY synteny fails are permitted".purple(),
            "RAPID: geneset, busco and synteny fails are permitted".purple()
        );
    
        Ok(())
    }
}