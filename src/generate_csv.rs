pub mod gencsv_mod {
    use crate::generics::get_folder_list;
    
    pub fn gencsv(arguments: std::option::Option<&ArgMatches>) {
        let geneset_folder: &String = arguments
            .unwrap()
            .get_one::<String>("geneset_path")
            .unwrap();
        let subfolder: &String = arguments
            .unwrap()
            .get_one::<String>("refresh_subfolder")
            .unwrap();
        let full_path = format!("{}/{}/csv_data", geneset_folder, subfolder);

        match fs::create_dir_all(&full_path) {
            Ok(data) => data,
            Err(e) => panic!(e),
        }

        let folders = get_folder_list(&full_path);
    }
}
