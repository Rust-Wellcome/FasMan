pub fn split_file_by_size(fasta_file: &String, mem_size: &u16, _output_directory: &str) {
    println!("Fasta file for processing: {:?}", &fasta_file);
    println!("Size to chunk fasta into: {:?}", mem_size);
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
