use nekosearch::components::prelude::*;

#[test]
fn test_identical_strings() {
    let cosine = Cosine::new();
    let s1 = "martha";
    let s2 = "martha";
    let similarity = cosine.calc(s1.into(), s2.into());
    assert_eq!(similarity, 1.0);
}

#[test]
fn test_transposed_letters() {
    let cosine = Cosine::new();
    let s1 = "martha";
    let s2 = "marhta";
    let similarity = cosine.calc(s1.into(), s2.into());
    assert!(similarity > 0.3, "similarity = {}", similarity);
}

#[test]
fn test_small_difference() {
    let cosine = Cosine::new();
    let s1 = "martha";
    let s2 = "marsha";
    let similarity = cosine.calc(s1.into(), s2.into());
    assert!(
        similarity > 0.35 && similarity < 1.0,
        "similarity = {}",
        similarity
    );
}

#[test]
fn test_insertion_or_deletion() {
    let cosine = Cosine::new();
    let s1 = "martha";
    let s2 = "marthas";
    let similarity = cosine.calc(s1.into(), s2.into());
    assert!(
        similarity < 1.0 && similarity > 0.8,
        "similarity = {}",
        similarity
    );
}

#[test]
fn test_completely_different() {
    let cosine = Cosine::new();
    let s1 = "hello";
    let s2 = "world";
    let similarity = cosine.calc(s1.into(), s2.into());
    assert!(similarity < 0.4, "similarity = {}", similarity);
}

#[test]
fn test_empty_strings() {
    let cosine = Cosine::new();
    let similarity = cosine.calc("".into(), "".into());
    assert_eq!(similarity, 0.0);
}

#[test]
fn test_one_empty_string() {
    let cosine = Cosine::new();
    let similarity = cosine.calc("abc".into(), "".into());
    assert_eq!(similarity, 0.0);
}

#[test]
fn test_long_strings() {
    let cosine = Cosine::new();
    let s1 = "the quick brown fox jumps over the lazy dog";
    let s2 = "the quick brown fox jumped over a lazy dog";
    let similarity = cosine.calc(s1.into(), s2.into());
    assert!(
        similarity > 0.8 && similarity < 1.0,
        "similarity = {}",
        similarity
    );
}
