// pub use fasta_manipulation::tpf_fasta::*;
use fasta_manipulation::tpf_fasta_mod::{check_orientation, get_uniques, subset_vec_tpf, Tpf};
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
