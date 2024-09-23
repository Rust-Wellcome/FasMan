use fasta_manipulation::yaml_validator_mod::{
    get_file_list, validate_paths, AssemReads, CRAMtags, HicReads, TreeValYaml, YamlResults,
};
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
    assert_eq!(
        failures.first().unwrap(),
        "Failed on: Reference | Value: FAIL"
    );
    assert_eq!(failures[1], "Failed on: Aligner | Value: FAIL");
    assert_eq!(failures[2], "Failed on: Longread Data | Value: FAIL");
    assert_eq!(failures[3], "Failed on: Busco Paths | Value: FAIL");
    assert_eq!(
        failures.last().unwrap(),
        "Failed on: Telomere Motif | Value: FAIL"
    )
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
    assert_eq!(
        failures.first().unwrap(),
        "Failed on: Reference | Value: FAIL"
    );
    assert_eq!(failures[1], "Failed on: Busco Paths | Value: FAIL");
    assert_eq!(
        failures.last().unwrap(),
        "Failed on: Telomere Motif | Value: FAIL"
    )
}

#[test]
fn check_check_secondaries_for_all_pass() {
    let vec_one = vec!["PASS".to_string()];
    let vec_two = vec!["PASS".to_string()];
    let vec_three = vec!["PASS".to_string()];
    let secondaries = vec![&vec_one, &vec_two, &vec_three];
    let yaml_results = YamlResults {
        ..Default::default()
    };
    let failures = yaml_results.check_secondaries(secondaries);
    assert!(failures.is_empty())
}

#[test]
fn check_check_secondaries_for_all_fails() {
    let vec_one = vec!["FAIL".to_string()];
    let vec_two = vec!["FAIL".to_string()];
    let vec_three = vec!["FAIL".to_string()];
    let secondaries = vec![&vec_one, &vec_two, &vec_three];
    let yaml_results = YamlResults {
        ..Default::default()
    };
    let failures = yaml_results.check_secondaries(secondaries);
    assert_eq!(*failures.first().unwrap(), "FAIL");
    assert_eq!(failures[1], "FAIL");
    assert_eq!(*failures.last().unwrap(), "FAIL");
}

#[test]
fn check_check_secondaries_for_all_nos() {
    let vec_one = vec!["NO".to_string()];
    let vec_two = vec!["NO".to_string()];
    let vec_three = vec!["NO".to_string()];
    let secondaries = vec![&vec_one, &vec_two, &vec_three];
    let yaml_results = YamlResults {
        ..Default::default()
    };
    let failures = yaml_results.check_secondaries(secondaries);
    assert_eq!(*failures.first().unwrap(), "NO");
    assert_eq!(failures[1], "NO");
    assert_eq!(*failures.last().unwrap(), "NO");
}

#[test]
fn check_check_secondaries_for_fails_and_nos() {
    let vec_one = vec!["FAIL".to_string()];
    let vec_two = vec!["NO".to_string()];
    let vec_three = vec!["FAIL".to_string()];
    let vec_four = vec!["PASS".to_string()];
    let secondaries = vec![&vec_one, &vec_two, &vec_three, &vec_four];
    let yaml_results = YamlResults {
        ..Default::default()
    };
    let failures = yaml_results.check_secondaries(secondaries);
    assert_eq!(*failures.first().unwrap(), "FAIL");
    assert_eq!(failures[1], "NO");
    assert_eq!(*failures.last().unwrap(), "FAIL");
}

#[test]
fn check_validate_fasta() {
    let tree_val_yaml = TreeValYaml {
        reference_file: "test_data/iyAndFlav1/tiny/tiny_test.fa".to_string(),
        ..Default::default()
    };

    assert!(tree_val_yaml.validate_fasta().contains("PASS"));

    let tree_val_yaml = TreeValYaml {
        reference_file: "test_data/iyAndFlav1/tiny/empty_file.txt".to_string(),
        ..Default::default()
    };
    assert!(tree_val_yaml.validate_fasta().contains("FAIL"));
}

#[test]
fn check_validate_csv() {
    let tree_val_yaml = TreeValYaml {
        ..Default::default()
    };

    assert!(tree_val_yaml
        .validate_csv(&"test_data/iyAndFlav1/tiny/valid_csv.csv".to_string())
        .contains("PASS"));

    let tree_val_yaml = TreeValYaml {
        ..Default::default()
    };
    assert!(tree_val_yaml
        .validate_csv(&"test_data/iyAndFlav1/tiny/empty_file.csv".to_string())
        .contains("FAIL"));
}

#[test]
fn check_validate_aligner_for_pass() {
    let hic_reads_bwamem2 = HicReads {
        hic_aligner: "bwamem2".to_string(),
        ..Default::default()
    };
    let hic_reads_minimap2 = HicReads {
        hic_aligner: "minimap2".to_string(),
        ..Default::default()
    };
    assert_eq!("PASS : bwamem2", hic_reads_bwamem2.validate_aligner());
    assert_eq!("PASS : minimap2", hic_reads_minimap2.validate_aligner());
}

#[test]
fn check_validate_aligner_for_fail() {
    let hic_reads = HicReads {
        hic_aligner: "bwa".to_string(),
        ..Default::default()
    };
    assert_eq!(
        "FAIL : bwa NOT IN [\"bwamem2\", \"minimap2\"]",
        hic_reads.validate_aligner()
    );
}

#[test]
fn check_validate_longread_pass() {
    let read_data = "test_data/iyAndFlav1/tiny/tiny_test.fa".to_string();
    let assem_read = AssemReads {
        read_data,
        ..Default::default()
    };
    assert_eq!(
        "PASS : test_data/iyAndFlav1/tiny/tiny_test.fa : FASTA.GZ = 1",
        assem_read.validate_longread()
    );
}

// Revise this test
#[test]
fn check_validate_longread_fail() {
    let read_data = "test_data/iyAndFlav1/tiny/empty.fasta.gz".to_string();
    let assem_read = AssemReads {
        read_data,
        ..Default::default()
    };
    assert_eq!(
        "PASS : test_data/iyAndFlav1/tiny/empty.fasta.gz : FASTA.GZ = 1",
        assem_read.validate_longread()
    );
}

#[test]
fn validate_longread_invalid_paths() {
    let read_data = "test_data/iyAndFlav1/tiny/tiny_test1.fa".to_string();
    let assem_read = AssemReads {
        read_data,
        ..Default::default()
    };
    assert_eq!(
        "FAIL : test_data/iyAndFlav1/tiny/tiny_test1.fa",
        assem_read.validate_longread()
    );
}
