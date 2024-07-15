pub mod yaml_validator_mod {
    use clap::ArgMatches;
    use colored::{ColoredString, Colorize};
    use csv::ReaderBuilder;
    use noodles::{cram, fasta};
    use serde::{Deserialize, Serialize};
    use std::fs::{self, File};
    use std::path::PathBuf;
    use walkdir::WalkDir;

    #[derive(Debug)]
    struct CRAMtags<'a> {
        header_sort_order: &'a str,
        other_header_fields: &'a Vec<String>,
        reference_sequence: &'a usize,
        header_read_groups: &'a Vec<String>,
    }

    impl<'a> std::fmt::Display for CRAMtags<'a> {
        // Pretty Printing CRAMtags
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            write!(
                fmt,
                "CRAMtags:\n\t@SO {:?}\n\t@RG {:?}\n\t@?? {:?} <-- Other Tags\n\t@SQ {:?} Counted",
                self.header_sort_order,
                self.header_read_groups,
                self.other_header_fields,
                self.reference_sequence
            )
        }
    }

    // let mut reader = File::open("sample.cram").map(cram::io::Reader::new)?;
    // let header = reader.read_header()?;

    // for result in reader.records(&header) {
    //     let record = result?;
    //     // ...
    // }

    // Really each of the methods described below would be under their own struct,
    // however, many of them have cross overs so I have chosen to keep them all in
    // one place rather than split out some
    //
    // The methods here should be changing the values in the struct,
    // this would make reporting much easier as we could implement our own
    // fmt::display
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

    /// Struct functions
    impl TreeValYaml {
        /// A function to validate a path given as a &str
        fn validate_paths(&self, path: &str) -> ColoredString {
            match fs::metadata(path) {
                Ok(_) => format!("PASS : {}", &path).green(),
                Err(_) => format!("FAIL : {:?} <-- doesn't exist", &path).red(),
            }
        }

        // Replicate function from generate_csv
        fn get_file_list(&self, root: &str) -> Vec<PathBuf> {
            WalkDir::new(root)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
                .map(|e| e.into_path())
                .collect()
        }

        /// Validate that the input fasta is infact a fasta format and count records.
        fn validate_fasta(&self) -> ColoredString {
            let reader = fasta::reader::Builder.build_from_path(&self.reference_file);

            let mut binding = reader.expect("NO VALID HEADER / SEQUENCE PAIRS");
            let result = binding.records();
            let counter = result.count();
            if counter >= 1 {
                format!("PASS : FASTA CONTAINS - {} {}", counter, "H/S PAIRS").green()
            } else {
                format!("FAIL : NO HEADER/SEQ PAIRS").red()
            }
        }

        /// Validate the location of the busco databases
        fn validate_busco_path(&self) -> ColoredString {
            let full_busco_path = format!(
                "{}/lineages/{}",
                self.busco.lineages_path, self.busco.lineage
            );
            self.validate_paths(&full_busco_path)
        }

        /// Validate the location of the FASTA.GZ long read files
        fn validate_longread(&self) -> ColoredString {
            let main_path_check = self.validate_paths(&self.assem_reads.read_data);

            if main_path_check.contains("FAIL") {
                // Check that the above top level dir is valid and if fail break function
                return main_path_check;
            };

            let list_of_files = self.get_file_list(&self.assem_reads.read_data);

            let fasta_reads = &list_of_files
                .into_iter()
                .filter(|f| !f.ends_with(".fasta.gz"))
                .collect::<Vec<PathBuf>>();

            if !fasta_reads.is_empty() {
                format!(
                    "PASS ({}) FASTA.GZ = {}",
                    &self.assem_reads.read_data,
                    fasta_reads.len()
                )
                .green()
            } else {
                format!("FAIL ({}) NO READS", &self.assem_reads.read_data).red()
            }
        }

        fn get_cram_head(&self, cram_files: &Vec<PathBuf>) -> Result<(), std::io::Error> {
            for i in cram_files {
                let mut reader = File::open(i).map(cram::Reader::new)?;
                let head = reader.read_header()?;

                // Get read groups into a Vec otherwise you have a 100 long type that you can't do anything with.
                let readgroups: &Vec<_> = &head
                    .read_groups()
                    .keys()
                    .map(|x| x.to_string())
                    .collect::<Vec<std::string::String>>();
                let otherflags: &Vec<_> = &head
                    .header()
                    .unwrap()
                    .other_fields()
                    .into_iter()
                    .map(|y| format!("{} -- {}", y.0, y.1))
                    .collect::<Vec<std::string::String>>();

                let cram_obj = CRAMtags {
                    header_sort_order: &head.header().unwrap().sort_order().unwrap().to_string(),
                    other_header_fields: otherflags,
                    header_read_groups: readgroups, //.map(|x| x.to_string()),
                    reference_sequence: &head.reference_sequences().len(),
                };

                println!("{}", cram_obj);

                println!(
                    "{}",
                    format!("Confirm EOF (@??): ID WHETHER EOF EXISTS - NOODLES CRAM DOES NOT SUPPORT THE EOF CONTAINER")
                        .yellow()
                )
            }

            Ok(())
        }

        /// Validate the location of the CRAM file as well as whether a CRAI file is with it.
        /// TODO: Validate the contents of the CRAM
        /// - [x] NO SQ headers
        /// - [ ] first 100 reads and see whether they are sorted or come in pairs
        /// - [ ] samtools quickcheck -vvv - to see whether full file file and not corrupted
        fn validate_cram(&self) -> ColoredString {
            let main_path_check = &self.validate_paths(&self.hic_data.hic_cram);

            if main_path_check.contains("FAIL") {
                // Check that the above top level dir is valid and if fail break function
                return main_path_check.clone();
            };

            let list_of_files = self.get_file_list(&self.hic_data.hic_cram);

            let cram_files = &list_of_files
                .clone()
                .into_iter()
                .filter(|f| !f.ends_with(".cram"))
                .collect::<Vec<PathBuf>>();
            let crai_files = &list_of_files
                .into_iter()
                .filter(|f| !f.ends_with(".crai"))
                .collect::<Vec<PathBuf>>();

            let _ = self.get_cram_head(cram_files);

            if cram_files.len().eq(&crai_files.len()) && cram_files.len().ne(&0) {
                format!(
                    "PASS : {:?} : cram/crai = {}/{}",
                    cram_files,
                    cram_files.len(),
                    crai_files.len()
                )
                .green()
            } else {
                format!("FAIL : {:?} : INCORRECT NUMBER OF CRAM TO CRAI", cram_files).red()
            }
        }

        /// Validate the aligner against a set Vec of options
        fn validate_aligner(&self) -> ColoredString {
            // Should be const
            let aligners = vec!["bwamem2".to_string(), "minimap2".to_string()];
            if aligners.contains(&self.hic_data.hic_aligner.to_string()) {
                format!("PASS : {}", &self.hic_data.hic_aligner).green()
            } else {
                format!(
                    "FAIL : {} NOT IN {:?}",
                    &self.hic_data.hic_aligner, aligners
                )
                .red()
            }
        }

        fn validate_csv(&self, csv_path: &String) -> ColoredString {
            let file = File::open(&csv_path).unwrap();
            let name = &csv_path.split('/').collect::<Vec<&str>>();

            let mut reader = ReaderBuilder::new()
                .has_headers(true)
                .delimiter(b',')
                .from_reader(file);

            let record = reader.records().count();

            format!(
                "PASS : {:?} : {} : {} : {}",
                name.last().unwrap(),
                "RECORD-COUNT: >",
                record,
                "<"
            )
            .green()
        }

        /// Validate the geneset location, the presence of the csv file
        /// TODO: validate the contents of the csv file.
        fn validate_genesets(&self) -> Vec<ColoredString> {
            let mut exist_tuple = Vec::new();
            let genesets: Vec<&str> = self.alignment.geneset_id.split(',').collect();

            for i in genesets {
                // should probably be more of a if directory and csv file exist {pass} else {fail - check for csv and directory}
                let species_name: Vec<&str> = self.alignment.geneset_id.split('.').collect();

                let full_geneset_path = format!(
                    "{}/{}/{}",
                    self.alignment.data_dir, self.assembly.defined_class, species_name[0]
                );
                exist_tuple.push(self.validate_paths(&full_geneset_path));

                let full_geneset_csv = format!(
                    "{}/{}/csv_data/{}-data.csv",
                    self.alignment.data_dir, self.assembly.defined_class, i
                );
                exist_tuple.push(self.validate_paths(&full_geneset_csv));

                exist_tuple.push(self.validate_csv(&full_geneset_csv))
            }
            exist_tuple // shouldn't then use .all(|x| validate_paths(x)) to get one value because on fail we want to know which one
        }

        /// Validate the location of the synteny fasta files
        fn validate_synteny(&self) -> Vec<ColoredString> {
            // Very similar to genesets
            let mut exist_tuple = Vec::new();
            let syntenic_genomes: Vec<&str> = self.synteny.synteny_genomes.split(',').collect();

            let path_to_genome = format!(
                "{}/{}/",
                self.synteny.synteny_path, self.assembly.defined_class
            );

            let main_path_check = self.validate_paths(&path_to_genome);
            if main_path_check.contains("FAIL") {
                // Check that the above top level dir is valid and if fail break function
                exist_tuple.push(main_path_check);
                return exist_tuple;
            }

            // If the above is valid this second half of the function should then scan through the contents
            let list_of_paths = fs::read_dir(&path_to_genome).unwrap();

            let count_provided_syntenics = syntenic_genomes.len();
            let count_found_syntenics = &list_of_paths.count();

            // Fall towards more pythonic style here
            if count_provided_syntenics <= 1 {
                exist_tuple.push("NO SYNTENICS PROVIDED".to_string().yellow());
                exist_tuple
            } else {
                // This is pretty cool, reformat the string into the required path and then run and return a function on each.
                let mut full_paths: Vec<ColoredString> = syntenic_genomes
                    .into_iter()
                    .map(|x| format!("{}{}.fasta", path_to_genome, x))
                    .map(|x| self.validate_paths(&x))
                    .collect();

                full_paths.push(
                    format!(
                        "AVAILABLE: {} | REQUESTED: {}",
                        count_found_syntenics,
                        exist_tuple.len()
                    )
                    .blue(),
                );

                full_paths
            }
        }

        // Validate whether a previous kmer profile exists
        fn validate_kmer_prof(&self) -> ColoredString {
            let ktab_path = format!(
                "{}/k{}/{}.k{}.ktab",
                &self.kmer_profile.dir,
                &self.kmer_profile.kmer_length.to_string(),
                &self.assembly.sample_id,
                &self.kmer_profile.kmer_length.to_string()
            );

            self.validate_paths(&ktab_path)
        }

        /// Validate whether the telomere motif is ALPHABETICAL
        /// No upper bound as motifs can be large.
        fn validate_telomere(&self) -> ColoredString {
            if self.telomere.teloseq.chars().all(char::is_alphabetic)
                && self.telomere.teloseq.chars().collect::<Vec<_>>().len() > 3
            {
                format!("PASS : {}", &self.telomere.teloseq).green()
            } else {
                format!("FAIL : {}", &self.telomere.teloseq).red()
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct KmerProfile {
        kmer_length: u16,
        dir: String,
    }

    impl KmerProfile {}

    #[derive(Debug, Serialize, Deserialize)]
    struct HicReads {
        hic_cram: String,
        hic_aligner: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Assembly {
        sample_id: String,  // Anything the user wants
        latin_name: String, // Not in use but maybe in future, how to validate a latin name. Api call with a fallback to yes... it is alphabetical
        defined_class: String,
        assem_version: u8,  // Any number
        project_id: String, // Can be anything the user wants, not in use
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct AssemReads {
        read_type: String,
        read_data: String,
        supplementary_data: String, // Not yet in use
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Alignment {
        data_dir: String,
        common_name: String, // Not yet in use
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

    fn print_pretty(input: Vec<ColoredString>) {
        for i in input {
            println!("\t--{}", i);
        }
    }

    pub fn validate_yaml(arguments: std::option::Option<&ArgMatches>) {
        let file = arguments.unwrap().get_one::<String>("yaml").unwrap();
        let _output: &String = arguments.unwrap().get_one::<String>("output").unwrap();
        let _verbose_flag: &bool = arguments.unwrap().get_one::<bool>("verbose").unwrap();

        println! {"Validating Yaml: {}", file.purple()};

        let input = fs::File::open(file).expect("Unable to read from file");
        let contents: TreeValYaml =
            serde_yaml::from_reader(input).expect("Unable to read from file");

        println!("FASTA VALID: {}", contents.validate_fasta());
        println!("CRAM       : {}", contents.validate_cram());
        println!("ALIGNER    : {}", contents.validate_aligner());
        println!("LONGREAD   : {}", contents.validate_longread());
        println!("BUSCO PATH : {}", contents.validate_busco_path());
        println!("GENESET P. : -"); // :? because there is no impl of displaying vec of colouredStr
        print_pretty(contents.validate_genesets());
        println!("TELOMOT P. : {}", contents.validate_telomere());
        println!("SYNTENICS P: -");
        print_pretty(contents.validate_synteny());
        println!("KMER PROF P: {}", contents.validate_kmer_prof());
    }
}
