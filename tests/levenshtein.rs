use nekosearch::components::prelude::*;

#[test]
fn test_perfect_match() {
    assert_eq!(levenshtein_distance("rust", "rust"), 0);
}

#[test]
fn test_simple_substitution() {
    assert_eq!(levenshtein_distance("kitten", "sitten"), 1);
}

#[test]
fn test_multiple_operations() {
    assert_eq!(levenshtein_distance("saturday", "sunday"), 3);
}

#[test]
fn test_empty_strings() {
    assert_eq!(levenshtein_distance("", "test"), 4);
    assert_eq!(levenshtein_distance("test", ""), 4);
    assert_eq!(levenshtein_distance("", ""), 0);
}

#[test]
fn test_complex_case() {
    assert_eq!(levenshtein_distance("programming", "programing"), 1);
}
