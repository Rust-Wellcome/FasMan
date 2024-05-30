pub mod split_by_size_mod {
    use crate::generics::{only_keys, validate_fasta, write_fasta};
    use clap::ArgMatches;
    use noodles::fasta;
    use noodles::fasta::record::Definition;
    use noodles::fasta::repository::adapters::IndexedReader;
    use noodles::fasta::Record;
    use std::collections::HashMap;
    use std::path::Path;

    pub fn find_chunks<'a>(
        header_sizes: &'a HashMap<std::string::String, usize>,
        size: &usize,
    ) -> HashMap<usize, HashMap<&'a String, &'a usize>> {
        //let mut new_map = HashMap::new();
        let mut chunk = 1;
        let mut new_map: HashMap<usize, HashMap<&String, &usize>> = HashMap::new();
        let mut subset_map: HashMap<&String, &usize> = HashMap::new();
        let mut temp_map: HashMap<&String, &usize> = HashMap::new();

        for i in header_sizes {
            let scaff_name = i.0;
            let scaff_size = i.1;
            // If scaffold size is greater than chunk then output
            // straight away
            if i.1 > size {
                // Must be something cleaner for this bit
                temp_map.insert(scaff_name, scaff_size);
                new_map.insert(chunk, temp_map);

                // Clear Hashmap
                temp_map = HashMap::new();
                chunk += 1;
            // If Scaffold not > chunk size, add to HashMap
            // scan through HashMap and check whether greater than Chunk.
            } else {
                subset_map.insert(scaff_name, scaff_size);
                // If this list sums to larger than Chunk then
                // remove last item and check again.
                // if removing [-1] makes total size < chunk
                // out to file and keep that [-1] in list for next round of
                // chunking
                if subset_map.len() > 1 {
                    let summed: usize = subset_map.values().copied().sum();
                    if summed > *size {
                        subset_map.remove(scaff_name);
                        let summed: usize = subset_map.values().copied().sum();
                        if summed < *size {
                            new_map.insert(chunk, subset_map);
                            chunk += 1;
                        } else {
                            println!("ERROR: MORE LOGIC NEEDED TO SPLIT UP")
                        }
                        subset_map = HashMap::new();
                        subset_map.insert(scaff_name, scaff_size);
                    }
                }
            }
        }
        new_map.insert(chunk.to_owned(), subset_map.to_owned());

        new_map
    }

    pub fn split_file_by_size(arguments: std::option::Option<&ArgMatches>) {
        let fasta_file: &String = arguments.unwrap().get_one::<String>("fasta-file").unwrap();
        let chunk_size: &usize = arguments.unwrap().get_one::<usize>("mem-size").unwrap();
        let data_type: &String = arguments.unwrap().get_one::<String>("data_type").unwrap();
        let outpath: &String = arguments
            .unwrap()
            .get_one::<String>("output-directory")
            .unwrap();

        let path_obj = Path::new(fasta_file);
        let grab_name = path_obj.file_name().unwrap();
        let actual_list: Vec<&str> = grab_name.to_str().unwrap().split('.').collect();
        let actual_name = actual_list[0];

        let new_outpath = format!("{}/{}/{}/", outpath, actual_name, data_type);

        println!("Fasta file for processing: {:?}", &fasta_file);
        println!("Size to chunk fasta into: {:?}", &chunk_size);

        let validation = validate_fasta(fasta_file);

        // Deserved better error handling here
        let results = validation.unwrap();

        // Returns only the HashMap< usize, Hashmap<String, usize>>
        let split_hash = find_chunks(&results, chunk_size);

        // Duplicated from TPF_FASTA
        // Should be abstracted into generics
        let reader = fasta::indexed_reader::Builder::default().build_from_path(fasta_file);
        let fasta_repo = match reader {
            Ok(data) => {
                let adapter = IndexedReader::new(data);

                // Now read the fasta and return is as a queryable object
                fasta::Repository::new(adapter)
            }
            Err(_) => todo!(), // Probably just panic!
        };

        for i in split_hash {
            let mut record_list: Vec<Record> = Vec::new();
            let list: Vec<&String> = only_keys(i.1.to_owned()).collect();
            for ii in list {
                let results = fasta_repo.get(ii).transpose();
                let new_rec = match results {
                    Ok(data) => {
                        let definition = Definition::new(ii, None);
                        Record::new(definition, data.unwrap())
                    }
                    Err(e) => panic!("{:?}", e),
                };
                record_list.push(new_rec)
            }
            let file_name = format!("{}_f{}_{}.fasta", actual_name, i.0, data_type);

            let _ = write_fasta(&new_outpath, file_name, record_list);
        }
        //println!("{:?}", split_hash)
    }
}
