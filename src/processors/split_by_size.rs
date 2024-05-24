use clap::ArgMatches;

pub fn split_file_by_size(fasta_file: &String, mem_size: &u16, output_directory: &String) {
    println!("Fasta file for processing: {:?}", &fasta_file);
    println!("Size to chunk fasta into: {:?}", mem_size);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
