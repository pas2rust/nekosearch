use nekosearch::components::prelude::*;

const EPSILON: f64 = 0.00001;

#[test]
fn test_find_perfect_match() {
    let neko = NekoSearch::new().target("Rust");

    let result = neko.find("Rust");
    assert!((result.score - 1.0).abs() < EPSILON);
}

#[test]
fn test_find_typo() {
    let neko = NekoSearch::new().target("Python");

    let result = neko.find("Piton");
    assert!(result.score > 0.6);
}

#[test]
fn test_find_transposed_letters() {
    let neko = NekoSearch::new().target("martha");

    let result = neko.find("marhta");
    assert!(result.score > 0.6);
}

#[test]
fn test_find_common_prefix() {
    let neko = NekoSearch::new().target("Rustacean");

    let result = neko.find("Rust");
    assert!(result.score > 0.6);
}

#[test]
fn test_find_no_match() {
    let neko = NekoSearch::new().target("Rust");

    let result = neko.find("C++");
    assert!((result.score - 0.0) < 0.2);
}

#[test]
fn test_find_different_lengths() {
    let neko = NekoSearch::new().target("development");

    let result = neko.find("dev");
    assert!(result.score > 0.4 && result.score < 0.7);
}

#[test]
fn test_custom_weights() {
    let neko = NekoSearch::new()
        .target("martha")
        .lev_weight(0.1)
        .jaro_winkler_weight(0.8)
        .n_gram_weight(0.1);

    let result_low_lev = neko.find("marhta");

    let neko_high_lev = NekoSearch::new()
        .target("martha")
        .lev_weight(0.8)
        .jaro_winkler_weight(0.1)
        .n_gram_weight(0.1);

    let result_high_lev = neko_high_lev.find("marhta");

    assert!(result_low_lev.score > result_high_lev.score);
}

#[test]
fn test_filter_method() {
    let neko = NekoSearch::new().target("Rust");
    let terms = vec!["Rost", "Java", "Python", "Rust"];
    let results = neko.filter(terms);
    assert_eq!(results.len(), 4);
    let perfect_match = results.iter().find(|r| r.term == "Rust").unwrap();
    assert!((perfect_match.score - 1.0).abs() < EPSILON);
    let typo = results.iter().find(|r| r.term == "Rost").unwrap();
    println!("typo: {:#?}", typo);
    assert!(typo.score > 0.6);
    assert!(typo.jaro_score > 0.8);
    assert!(typo.lev_score > 0.6);
    let non_match = results.iter().find(|r| r.term == "Java").unwrap();
    assert!(non_match.score < 0.2);
}

#[test]
fn test_filter_with_custom_weights() {
    let neko = NekoSearch::new()
        .target("martha")
        .lev_weight(0.1)
        .jaro_winkler_weight(0.8)
        .n_gram_weight(0.1);
    let terms = vec!["marhta", "martha"];
    let results = neko.filter(terms);
    let perfect_match = results.iter().find(|r| r.term == "martha").unwrap();
    assert!((perfect_match.score - 1.0).abs() < EPSILON);
    let transposed = results.iter().find(|r| r.term == "marhta").unwrap();
    assert!(transposed.score > 0.8);
}
