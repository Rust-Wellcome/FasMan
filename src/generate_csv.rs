/// Generate CSV generates a csv file which describes a specific data directory /User/...../geneset_alignment_data/insect/ApisMeliffera/ApisMeliffera.AMel1_1/{pep,cdna,rna,cds}/files.fa
/// This is for data tracking for TreeVal
/// This may be replaced or enhanced with a function to send this to a Google Sheets so the team has an easier way of tracking it all.
pub mod gencsv_mod {
    use crate::generics::get_folder_list;
    use clap::ArgMatches;
    use csv::Writer;
    use std::collections::HashMap;
    use std::error::Error;
    use std::{fs, path::Path, path::PathBuf};
    use walkdir::WalkDir;

    fn get_file_list(root: &str) -> Vec<PathBuf> {
        WalkDir::new(root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| e.into_path())
            .collect()
    }

    // Function to convert list to dictionary
    fn list_2_dict(file_list: &Vec<PathBuf>) -> (HashMap<String, Vec<String>>, String) {
        let mut file_dict = HashMap::new();
        let mut org = String::new();
        for path in file_list {
            let path_str = path.to_str().unwrap();
            let path_list: Vec<&str> = path_str.split('/').collect();
            let file_name = path_list[path_list.len() - 1];
            if file_name.to_lowercase() != "readme.txt" && file_name.to_lowercase() != "readme" {
                file_dict.insert(
                    file_name.to_string(),
                    vec![
                        path_list[path_list.len() - 3].to_string(),
                        path_list[path_list.len() - 2].to_string(),
                        path_str.to_string(),
                    ],
                );
                org = path_list[path_list.len() - 3].to_string();
            }
        }
        (file_dict, org)
    }

    fn save_data(
        dict_of_data: HashMap<String, Vec<String>>,
        save_loc: &str,
        org_accession: &str,
    ) -> Result<(), Box<dyn Error>> {
        let save_dir = format!("{}/csv_data", save_loc);

        let save_path = format!("{}/csv_data/{}-data.csv", save_loc, org_accession);
        let save_path = Path::new(&save_path);

        // Ensure the save directory exists
        if !Path::new(&save_dir).exists() {
            fs::create_dir_all(&save_dir).unwrap();
        }

        if save_path.exists() {
            fs::remove_file(save_path).unwrap();
        }

        println!(
            "Generating CSV for:\t{}\nSave Path:\t\t{}",
            org_accession,
            save_path.display()
        );

        println!("{}", save_dir);

        let mut wtr = Writer::from_path(save_path)?;
        wtr.write_record(&["org", "type", "data_file"])?;
        for (_key, value) in dict_of_data {
            wtr.write_record(&value)?;
        }
        wtr.flush()?;
        Ok(())
    }

    pub fn gencsv(arguments: std::option::Option<&ArgMatches>) {
        let geneset_folder: &String = arguments.unwrap().get_one::<String>("geneset_dir").unwrap();

        let clade_folder = get_folder_list(&geneset_folder);

        for clade in clade_folder {
            let save_clade = clade.clone();
            let org_folder = get_folder_list(&clade.into_os_string().into_string().unwrap());

            // Filter out the folders ending with csv_data as these are output folders
            let new_org_folder: Vec<&PathBuf> = org_folder
                .iter()
                .filter(|x| !x.ends_with("csv_data"))
                .collect();

            for org in new_org_folder {
                let mut master_list = Vec::new();

                let accession_folder = get_folder_list(
                    &<PathBuf as Clone>::clone(&org)
                        .into_os_string()
                        .into_string()
                        .unwrap(),
                );

                for accession in accession_folder {
                    let data_list = get_folder_list(accession.to_str().unwrap());
                    for data in data_list {
                        master_list.push(get_file_list(data.to_str().unwrap()));
                    }

                    let file_dict: HashMap<String, Vec<String>>;
                    let orgs: String;
                    (file_dict, orgs) =
                        list_2_dict(&master_list.iter().flatten().cloned().collect());
                    let save_loc = format!(
                        "{}/{}",
                        geneset_folder,
                        save_clade.file_name().unwrap().to_str().unwrap()
                    );
                    let _ = save_data(file_dict, &save_loc, &orgs);
                }
            }
        }
    }
}
