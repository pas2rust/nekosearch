use nekosearch::components::prelude::*;

const EPSILON: f64 = 0.000001;

#[test]
fn test_perfect_match() {
    let lev = Levenshtein::new();
    let similarity = lev.calc("rust", "rust");
    assert!((similarity - 0.0).abs() < EPSILON);
}

#[test]
fn test_simple_substitution() {
    let lev = Levenshtein::new();
    let similarity = lev.calc("kitten", "sitten");
    assert!((similarity - 1.0).abs() < EPSILON);
}

#[test]
fn test_multiple_operations() {
    let lev = Levenshtein::new();
    let similarity = lev.calc("saturday", "sunday");
    assert!((similarity - 3.0).abs() < EPSILON);
}

#[test]
fn test_empty_strings() {
    let lev = Levenshtein::new();
    assert!((lev.calc("", "test") - 4.0).abs() < EPSILON);
    assert!((lev.calc("test", "") - 4.0).abs() < EPSILON);
    assert!((lev.calc("", "") - 0.0).abs() < EPSILON);
}

#[test]
fn test_complex_case() {
    let lev = Levenshtein::new();
    let similarity = lev.calc("programming", "programing");
    assert!((similarity - 1.0).abs() < EPSILON);
}
