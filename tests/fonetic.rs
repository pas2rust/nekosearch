use nekosearch::components::prelude::*;

fn run_test(s1: &str, s2: &str) -> f32 {
    let algo = Metaphone::new().weight(1.0);
    algo.calc(s1.to_string(), s2.to_string())
}

#[test]
fn test_metaphone_general_similarity() {
    assert_eq!(run_test("Fisher", "Fischer"), 1.0);
    assert_eq!(run_test("Smith", "Smyth"), 1.0);
    assert_eq!(run_test("Robert", "Rupert"), 1.0);
    assert_eq!(run_test("phoney", "fone"), 1.0);
}

#[test]
fn test_metaphone_difference() {
    assert_eq!(run_test("test", "text"), 0.0);
    assert_eq!(run_test("phone", "home"), 0.0);
    assert_eq!(run_test("apple", "orange"), 0.0);
}

#[test]
fn test_metaphone_misspellings() {
    assert_eq!(run_test("algorithm", "algoritim"), 1.0);
    assert_eq!(run_test("receive", "recieve"), 1.0);
}

#[test]
fn test_metaphone_perfect_match() {
    assert_eq!(run_test("symmetry", "symmetry"), 1.0);
}

#[test]
fn test_metaphone_no_match() {
    assert_eq!(run_test("qwert", "zxcvb"), 0.0);
}
