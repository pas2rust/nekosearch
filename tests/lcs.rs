use nekosearch::components::prelude::*;

fn run_test(s1: &str, s2: &str) -> f32 {
    let algo = Lcs::new().weight(1.0);
    algo.calc(s1.to_string(), s2.to_string())
}

#[test]
fn test_lcs_perfect_match() {
    assert_eq!(run_test("banana", "banana"), 1.0);
}

#[test]
fn test_lcs_common_subsequence() {
    let score = run_test("GATTACA", "GCATAG");
    assert!((score - 0.5714286).abs() < 1e-6);
}

#[test]
fn test_lcs_insertion_or_deletion() {
    let score = run_test("kitten", "sitting");
    assert!((score - 0.5714286).abs() < 1e-6);
}

#[test]
fn test_lcs_transposed_letters() {
    assert_eq!(run_test("ca", "ac"), 0.5);
}

#[test]
fn test_lcs_no_match() {
    assert_eq!(run_test("abc", "xyz"), 0.0);
}

#[test]
fn test_lcs_one_empty_string() {
    assert_eq!(run_test("", "abc"), 0.0);
}
