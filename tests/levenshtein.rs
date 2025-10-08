use nekosearch::components::prelude::*;

#[test]
fn test_identical() {
    let lev = Levenshtein::new();
    assert_eq!(lev.calc("rust".into(), "rust".into()), 1.0);
}

#[test]
fn test_one_substitution() {
    let lev = Levenshtein::new();
    assert!(lev.calc("kitten".into(), "sitten".into()) >= 0.8);
}

#[test]
fn test_two_ops() {
    let lev = Levenshtein::new();
    assert!(lev.calc("saturday".into(), "sunday".into()) >= 0.62);
}

#[test]
fn test_insert_delete() {
    let lev = Levenshtein::new();
    assert_eq!(lev.calc("abc".into(), "abcd".into()), 0.75);
}

#[test]
fn test_empty() {
    let lev = Levenshtein::new();
    assert_eq!(lev.calc("".into(), "".into()), 1.0);
    assert_eq!(lev.calc("test".into(), "".into()), 0.0);
}
