use nekosearch::components::prelude::*;

const EPSILON: f64 = 0.000001;

#[test]
fn test_perfect_match() {
    let lev = Levenshtein::new();
    let similarity = lev.calc("rust".into(), "rust".into());
    let expected = 1.0;
    assert!((similarity - expected).abs() < EPSILON);
}

#[test]
fn test_simple_substitution() {
    let lev = Levenshtein::new();
    let similarity = lev.calc("kitten".into(), "sitten".into());
    let expected = 1.0 - (1.0 / 6.0);
    assert!((similarity - expected).abs() < EPSILON);
}

#[test]
fn test_multiple_operations() {
    let lev = Levenshtein::new();
    let similarity = lev.calc("saturday".into(), "sunday".into());
    let expected = 1.0 - (3.0 / 8.0);
    assert!((similarity - expected).abs() < EPSILON);
}

#[test]
fn test_empty_strings() {
    let lev = Levenshtein::new();
    let expected_full_diff = 0.0;
    let expected_perfect_match = 1.0;

    assert!((lev.calc("".into(), "test".into()) - expected_full_diff).abs() < EPSILON);
    assert!((lev.calc("test".into(), "".into()) - expected_full_diff).abs() < EPSILON);
    assert!((lev.calc("".into(), "".into()) - expected_perfect_match).abs() < EPSILON);
}

#[test]
fn test_complex_case() {
    let lev = Levenshtein::new();
    let similarity = lev.calc("programming".into(), "programing".into());
    let expected = 1.0 - (1.0 / 11.0);
    assert!((similarity - expected).abs() < EPSILON);
}
