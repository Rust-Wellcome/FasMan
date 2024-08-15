pub mod yaml_validator_mod {
    use clap::ArgMatches;
    use colored::Colorize;
    use csv::ReaderBuilder;
    use noodles::{cram, fasta};
    use serde::{Deserialize, Serialize};
    use std::fs::{self, File};
    use std::marker::PhantomData;
    use std::path::PathBuf;
    use walkdir::WalkDir;

    /// A function to validate a path given as a &str
    fn validate_paths(path: &str) -> String {
        match fs::metadata(path) {
            Ok(_) => format!("PASS : {}", &path),
            Err(_) => format!("FAIL : {}", &path),
        }
    }

    // Replicate function from generate_csv
    fn get_file_list(root: &str) -> Vec<PathBuf> {
        WalkDir::new(root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| e.into_path())
            .collect()
    }

    #[derive(Debug, Serialize, Deserialize)]
    // https://doc.rust-lang.org/std/marker/struct.PhantomData.html
    struct YamlResults<'a> {
        ReferenceResults: String,
        CramResults: CRAMtags,
        AlignerResults: String,
        LongreadResults: String,
        BuscoResults: String,
        TelomereResults: String,
        KmerProfileResults: String,
        GenesetResults: Vec<String>,
        SyntenicResults: Vec<String>,
        phantom: PhantomData<&'a String>,
    }

    impl<'a> std::fmt::Display for YamlResults<'a> {
        // Pretty Printing YamlResults
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            write!(
                fmt,
                "YamlResults:\n\tReference: {:#?}\n\tCram: {:#?}\n\tAligner: {:#?}\n\tLongread: {:#?}\n\tBusco: {:#?}\n\tTelomere: {:#?}\n\tKmerProfile: {:#?}\n\tGenesetPaths: {:#?}\n\tSyntenicPaths: {:#?}\n\t{:#?}",
                &self.ReferenceResults,
                &self.is_cram_valid(),
                &self.AlignerResults,
                &self.LongreadResults,
                &self.BuscoResults,
                &self.TelomereResults,
                &self.KmerProfileResults,
                &self.GenesetResults,
                &self.SyntenicResults,
                &self.CramResults,
            )
        }
    }

    impl<'a> YamlResults<'a> {
        fn is_cram_valid(&self) -> String {
            // this should add a field to the cramresults struct
            if &self.CramResults.header_read_groups.len() >= &1 {
                "PASS".to_string()
            } else {
                "FAIL".to_string()
            }
        }

        fn to_stdout(&self) {
            println!("{}", &self)
        }

        fn to_file(&self, output_location: String) -> Result<(), std::io::Error> {
            let string_data = format!("YamlResults:\n\tReference: {:#?}\n\tCram: {:#?}\n\tAligner: {:#?}\n\tLongread: {:#?}\n\tBusco: {:#?}\n\tTelomere: {:#?}\n\tKmerProfile: {:#?}\n\tGenesetPaths: {:#?}\n\tSyntenicPaths: {:#?}\n\t{:#?}",
                            &self.ReferenceResults,
                            &self.is_cram_valid(),
                            &self.AlignerResults,
                            &self.LongreadResults,
                            &self.BuscoResults,
                            &self.TelomereResults,
                            &self.KmerProfileResults,
                            &self.GenesetResults,
                            &self.SyntenicResults,
                            &self.CramResults,
                        );
            fs::write(output_location, string_data)
        }

        fn check_primaries(&self, primary_list: Vec<Vec<&str>>) -> Vec<String> {
            let mut failures = Vec::new();
            for i in primary_list {
                if !i[1].contains("PASS") {
                    failures.push(format!("Failed on: {} | Value: {}", i[0], i[1]));
                }
            }
            failures
        }

        fn check_secondaries(&'a self, secondary_list: Vec<&'a Vec<String>>) -> Vec<&String> {
            let mut failures: Vec<&String> = Vec::new();
            for i in secondary_list {
                let collection = i
                    .into_iter()
                    .filter(|j| j.contains("FAIL") || j.contains("NO"))
                    .collect::<Vec<&String>>();

                for i in collection {
                    failures.push(i)
                }
            }

            failures
        }

        /// Check the struct and check whether
        fn to_check(&self) {
            // Primary fields are where the program must quit
            // and error out, these fields are essential to
            // TreeVal.
            // Secondary fields are those which can FAIL and
            // will not cause a TreeVal run to fail,
            // may cause missing data if accidentaly ommitted.
            let primary_fields: Vec<Vec<&str>> = vec![
                vec!["Reference", &self.ReferenceResults],
                vec!["Aligner", &self.AlignerResults],
                vec!["Longread Data", &self.LongreadResults],
                vec!["Busco Paths", &self.BuscoResults],
                vec!["Telomere Motif", &self.TelomereResults],
            ];
            let secondary_fields: Vec<&Vec<String>> =
                vec![&self.GenesetResults, &self.SyntenicResults];

            let failed_primaries = self.check_primaries(primary_fields);
            let failed_secondary = self.check_secondaries(secondary_fields);

            let failed_primary_count = &failed_primaries.len();
            let failed_secondary_count = &failed_secondary.len();

            if &failed_primaries.len() >= &1 {
                println!(
                    "Primary Values Failed: {}\nSecondary Values Failed: {}\nPrimary Values that failed:\n{:?}\nSecondary Values that failed (These are not essential for TreeVal):\n{:?}\n",
                    failed_primary_count, failed_secondary_count,
                    failed_primaries, failed_secondary
                );
                std::process::exit(1)
            } else if &failed_secondary.len() >= &1 {
                println!("Secondary Values Failed: {}\nSecondary Values that failed (These are not essential for TreeVal):\n{:?}\n",
                    failed_secondary_count, failed_secondary)
            } else {
                println!("All passed!")
            }
        }
    }

    // Default allows us to create an empty Struct later on,
    // This was helpful for breaking out of a function early
    // without having to generate some dummy files.
    #[derive(Debug, Serialize, Deserialize, Default)]
    struct CRAMtags {
        header_sort_order: Vec<String>,
        other_header_fields: Vec<String>,
        reference_sequence: Vec<usize>,
        header_read_groups: Vec<String>,
    }

    impl<'a> std::fmt::Display for CRAMtags {
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
        /// Pour the results into a results struct
        fn into_results(&self) -> YamlResults {
            let results = YamlResults {
                ReferenceResults: self.validate_fasta(),
                CramResults: self.hic_data.validate_cram().1,
                AlignerResults: self.hic_data.validate_aligner(),
                LongreadResults: self.assem_reads.validate_longread(),
                BuscoResults: self.busco.validate_busco_path(),
                TelomereResults: self.telomere.validate_telomere(),
                KmerProfileResults: self.validate_kmer_prof(),
                GenesetResults: self.validate_genesets(),
                SyntenicResults: self.validate_synteny(),
                phantom: PhantomData,
            };
            results
        }

        /// Validate that the input fasta is infact a fasta format and count records.
        fn validate_fasta(&self) -> String {
            let reader = fasta::reader::Builder.build_from_path(&self.reference_file);

            let mut binding = reader.expect("NO VALID HEADER / SEQUENCE PAIRS");
            let result = binding.records();
            let counter = result.count();
            if counter >= 1 {
                format!("PASS : FASTA CONTAINS - {} {}", counter, "H/S PAIRS")
            } else {
                "FAIL : NO HEADER/SEQ PAIRS".to_string()
            }
        }

        fn validate_csv(&self, csv_path: &String) -> String {
            let file = File::open(csv_path);

            match file {
                Ok(valid_data) => {
                    format!("PASS: {}", csv_path);
                    let name = &csv_path.split('/').collect::<Vec<&str>>();

                    let mut reader = ReaderBuilder::new()
                        .has_headers(true)
                        .delimiter(b',')
                        .from_reader(valid_data);

                    format!(
                        "PASS : {:?}=RECORD-COUNT: >{}<",
                        name.last().unwrap(),
                        reader.records().count(),
                    )
                }
                Err(error) => return format!("FAIL : {}", error),
            }
        }

        /// Validate the geneset location, the presence of the csv file
        /// TODO: validate the contents of the csv file.
        fn validate_genesets(&self) -> Vec<String> {
            let mut exist_tuple = Vec::new();
            let genesets: Vec<&str> = self.alignment.geneset_id.split(',').collect();

            for i in genesets {
                // should probably be more of a if directory and csv file exist {pass} else {fail - check for csv and directory}
                let species_name: Vec<&str> = self.alignment.geneset_id.split('.').collect();

                let full_geneset_path = format!(
                    "{}/{}/{}",
                    self.alignment.data_dir, self.assembly.defined_class, species_name[0]
                );
                exist_tuple.push(validate_paths(&full_geneset_path));

                let full_geneset_csv = format!(
                    "{}/{}/csv_data/{}-data.csv",
                    self.alignment.data_dir, self.assembly.defined_class, i
                );
                exist_tuple.push(validate_paths(&full_geneset_csv));

                exist_tuple.push(self.validate_csv(&full_geneset_csv))
            }
            exist_tuple // shouldn't then use .all(|x| validate_paths(x)) to get one value because on fail we want to know which one
        }

        /// Validate the location of the synteny fasta files
        fn validate_synteny(&self) -> Vec<String> {
            // Very similar to genesets
            let mut exist_tuple = Vec::new();
            let syntenic_genomes: Vec<&str> = self.synteny.synteny_genomes.split(',').collect();

            let path_to_genome = format!(
                "{}/{}/",
                self.synteny.synteny_path, self.assembly.defined_class
            );

            let main_path_check = validate_paths(&path_to_genome);
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
                exist_tuple.push("NO SYNTENICS PROVIDED".to_string());
                exist_tuple
            } else {
                // This is pretty cool, reformat the string into the required path and then run and return a function on each.
                let mut full_paths: Vec<String> = syntenic_genomes
                    .into_iter()
                    .map(|x| format!("{}{}.fasta", path_to_genome, x))
                    .map(|x| validate_paths(&x))
                    .collect();

                full_paths.push(format!(
                    "AVAILABLE: {} | REQUESTED: {}",
                    count_found_syntenics,
                    exist_tuple.len()
                ));

                full_paths
            }
        }

        // Validate whether a previous kmer profile exists
        fn validate_kmer_prof(&self) -> String {
            let ktab_path = format!(
                "{}/k{}/{}.k{}.ktab",
                &self.kmer_profile.dir,
                &self.kmer_profile.kmer_length.to_string(),
                &self.assembly.sample_id,
                &self.kmer_profile.kmer_length.to_string()
            );

            validate_paths(&ktab_path)
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

    impl HicReads {
        /// Validate the aligner against a set Vec of options
        fn validate_aligner(&self) -> String {
            // Should be const
            let aligners = vec!["bwamem2".to_string(), "minimap2".to_string()];
            if aligners.contains(&self.hic_aligner.to_string()) {
                format!("PASS : {}", &self.hic_aligner)
            } else {
                format!("FAIL : {} NOT IN {:?}", &self.hic_aligner, aligners)
            }
        }

        /// Grab the data from the cram header and generate a small report
        fn get_cram_head(&self, cram_files: &Vec<PathBuf>) -> Result<CRAMtags, std::io::Error> {
            let mut header_sort_order: Vec<String> = Vec::new();
            let mut header_read_groups: Vec<String> = Vec::new();
            let mut other_header_fields: Vec<String> = Vec::new();
            let mut reference_sequence: Vec<usize> = Vec::new();

            let sort_order_key: [u8; 2] = [b'S', b'O'];
            for i in cram_files {
                let mut reader = File::open(i).map(cram::io::Reader::new)?;
                let head = reader.read_header()?;

                // Get read groups into a Vec otherwise you have a 100 long type that you can't do anything with.
                let read_groups: String = head
                    .read_groups()
                    .keys()
                    .map(|x| x.to_string())
                    .collect::<Vec<std::string::String>>()
                    .join("-&-");
                header_read_groups.push(read_groups);

                let other_headers = head
                    .header()
                    .unwrap()
                    .other_fields()
                    .into_iter()
                    .map(|y| format!("@{}: {}", y.0, y.1))
                    .collect::<std::string::String>();
                other_header_fields.push(other_headers);

                let x = &head
                    .header()
                    .unwrap()
                    .other_fields()
                    .get(&sort_order_key)
                    .unwrap()
                    .to_owned();
                header_sort_order.push(x.to_string());

                let reference_sequence_value = head.reference_sequences().len();
                reference_sequence.push(reference_sequence_value);
            }

            let cram_ob = CRAMtags {
                header_sort_order,
                other_header_fields,
                header_read_groups,
                reference_sequence,
            };
            Ok(cram_ob)
        }

        /// Validate the location of the CRAM file as well as whether a CRAI file is with it.
        /// TODO: Validate the contents of the CRAM
        /// - [x] NO SQ headers
        /// - [ ] first 100 reads and see whether they are sorted or come in pairs
        /// - [ ] samtools quickcheck -vvv - to see whether full file file and not corrupted
        fn validate_cram(&self) -> (String, CRAMtags) {
            let main_path_check = validate_paths(&self.hic_cram);

            if main_path_check.contains("FAIL") {
                // Check that the above top level dir is valid and if fail break function
                return (main_path_check.clone(), CRAMtags::default());
            };

            let list_of_files = get_file_list(&self.hic_cram);

            let cram_files = &list_of_files
                .clone()
                .into_iter()
                .filter(|f| "cram" == f.extension().unwrap().to_str().unwrap())
                .collect::<Vec<PathBuf>>();
            let crai_files = &list_of_files
                .into_iter()
                .filter(|f| "crai" == f.extension().unwrap().to_str().unwrap())
                .collect::<Vec<PathBuf>>();

            let cram_head = self.get_cram_head(&cram_files).unwrap();

            // If number of cram file is eq to number of crai (index) files AND cram_files doesn't eq 0
            if cram_files.len().eq(&crai_files.len()) && cram_files.len().ne(&0) {
                (
                    format!(
                        "PASS : {:?} : cram/crai = {}/{}",
                        cram_files,
                        cram_files.len(),
                        crai_files.len()
                    ),
                    cram_head,
                )
            } else {
                (
                    format!("FAIL : {:?} : INCORRECT NUMBER OF CRAM TO CRAI", cram_files),
                    cram_head,
                )
            }
        }
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

    impl AssemReads {
        /// Validate the location of the FASTA.GZ long read files
        fn validate_longread(&self) -> String {
            let main_path_check = validate_paths(&self.read_data);

            if main_path_check.contains("FAIL") {
                // Check that the above top level dir is valid and if fail break function
                return main_path_check;
            };

            let list_of_files = get_file_list(&self.read_data);

            let fasta_reads = &list_of_files
                .into_iter()
                .filter(|f| !f.ends_with(".fasta.gz"))
                .collect::<Vec<PathBuf>>();

            if !fasta_reads.is_empty() {
                format!(
                    "PASS : {} : FASTA.GZ = {}",
                    &self.read_data,
                    fasta_reads.len() // TODO: Placeholder - Hopefully will eventually be
                )
            } else {
                format!("FAIL ({}) NO READS", &self.read_data)
            }
        }
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

    impl Telomere {
        /// Validate whether the telomere motif is ALPHABETICAL
        /// No upper bound as motifs can be large.
        fn validate_telomere(&self) -> String {
            if self.teloseq.chars().all(char::is_alphabetic)
                && self.teloseq.chars().collect::<Vec<_>>().len() > 3
            {
                format!("PASS : {}", &self.teloseq)
            } else {
                format!("FAIL : {}", &self.teloseq)
            }
        }
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

    impl Busco {
        /// Validate the location of the busco databases
        fn validate_busco_path(&self) -> String {
            let full_busco_path = format!("{}/lineages/{}", self.lineages_path, self.lineage);
            validate_paths(&full_busco_path)
        }
    }

    /// Validate the yaml file required for the TreeVal pipeline
    pub fn validate_yaml(arguments: std::option::Option<&ArgMatches>) {
        let file = arguments.unwrap().get_one::<String>("yaml").unwrap();
        let output_to_file: &bool = arguments
            .unwrap()
            .get_one::<bool>("output_to_file")
            .unwrap();
        let output_to_stdout: &bool = arguments
            .unwrap()
            .get_one::<bool>("output_to_stdout")
            .unwrap();
        let output_to_pipeline: &bool = arguments
            .unwrap()
            .get_one::<bool>("output_to_pipeline")
            .unwrap();

        let output_file = if output_to_file.to_owned() {
            "./yamlresults.txt".to_string()
        } else {
            "".to_string()
        };

        println! {"Validating Yaml: {}", file.purple()};

        let input = fs::File::open(file).expect("Unable to read from file");
        let contents: TreeValYaml =
            serde_yaml::from_reader(input).expect("Unable to read from file");

        let results = contents.into_results();

        if output_to_stdout == &true {
            results.to_stdout();
        }

        if output_to_file == &true {
            results
                .to_file(output_file)
                .expect("Can't create final report");
        }

        if output_to_pipeline == &true {
            results.to_check()
        }
    }
}
