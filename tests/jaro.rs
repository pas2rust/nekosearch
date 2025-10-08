use nekosearch::components::prelude::*;

#[test]
fn test_identical_strings() {
    let s1 = "martha";
    let s2 = "martha";
    let jaro = Jaro::new();
    let similarity = jaro.calc(s1.into(), s2.into());
    assert_eq!(similarity, 1.0f32);
}

#[test]
fn test_transposed_letters() {
    let s1 = "martha";
    let s2 = "marhta";
    let jaro = Jaro::new();
    let similarity = jaro.calc(s1.into(), s2.into());

    let expected = 0.961_f32;
    let eps = 0.01_f32;
    assert!(
        (similarity - expected).abs() <= eps,
        "similarity = {}, expected ≈ {} (±{})",
        similarity,
        expected,
        eps
    );
}

#[test]
fn test_different_lengths_with_prefix() {
    let s1 = "dixon";
    let s2 = "dicksonx";
    let jaro = Jaro::new().chars(3_usize);
    let similarity = jaro.calc(s1.into(), s2.into());

    assert!(
        similarity > 0.0 && similarity <= 1.0,
        "similarity = {}",
        similarity
    );
}

#[test]
fn test_no_match() {
    let s1 = "test";
    let s2 = "abc";
    let jaro = Jaro::new();
    let similarity = jaro.calc(s1.into(), s2.into());
    assert_eq!(similarity, 0.0f32);
}
