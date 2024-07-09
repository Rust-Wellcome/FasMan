// pub use fasta_manipulation::tpf_fasta::*;
use fasta_manipulation::tpf_fasta_mod::{
    check_orientation, get_uniques, parse_seq, parse_tpf, subset_vec_tpf, Tpf,
};

use noodles::fasta::record::Sequence;

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
    let path = "test_data/iyAndFlav1/iyAndFlav1.curated_subset.tpf".to_string();
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

#[ignore = "Work in progress (Still figuring out what it does)"]
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
    assert!(true);
}
