use nekosearch::components::prelude::*;

const EPSILON: f64 = 0.001;

fn build_neko(txt: &str, term: &str) -> NekoSearch {
    NekoSearch::new().txt(txt).term(term)
}

#[test]
fn test_find_perfect_match() {
    let neko = build_neko("Rust", "Rust");
    let result = neko.find();
    assert!((result.score - 1.0).abs() < EPSILON);
}

#[test]
fn test_find_typo() {
    let neko = build_neko("Python", "Piton");
    let result = neko.find();
    println!("{result:#?}");
    assert!(result.score > 0.6);
}

#[test]
fn test_find_transposed_letters() {
    let neko = build_neko("martha", "marhta");
    let result = neko.find();
    assert!(result.score > 0.6);
}

#[test]
fn test_find_common_prefix() {
    let neko = build_neko("Rustacean", "Rust");
    let result = neko.find();
    assert!(result.score > 0.6);
}

#[test]
fn test_find_no_match() {
    let neko = build_neko("Rust", "C++");
    let result = neko.find();
    assert!(result.score < 0.2);
}

#[test]
fn test_find_different_lengths() {
    let neko = build_neko("development", "dev");
    let result = neko.find();
    assert!(result.score > 0.4 && result.score < 0.7);
}

#[test]
fn test_custom_weights() {
    let neko_low_lev = NekoSearch::new().txt("martha").term("marhta").flow(vec![
        Jaro::new().weight(0.9).to_box(),
        Levenshtein::new().weight(0.1).to_box(),
    ]);

    let result_low_lev = neko_low_lev.find();

    let neko_high_lev = NekoSearch::new().txt("martha").term("marhta").flow(vec![
        Jaro::new().weight(0.1).to_box(),
        Levenshtein::new().weight(0.9).to_box(),
    ]);

    let result_high_lev = neko_high_lev.find();

    assert!((result_low_lev.score - result_high_lev.score).abs() > EPSILON);
}

#[test]
fn test_filter_method() {
    let txt = "Rust";
    let terms = ["Rost", "Java", "Python", "Rust"];
    let results: Vec<_> = terms
        .iter()
        .map(|term| build_neko(txt, term).find())
        .collect();

    assert_eq!(results.len(), 4);

    println!("results: {results:#?}");

    let perfect_match = results.iter().find(|r| r.term == "Rust").unwrap();
    assert!((perfect_match.score - 1.0).abs() < EPSILON);

    let typo = results.iter().find(|r| r.term == "Rost").unwrap();
    assert!(typo.score > 0.6);

    let non_match = results.iter().find(|r| r.term == "Java").unwrap();
    assert!(non_match.score < 0.2);
}

#[test]
fn test_find_perfect_match_sentence() {
    let neko = build_neko("The quick brown fox", "The quick brown fox");
    let result = neko.find();
    assert!((result.score - 1.0).abs() < EPSILON);
}

#[test]
fn test_find_typo_in_sentence() {
    let neko = build_neko("The quick brown fox", "The quikc brown fx");
    let result = neko.find();
    println!("{result:#?}");
    assert!(result.score > 0.6);
}

#[test]
fn test_find_added_word() {
    let neko = build_neko("The quick brown fox", "The quick brown fox jumps");
    let result = neko.find();
    assert!(result.score > 0.5 && result.score < 1.0);
}

#[test]
fn test_find_removed_word() {
    let neko = build_neko("The quick brown fox jumps", "The quick brown fox");
    let result = neko.find();
    assert!(result.score > 0.5 && result.score < 1.0);
}

#[test]
fn test_find_transposed_words() {
    let neko = build_neko("The brown quick fox", "The quick brown fox");
    let result = neko.find();
    assert!(result.score > 0.5);
}
