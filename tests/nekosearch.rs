use nekosearch::components::prelude::*;

const EPSILON: f64 = 0.00001;

fn build_neko(txt: &str, term: &str) -> NekoSearch {
    NekoSearch::new()
        .txt(txt)
        .term(term)
        .jaro(Jaro::new().weight(0.8))
        .jaccard(Jaccard::new().weight(0.2))
        .levenshtein(Levenshtein::new().weight(0.5))
}

#[test]
fn test_find_perfect_match() {
    let neko = build_neko("Rust", "Rust");
    let result = neko.find(3);
    assert!((result.score - 1.0).abs() < EPSILON);
}

#[test]
fn test_find_typo() {
    let neko = build_neko("Python", "Piton");
    let result = neko.find(3);
    assert!(result.score > 0.6);
}

#[test]
fn test_find_transposed_letters() {
    let neko = build_neko("martha", "marhta");
    let result = neko.find(3);
    assert!(result.score > 0.6);
}

#[test]
fn test_find_common_prefix() {
    let neko = build_neko("Rustacean", "Rust");
    let result = neko.find(3);
    assert!(result.score > 0.6);
}

#[test]
fn test_find_no_match() {
    let neko = build_neko("Rust", "C++");
    let result = neko.find(3);
    assert!(result.score < 0.2);
}

#[test]
fn test_find_different_lengths() {
    let neko = build_neko("development", "dev");
    let result = neko.find(3);
    assert!(result.score > 0.4 && result.score < 0.7);
}

#[test]
fn test_custom_weights() {
    let neko_low_lev = NekoSearch::new()
        .txt("martha")
        .term("marhta")
        .levenshtein(Levenshtein::new().weight(0.1))
        .jaro(Jaro::new().weight(0.8))
        .jaccard(Jaccard::new().weight(0.1));

    let result_low_lev = neko_low_lev.find(3);

    let neko_high_lev = NekoSearch::new()
        .txt("martha")
        .term("marhta")
        .levenshtein(Levenshtein::new().weight(0.8))
        .jaro(Jaro::new().weight(0.1))
        .jaccard(Jaccard::new().weight(0.1));

    let result_high_lev = neko_high_lev.find(3);

    assert!(result_low_lev.score > result_high_lev.score);
}

#[test]
fn test_filter_method() {
    let txt = "Rust";
    let terms = vec!["Rost", "Java", "Python", "Rust"];
    let results: Vec<_> = terms
        .iter()
        .map(|term| build_neko(txt, term).find(3))
        .collect();

    assert_eq!(results.len(), 4);

    let perfect_match = results.iter().find(|r| r.term == "Rust").unwrap();
    assert!((perfect_match.score - 1.0).abs() < EPSILON);

    let typo = results.iter().find(|r| r.term == "Rost").unwrap();
    assert!(typo.score > 0.6);
    assert!(typo.jaro_score > 0.8);
    assert!(typo.lev_score > 0.6);

    let non_match = results.iter().find(|r| r.term == "Java").unwrap();
    assert!(non_match.score < 0.2);
}
