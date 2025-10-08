use nekosearch::components::prelude::*;

#[test]
fn test_identical() {
    let lev = Levenshtein::new();
    assert_eq!(lev.calc("rust".into(), "rust".into()), 100);
}

#[test]
fn test_one_substitution() {
    let lev = Levenshtein::new();
    assert_eq!(lev.calc("kitten".into(), "sitten".into()), 83);
}

#[test]
fn test_two_ops() {
    let lev = Levenshtein::new();
    assert_eq!(lev.calc("saturday".into(), "sunday".into()), 63);
}

#[test]
fn test_insert_delete() {
    let lev = Levenshtein::new();
    assert_eq!(lev.calc("abc".into(), "abcd".into()), 75);
}

#[test]
fn test_empty() {
    let lev = Levenshtein::new();
    assert_eq!(lev.calc("".into(), "".into()), 100);
    assert_eq!(lev.calc("test".into(), "".into()), 0);
}
