pub mod yaml_validator_mod {
    use clap::ArgMatches;
    use colored::Colorize;
    use csv::Error;
    use csv::ReaderBuilder;
    use noodles::fasta;
    use serde::{Deserialize, Serialize};
    use std::fs::{self, File};
    use std::io::ErrorKind;
    use std::path::PathBuf;
    // Would be nice if there was a simple format_check
    // use noodles::cram as cram;

    #[derive(Debug, Serialize, Deserialize)]
    struct TreeValYaml {
        assembly: Assembly,
        reference_file: String,
        map_order: String,
        assem_reads: AssemReads,
        hic_data: HicReads,
        kmer_profile: KmerProfile,
        alignment: Alignment,
        self_comp: SelfComp,
        intron: Intron,
        telomere: Telomere,
        synteny: Synteny,
        busco: Busco,
    }

    impl TreeValYaml {
        #[allow(dead_code)]
        fn validate_fasta(&self) -> String {
            let reader = fasta::reader::Builder.build_from_path(&self.reference_file);

            let mut binding = reader.expect("NO VALID HEADER / SEQUENCE PAIRS");
            let result = binding.records();
            let counter = result.count();
            if counter >= 1 {
                format!(
                    "{} {} {}",
                    ">- VALID REFERENCE H/S PAIRS:".green(),
                    counter,
                    "H/S PAIRS".green()
                )
            } else {
                format!("{}", "NO HEADER/SEQ PAIRS".red())
            }
        }

        #[allow(dead_code)]
        fn validate_busco_path(&self) -> String {
            let full_busco_path = format!(
                "{}/lineage/{}",
                self.busco.lineages_path, self.busco.lineage
            );
            full_busco_path
        }

        #[allow(dead_code)]
        fn validate_data(&self) {
            // list_dir(self.hic_reads.dir)
            // if i in list == .cram {validate_cram} elif i == .fasta.gz { do i need to validate pacbio? }
        }

        #[allow(dead_code)]
        fn validate_cram() {}

        #[allow(dead_code)]
        fn validate_genesets(&self) {}

        #[allow(dead_code)]
        fn validate_synteny(&self) {}

        #[allow(dead_code)]
        fn validate_kmer_prof(&self) {}

        #[allow(dead_code)]
        fn validate_telomere(&self) {
            // make sure only AlphaNumeric
            // and longer than 3
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct KmerProfile {
        kmer_length: u16,
        dir: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct HicReads {
        hic_cram: String,
        hic_aligner: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Assembly {
        assem_level: String,
        sample_id: String,
        latin_name: String,
        defined_class: String,
        assem_version: u16,
        project_id: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct AssemReads {
        read_type: String,
        read_data: String,
        supplementary_data: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Alignment {
        data_dir: String,
        common_name: String,
        geneset_id: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct SelfComp {
        motif_len: u16,
        mummer_chunk: u16,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Intron {
        size: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Telomere {
        teloseq: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Synteny {
        synteny_path: String,
        synteny_genomes: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Busco {
        lineages_path: String,
        lineage: String,
    }

    //
    // CSV STRUCT
    //
    //#[derive(Deserialize)]
    //struct Record {
    //    org: String,
    //    type: String,
    //    data_file: String
    //}

    pub fn validate_paths(path: &str, field_id: &str) {
        match fs::metadata(path) {
            Ok(_) => {
                println!(
                    "{}{}   \t{}\t{}",
                    ">-".green(),
                    &field_id.green(),
                    "| PATH EXISTS: ".green(),
                    path.green()
                );
                match field_id {
                    "REFERENCE" => validate_fasta(path),
                    "GENESET-CSV" => {
                        _ = validate_csv(path);
                    }
                    "HIC" => {}
                    _ => println!("Error"),
                }
            }
            Err(_) => println!(
                "{}{}   \t{}\t{}",
                "<-".red().bold(),
                &field_id.red().bold(),
                "| CHECK YAML!:".red().bold(),
                path
            ),
        }
    }

    pub fn validate_fasta(path: &str) {
        let reader = fasta::reader::Builder.build_from_path(path);

        let mut binding = reader.expect("NO VALID HEADER / SEQUENCE PAIRS");
        let result = binding.records();
        let counter = result.count();
        println!(
            "{} {} {}",
            ">- REFERENCE H/S PAIRS:".green(),
            counter,
            "H/S PAIRS".green()
        )
    }

    pub fn validate_csv(path: &str) -> Result<(), Error> {
        let file = File::open(path)?;

        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b',')
            .from_reader(file);

        let record = reader.records().count();
        println!(
            "{} {} {}",
            ">-GENESET-RECORD-COUNT: >".green(),
            record,
            "<".green()
        );

        Ok(())
    }

    //
    // FUNCTION: Check if pacbio has fasta.gz files, cram has cram and crai and synteny has fasta
    //           could make this much easier and consise by passing in a list of file types to check
    //           validatedata(path, [fa, fna, fasta])
    //
    pub fn validate_data(path: &str, dtype: &str) {
        match fs::read_dir(path) {
            Err(e) if e.kind() == ErrorKind::NotFound => {}
            Err(e) => panic!("{} {e}", "<-DIRECTORY PATH DOESN'T EXIST: ".red().bold()),
            Ok(data_files) => {
                if dtype == "pacbio" {
                    let files: Vec<PathBuf> = data_files
                        .filter_map(|f| f.ok())
                        .filter(|d| match d.path().extension() {
                            None => false,
                            Some(ex) => ex == "fasta.gz",
                        })
                        .map(|f| f.path())
                        .collect();

                    if files.is_empty() {
                        println!("{}", "<-NO PACBIO DATA FILES".red())
                    } else {
                        println!("{} {:?}", ">-YOUR FILES ARE:".green(), &files);
                    }
                } else if dtype == "hic" {
                    let files: Vec<PathBuf> = data_files
                        .filter_map(|f| f.ok())
                        .filter(|d| match d.path().extension() {
                            None => false,
                            Some(ex) => ex == "cram" || ex == "crai",
                        })
                        .map(|f| f.path())
                        .collect();

                    if files.is_empty() {
                        println!("{}", "<-NO HIC DATA FILES".red())
                    } else {
                        println!("{} {:?}", ">-YOUR FILES ARE:".green(), &files);
                    }
                } else if dtype == "synteny" {
                    let files: Vec<PathBuf> = data_files
                        .filter_map(|f| f.ok())
                        .filter(|d| match d.path().extension() {
                            None => false,
                            Some(ex) => ex == "fa" || ex == "fasta" || ex == "fna",
                        })
                        .map(|f| f.path())
                        .collect();

                    if files.is_empty() {
                        println!("{}", "<-NO SYNTENIC GENOMES".red())
                    } else {
                        println!("{} {:?}", ">-YOUR GENOMES ARE:".green(), &files);
                    }
                }
            }
        };
    }

    pub fn validate_yaml(arguments: std::option::Option<&ArgMatches>) {
        let file = arguments.unwrap().get_one::<String>("yaml").unwrap();
        let _output: &String = arguments.unwrap().get_one::<String>("output").unwrap();
        let _verbose_flag: &bool = arguments.unwrap().get_one::<bool>("verbose").unwrap();

        println! {"Validating Yaml: {}", file.purple()};

        let input = fs::File::open(file).expect("Unable to read from file");
        let contents: TreeValYaml =
            serde_yaml::from_reader(input).expect("Unable to read from file");

        println!(
            "RUNNING VALIDATE-YAML FOR SAMPLE: {}",
            contents.assembly.sample_id.purple()
        );

        validate_paths(&contents.reference_file, "REFERENCE");
        validate_paths(&contents.alignment.data_dir, "GENESET");
        validate_paths(&contents.synteny.synteny_path, "SYNTENY");
        validate_paths(&contents.busco.lineages_path, "BUSCO");

        validate_paths(&contents.assem_reads.read_data, "PACBIO");
        validate_data(&contents.assem_reads.read_type, "pacbio");

        validate_paths(&contents.hic_data.hic_cram, "HIC");
        validate_data(&contents.hic_data.hic_aligner, "hic");

        println!("{}", "CHECKING GENESET DIRECTORY RESOLVES".blue());
        let genesets = contents.alignment.geneset_id.split(',');
        for set in genesets {
            let gene_alignment_path = contents.alignment.data_dir.clone()
                + &contents.assembly.defined_class
                + "/csv_data/"
                + set
                + "-data.csv";
            validate_paths(&gene_alignment_path, "GENESET-CSV");
        }

        println!("{}", "CHECKING SYNTENY DIRECTORY RESOLVES".blue());
        let synteny_full =
            contents.synteny.synteny_path.clone() + &contents.assembly.defined_class + "/";
        validate_paths(&synteny_full, "SYNTENY-FASTA");
        validate_data(&synteny_full, "synteny");

        println!("{}", "CHECKING BUSCO DIRECTORY RESOLVES".blue());
        let busco_path =
            contents.busco.lineages_path.clone() + "/lineages/" + &contents.busco.lineage;
        validate_paths(&busco_path, "BUSCO-DB");
        // NOW CHECK FOR FILES IN DIRECTORY?

        println!(
            "{}\n{}\n{}\n{}\n{}",
            "VALIDATION COMPLETE".purple().bold(),
            "GENERAL INFORMATION:".purple().bold(),
            "Check the log to see what failed".bold(),
            "FULL : ONLY synteny fails are permitted".purple(),
            "RAPID: geneset, busco and synteny fails are permitted".purple()
        );
    }
}
