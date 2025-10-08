use nekosearch::components::prelude::*;

#[test]
fn test_identical_strings() {
    let dl = DamerauLevenshtein::new();
    let s1 = "martha";
    let s2 = "martha";
    let similarity = dl.calc(s1.into(), s2.into());
    assert_eq!(similarity, 100);
}

#[test]
fn test_transposed_letters() {
    let dl = DamerauLevenshtein::new();
    let s1 = "martha";
    let s2 = "marhta"; // transposição 'h' e 't'
    let similarity = dl.calc(s1.into(), s2.into());
    assert!(similarity > 80, "similarity = {}", similarity);
}

#[test]
fn test_one_substitution() {
    let dl = DamerauLevenshtein::new();
    let s1 = "martha";
    let s2 = "marsha"; // 't' → 's'
    let similarity = dl.calc(s1.into(), s2.into());
    assert!(
        similarity > 80 && similarity < 100,
        "similarity = {}",
        similarity
    );
}

#[test]
fn test_insertion_and_deletion() {
    let dl = DamerauLevenshtein::new();
    let s1 = "martha";
    let s2 = "marthas"; // inserção de 's'
    let similarity = dl.calc(s1.into(), s2.into());
    assert!(
        similarity < 100 && similarity > 80,
        "similarity = {}",
        similarity
    );
}

#[test]
fn test_completely_different() {
    let dl = DamerauLevenshtein::new();
    let s1 = "test";
    let s2 = "abcd";
    let similarity = dl.calc(s1.into(), s2.into());
    assert!(similarity < 40, "similarity = {}", similarity);
}

#[test]
fn test_empty_strings() {
    let dl = DamerauLevenshtein::new();
    let similarity = dl.calc("".into(), "".into());
    assert_eq!(similarity, 100);
}

#[test]
fn test_one_empty_string() {
    let dl = DamerauLevenshtein::new();
    let similarity = dl.calc("abc".into(), "".into());
    assert_eq!(similarity, 0);
}
