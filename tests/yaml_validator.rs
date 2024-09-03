use fasta_manipulation::yaml_validator_mod::{get_file_list, validate_paths, CRAMtags, YamlResults};
use std::path::PathBuf;

#[test]
fn check_validate_paths() {
    assert!(validate_paths("test_data/yaml/test.yaml").contains("PASS"));
    assert!(validate_paths("tests/data/invalid.yaml").contains("FAIL"));
}

#[test]
fn check_get_file_list() {
    let path = "test_data/iyAndFlav1".to_string();
    let expected_file_list = vec![
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
    ]
    .sort();
    let file_list = get_file_list(&path).sort();
    assert_eq!(expected_file_list, file_list);
}

#[test]
fn check_is_cram_valid() {
    let cram_tags_not_empty = CRAMtags {
        header_read_groups: vec![String::from("test")],
        ..Default::default()
    };
    let yaml_results_pass = YamlResults {
        cram_results: cram_tags_not_empty,
        ..Default::default()
    };
    assert_eq!("PASS", yaml_results_pass.is_cram_valid())
}

#[test]
fn check_is_cram_invalid() {
    let cram_tags_empty = CRAMtags {
        header_read_groups: vec![],
        ..Default::default()
    };
    let yaml_results_fail = YamlResults {
        cram_results: cram_tags_empty,
        ..Default::default()
    };
    assert_eq!("FAIL", yaml_results_fail.is_cram_valid())
}

#[test]
fn check_check_primaries_with_all_fails() {
    let primaries = vec![
        vec!["Reference", "FAIL"],
        vec!["Aligner", "FAIL"],
        vec!["Longread Data", "FAIL"],
        vec!["Busco Paths", "FAIL"],
        vec!["Telomere Motif", "FAIL"],
    ];
    let yaml_results = YamlResults {
        ..Default::default()
    };
    let failures = yaml_results.check_primaries(primaries);
    assert_eq!(failures.first().unwrap(), "Failed on: Reference | Value: FAIL");
    assert_eq!(failures[1], "Failed on: Aligner | Value: FAIL");
    assert_eq!(failures[2], "Failed on: Longread Data | Value: FAIL");
    assert_eq!(failures[3], "Failed on: Busco Paths | Value: FAIL");
    assert_eq!(failures.last().unwrap(), "Failed on: Telomere Motif | Value: FAIL")
}

#[test]
fn check_check_primaries_with_all_passes() {
    let primaries = vec![
        vec!["Reference", "PASS"],
        vec!["Aligner", "PASS"],
        vec!["Longread Data", "PASS"],
        vec!["Busco Paths", "PASS"],
        vec!["Telomere Motif", "PASS"],
    ];
    let yaml_results = YamlResults {
        ..Default::default()
    };
    let failures = yaml_results.check_primaries(primaries);
    assert!(failures.is_empty())
}

#[test]
fn check_check_primaries_with_fails_and_passes() {
    let primaries = vec![
        vec!["Reference", "FAIL"],
        vec!["Aligner", "PASS"],
        vec!["Longread Data", "PASS"],
        vec!["Busco Paths", "FAIL"],
        vec!["Telomere Motif", "FAIL"],
    ];
    let yaml_results = YamlResults {
        ..Default::default()
    };
    let failures = yaml_results.check_primaries(primaries);
    assert_eq!(failures.first().unwrap(), "Failed on: Reference | Value: FAIL");
    assert_eq!(failures[1], "Failed on: Busco Paths | Value: FAIL");
    assert_eq!(failures.last().unwrap(), "Failed on: Telomere Motif | Value: FAIL")
}
