use nekosearch::components::prelude::*;

#[test]
fn test_identical_strings() {
    let jaccard = Jaccard::new();
    let s1 = "martha";
    let s2 = "martha";
    let similarity = jaccard.calc(s1.into(), s2.into());
    assert_eq!(similarity, 100);
}

#[test]
fn test_transposed_letters() {
    let jaccard = Jaccard::new();
    let s1 = "martha";
    let s2 = "marhta";
    let similarity = jaccard.calc(s1.into(), s2.into());
    assert!(similarity < 50, "similarity = {}", similarity);
}

#[test]
fn test_small_difference() {
    let jaccard = Jaccard::new();
    let s1 = "martha";
    let s2 = "marsha";
    let similarity = jaccard.calc(s1.into(), s2.into());
    assert!(similarity < 50, "similarity = {}", similarity);
}

#[test]
fn test_insertion_or_deletion() {
    let jaccard = Jaccard::new();
    let s1 = "martha";
    let s2 = "marthas";
    let similarity = jaccard.calc(s1.into(), s2.into());
    assert!(
        similarity < 100 && similarity > 50,
        "similarity = {}",
        similarity
    );
}

#[test]
fn test_completely_different() {
    let jaccard = Jaccard::new();
    let s1 = "hello";
    let s2 = "world";
    let similarity = jaccard.calc(s1.into(), s2.into());
    assert!(similarity < 40, "similarity = {}", similarity);
}

#[test]
fn test_empty_strings() {
    let jaccard = Jaccard::new();
    let similarity = jaccard.calc("".into(), "".into());
    assert_eq!(similarity, 0);
}

#[test]
fn test_one_empty_string() {
    let jaccard = Jaccard::new();
    let similarity = jaccard.calc("abc".into(), "".into());
    assert_eq!(similarity, 0);
}

#[test]
fn test_long_strings() {
    let jaccard = Jaccard::new();
    let s1 = "the quick brown fox jumps over the lazy dog";
    let s2 = "the quick brown fox jumped over a lazy dog";
    let similarity = jaccard.calc(s1.into(), s2.into());
    assert!(
        similarity > 60 && similarity < 100,
        "similarity = {}",
        similarity
    );
}
