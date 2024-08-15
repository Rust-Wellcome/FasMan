use assert_cmd::Command;
use noodles::fasta::fai;
use std::fs;
use std::fs::File;
use std::io::Write;

use noodles::fasta::record::Sequence;
use tempfile::Builder;
use tempfile::NamedTempFile;
// pub use fasta_manipulation::tpf_fasta::*;
use fasta_manipulation::tpf_fasta_mod::{
    check_orientation, get_uniques, parse_seq, parse_tpf, save_to_fasta, subset_vec_tpf, NewFasta,
    Tpf,
};

mod util;

use util::are_files_identical;

// To test the check orientation function we need to publicly expose it
// Is there a way to test private functions?
#[test]
fn check_orientation_inverts_sequence_if_minus() {
    let sequence = Sequence::from(b"ATGC".to_vec());
    let orientation = "MINUS".to_string();
    let result = check_orientation(Some(sequence), orientation);
    assert_eq!(result, "GCAT".to_string());
}

#[test]
fn check_orientation_does_not_invert_sequence_if_plus() {
    let sequence = Sequence::from(b"ATGC".to_vec());
    let orientation = "PLUS".to_string();
    let result = check_orientation(Some(sequence), orientation);
    assert_eq!(result, "ATGC".to_string());
}

// Again we need to publicly expose the get_uniques function to test it
// Also we need to publicly expose the Tpf struct attributes
// Do we need a factory function to create Tpf structs?
#[test]
fn get_uniques_returns_unique_scaffold_names() {
    let tpf1 = Tpf {
        ori_scaffold: "scaffold1".to_string(),
        start_coord: 1,
        end_coord: 100,
        new_scaffold: "newScaffold1".to_string(),
        orientation: "PLUS".to_string(),
    };
    let tpf2 = Tpf {
        ori_scaffold: "scaffold2".to_string(),
        start_coord: 1,
        end_coord: 100,
        new_scaffold: "newScaffold2".to_string(),
        orientation: "PLUS".to_string(),
    };
    let tpf3 = Tpf {
        ori_scaffold: "scaffold1".to_string(),
        start_coord: 1,
        end_coord: 100,
        new_scaffold: "newScaffold1".to_string(),
        orientation: "PLUS".to_string(),
    };
    let tpfs = vec![tpf1, tpf2, tpf3];
    let result = get_uniques(&tpfs);
    assert_eq!(
        result,
        vec!["newScaffold1".to_string(), "newScaffold2".to_string()]
    );
}

// Need to add some docs for function
// as we were not entirely sure what it was doing
#[test]
fn get_subset_of_tpfs() {
    let tpf1 = Tpf {
        ori_scaffold: "scaffold1".to_string(),
        start_coord: 1,
        end_coord: 100,
        new_scaffold: "newScaffold1".to_string(),
        orientation: "PLUS".to_string(),
    };
    let tpf2 = Tpf {
        ori_scaffold: "scaffold2".to_string(),
        start_coord: 1,
        end_coord: 100,
        new_scaffold: "newScaffold2".to_string(),
        orientation: "PLUS".to_string(),
    };
    let tpf3 = Tpf {
        ori_scaffold: "scaffold1".to_string(),
        start_coord: 1,
        end_coord: 100,
        new_scaffold: "newScaffold1".to_string(),
        orientation: "PLUS".to_string(),
    };
    let tpfs = vec![tpf1, tpf2, tpf3];
    let fasta = (&"scaffold1".to_string(), &(1 as usize));
    let result = subset_vec_tpf(&tpfs, fasta);
    assert_eq!(result.len(), 2);
}

#[test]
fn check_parse_seq() {
    let sequence =
        Sequence::from(b"AATGGCCGGCGCGTTAAACCCAATGCCCCGGTTAANNGCTCGTCGCTTGCTTCGCAAAA".to_vec());
    let tpf1 = Tpf {
        ori_scaffold: "scaffold1".to_string(),
        start_coord: 3,
        end_coord: 5,
        new_scaffold: "newScaffold1".to_string(),
        orientation: "PLUS".to_string(),
    };
    let tpf2 = Tpf {
        ori_scaffold: "scaffold2".to_string(),
        start_coord: 10,
        end_coord: 20,
        new_scaffold: "newScaffold2".to_string(),
        orientation: "MINUS".to_string(),
    };
    let tpf3 = Tpf {
        ori_scaffold: "scaffold1".to_string(),
        start_coord: 1,
        end_coord: 58,
        new_scaffold: "newScaffold1".to_string(),
        orientation: "PLUS".to_string(),
    };

    let tpfs = vec![&tpf1, &tpf2, &tpf3];
    let input_sequence = Some(sequence);

    let new_fasta = parse_seq(input_sequence, tpfs);

    assert_eq!(new_fasta.len(), 3);
    assert_eq!(new_fasta.first().unwrap().sequence, "TGG");
    assert_eq!(new_fasta.get(1).unwrap().sequence, "GGTTTAACGCG");
    assert_eq!(
        new_fasta.get(2).unwrap().sequence,
        "AATGGCCGGCGCGTTAAACCCAATGCCCCGGTTAANNGCTCGTCGCTTGCTTCGCAAA"
    );
}

// This should panic with a end_coord > sequence.length
// Should the exception be handled in a more graceful way?
#[test]
#[should_panic]
fn check_parse_seq_bounds_error() {
    let sequence =
        Sequence::from(b"AATGGCCGGCGCGTTAAACCCAATGCCCCGGTTAANNGCTCGTCGCTTGCTTCGCAAAA".to_vec());
    let tpf = Tpf {
        ori_scaffold: "scaffold1".to_string(),
        start_coord: 10,
        end_coord: 60,
        new_scaffold: "newScaffold1".to_string(),
        orientation: "PLUS".to_string(),
    };
    let tpfs = vec![&tpf];

    let input_sequence = Some(sequence);

    parse_seq(input_sequence, tpfs);
}

#[test]
fn check_parse_tpf() {
    let path = "test_data/iyAndFlav1/full/iyAndFlav1.curated_subset.tpf".to_string();
    let tpfs = parse_tpf(&path);
    assert_eq!(tpfs.len(), 4);

    // ?	SCAFFOLD_12:1-900734	RL_3	MINUS
    // GAP	TYPE-2	200
    // ?	SCAFFOLD_50:1-61000	RL_3	PLUS
    // ?	SCAFFOLD_26:1-201195	RL_3_unloc_1	PLUS
    // ?	SCAFFOLD_84:1-2000	SCAFFOLD_84	PLUS

    let tpf1 = tpfs.first().unwrap();
    assert_eq!(tpf1.ori_scaffold, "SCAFFOLD_12".to_string());
    assert_eq!(tpf1.start_coord, 1);
    assert_eq!(tpf1.end_coord, 900734);
    assert_eq!(tpf1.new_scaffold, "SUPER_3".to_string());
    assert_eq!(tpf1.orientation, "MINUS".to_string());

    let tpf2 = tpfs.last().unwrap();
    assert_eq!(tpf2.ori_scaffold, "SCAFFOLD_84".to_string());
    assert_eq!(tpf2.start_coord, 1);
    assert_eq!(tpf2.end_coord, 2000);
    assert_eq!(tpf2.new_scaffold, "SCAFFOLD_84".to_string());
    assert_eq!(tpf2.orientation, "PLUS".to_string());
}

#[test]
fn check_save_to_fasta() {
    // Inputs: Vector of NewFasta types, vector of Tpf types, output path, and n_length
    // 1. Creates a data file based on the output path, and open the created file using OpenOption
    // 2. Creates a debug.txt file, and open that file.
    // 3. Retrieving unique scaffolds based on the initial tpf types

    // Iterating over the unique scaffold names:
    // - appends a > symbol to the start and a new line to the end
    // - appends the scaffold name to the file
    // - appends the scaffold name to file2 ()debug.txt)
    // - creates a struct called MyRecord with an empty name and sequence
    // - assigns the unique scaffold name to data name
    // - iterating over the tpf data (comes from parse_tpf function)
    // - if the new scaffold name is equal to the unique scaffold name
    // - iterates over the new_fasta data
    // - checking for object equality
    // - if the object is equal it formats the tpf into a string and writes it to file2 (debug.txt)
    // - if the object is equal it appends the fasta sequence to the data sequence
    // - creates a variable line_len set to 60
    // - creates a fixed variable which is is the sequence
    // - creates a n_string variable which is N repeated n_length times
    // - creates fixed2 variable which is fixed joined with n_string
    // - creates a variable called fixed3 which is converted to bytes and chunks it by line_len and converts it to a vector of strings
    // - iterates over the fixed3 variable and writes it to the file

    let new_fasta_items = vec![
        NewFasta {
            tpf: Tpf {
                ori_scaffold: "SCAFFOLD_1".to_string(),
                start_coord: 1,
                end_coord: 9,
                new_scaffold: "SUPER_1".to_string(),
                orientation: "MINUS".to_string(),
            },
            sequence: "GGCATGCAT".to_string(),
        },
        NewFasta {
            tpf: Tpf {
                ori_scaffold: "SCAFFOLD_3".to_string(),
                start_coord: 1,
                end_coord: 5,
                new_scaffold: "SUPER_2".to_string(),
                orientation: "PLUS".to_string(),
            },
            sequence: "AGTGT".to_string(),
        },
    ];

    let tpf_items = vec![
        Tpf {
            ori_scaffold: "SCAFFOLD_1".to_string(),
            start_coord: 1,
            end_coord: 9,
            new_scaffold: "SUPER_1".to_string(),
            orientation: "MINUS".to_string(),
        },
        Tpf {
            ori_scaffold: "SCAFFOLD_3".to_string(),
            start_coord: 1,
            end_coord: 5,
            new_scaffold: "SUPER_2".to_string(),
            orientation: "PLUS".to_string(),
        },
    ];

    let output = &"new.fasta".to_string();

    let n_length: usize = 200;

    save_to_fasta(new_fasta_items, tpf_items, output, n_length);

    assert!(
        are_files_identical(output, "test_data/iyAndFlav1/tiny/tiny_test.output.fasta").unwrap()
    );

    assert!(
        are_files_identical("debug.txt", "test_data/iyAndFlav1/tiny/tiny_test.debug.txt").unwrap()
    );

    match fs::remove_file(output) {
        Ok(_) => true,
        Err(_err) => panic!("File cannot be found!"),
    };
    match fs::remove_file("debug.txt") {
        Ok(_) => true,
        Err(_err) => panic!("File cannot be found!"),
    };
}

//#[ignore = "Work in Progress (WIP)"]
#[test]
fn check_curate_fasta() {
    let mut cmd = Command::cargo_bin("fasta_manipulation").unwrap();

    // Create temp directory that will get cleaned up
    let dir = Builder::new().prefix("local_tests").tempdir().unwrap();

    // Generate paths for mock files
    let fasta_path = &dir.path().join("input_fasta.fa");
    let fai_path = &dir.path().join("input_fasta.fa.fai");
    let tpf_path = &dir.path().join("input.tpf");

    // Actually generate the mock files
    let mut fasta = File::create(fasta_path).unwrap();
    let mut fai = File::create(fai_path).unwrap();
    let mut tpf = File::create(tpf_path).unwrap();

    let output = "./output.fa";

    write!(
        fai,
        "SCAFFOLD_1\t16\t12\t16\t17\nSCAFFOLD_3\t16\t41\t16\t17"
    )
    .unwrap();

    write!(
        fasta,
        ">SCAFFOLD_1\nATGCATGCCGTATAGA\n>SCAFFOLD_3\nAGTGTATTTTTATGCA"
    )
    .unwrap();

    write!(
        tpf,
        "?\tSCAFFOLD_1:1-9\tRL_1\tMINUS\nGAP\tTYPE-2\t200\n?\tSCAFFOLD_3:1-5\tRL_2\tPLUS"
    )
    .unwrap();

    cmd.arg("curate")
        .arg("-f")
        .arg(fasta_path)
        .arg("-t")
        .arg(tpf_path)
        .arg("-o")
        .arg(output)
        .assert()
        .success();
}
