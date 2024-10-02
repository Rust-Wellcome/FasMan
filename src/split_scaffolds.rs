pub mod split_scaffolds_mod {
    use crate::generics::validate_fasta;
    use clap::ArgMatches;

    pub fn split_scaffolds(arguments: std::option::Option<&ArgMatches>) {
        let fasta_file: &String = arguments.unwrap().get_one::<String>("fasta-file").unwrap();
        let scaff_limit: &usize = arguments.unwrap().get_one::<usize>("scaff_limit").unwrap();

        let validation = validate_fasta(fasta_file);

        println!("{:?}", validation);

        let scaffs_to_split = validation
            .as_ref()
            .unwrap()
            .keys()
            .filter(|x| validation.as_ref().unwrap()[x.to_owned()] > scaff_limit.to_owned());

        println!("{:?}", scaffs_to_split);
    }
}
