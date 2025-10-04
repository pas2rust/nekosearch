use nekosearch::components::prelude::*;

const EPSILON: f64 = 0.000001;

#[test]
fn test_identical_strings() {
    let s1 = "martha";
    let s2 = "martha";
    let jaro = Jaro::new();
    let similarity = jaro.calc(s1.into(), s2.into());
    assert!((similarity - 1.0).abs() < EPSILON);
}

#[test]
fn test_transposed_letters() {
    let s1 = "martha";
    let s2 = "marhta";
    let jaro = Jaro::new();
    let similarity = jaro.calc(s1.into(), s2.into());
    assert!((similarity - 0.961).abs() < 0.001);
}

#[test]
fn test_different_lengths_with_prefix() {
    let s1 = "dixon";
    let s2 = "dicksonx";
    let jaro = Jaro::new();
    let similarity = jaro.calc(s1.into(), s2.into());
    assert!((similarity - 0.767).abs() < 0.05);
}

#[test]
fn test_no_match() {
    let s1 = "test";
    let s2 = "abc";
    let jaro = Jaro::new();
    let similarity = jaro.calc(s1.into(), s2.into());
    assert!((similarity - 0.0).abs() < EPSILON);
}
