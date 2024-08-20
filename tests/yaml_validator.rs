use fasta_manipulation::yaml_validator_mod::{get_file_list, validate_paths, YamlResults};
use std::path::PathBuf;

#[test]
fn check_validate_paths() {
    assert!(validate_paths("test_data/yaml/test.yaml").contains("PASS"));
    assert!(validate_paths("tests/data/invalid.yaml").contains("FAIL"));
}

#[test]
fn check_get_file_list() {
    let path = "test_data/iyAndFlav1".to_string();
    let expected_file_list: Vec<PathBuf> = vec![
        PathBuf::from("test_data/iyAndFlav1/full/iyAndFlav1_subset.fa.fai"),
        PathBuf::from("test_data/iyAndFlav1/full/iyAndFlav1_subset.fa"),
        PathBuf::from("test_data/iyAndFlav1/full/iyAndFlav1.curated_subset.tpf"),
        PathBuf::from("test_data/iyAndFlav1/small/small_test.fa.fai"),
        PathBuf::from("test_data/iyAndFlav1/small/small_test.fa"),
        PathBuf::from("test_data/iyAndFlav1/small/small_test.curated.tpf"),
        PathBuf::from("test_data/iyAndFlav1/small/small_test.output.fasta"),
        PathBuf::from("test_data/iyAndFlav1/tiny/tiny_test.curated.tpf"),
        PathBuf::from("test_data/iyAndFlav1/tiny/tiny_test.output.fasta"),
        PathBuf::from("test_data/iyAndFlav1/tiny/tiny_test.fa"),
        PathBuf::from("test_data/iyAndFlav1/tiny/tiny_test.fa.fai"),
        PathBuf::from("test_data/iyAndFlav1/tiny/tiny_test.debug.txt"),
    ];
    let file_list = get_file_list(&path);
    assert_eq!(expected_file_list, file_list);
}
