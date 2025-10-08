use nekosearch::components::prelude::*;

fn build_neko(txt: &str, term: &str) -> NekoSearch {
    let neko = NekoSearch::new().txt(txt).term(term);
    let flow = &neko.flow;
    println!("{flow:#?}");
    neko
}

#[test]
fn test_find_perfect_match() {
    let mut neko = build_neko("Rust", "Rust");
    let result = neko.find();
    assert_eq!(result, 100);
}

#[test]
fn test_find_typo() {
    let mut neko = build_neko("Python", "Piton");
    let result = neko.find();
    assert!(result >= 50);
}

#[test]
fn test_find_transposed_letters() {
    let mut neko = build_neko("martha", "marhta");
    let result = neko.find();
    assert!(result > 60);
}

#[test]
fn test_find_common_prefix() {
    let mut neko = build_neko("Rustacean", "Rust");
    let result = neko.find();
    assert!(result > 50);
}

#[test]
fn test_find_no_match() {
    let mut neko = build_neko("Rust", "C++");
    let result = neko.find();
    assert!(result < 20);
}

#[test]
fn test_find_different_lengths() {
    let mut neko = build_neko("development", "dev");
    let result = neko.find();
    assert!((40..70).contains(&result));
}

#[test]
fn test_custom_weights() {
    let mut neko_low_lev = NekoSearch::new().txt("martha").term("marhta").flow(vec![
        Jaro::new().weight(90).to_box(),
        Levenshtein::new().weight(10).to_box(),
    ]);

    let result_low_lev = neko_low_lev.find();

    let mut neko_high_lev = NekoSearch::new().txt("martha").term("marhta").flow(vec![
        Jaro::new().weight(10).to_box(),
        Levenshtein::new().weight(90).to_box(),
    ]);

    let result_high_lev = neko_high_lev.find();

    assert_ne!(result_low_lev, result_high_lev);
}

#[test]
fn test_filter_method() {
    let txt = "Rust";
    let terms = ["Rost", "Java", "Python", "Rust"];

    for term in terms {
        let mut neko = NekoSearch::new().txt(txt).term(term).build().unwrap();
        let score = neko.find();

        if term == "Rust" {
            assert_eq!(score, 100);
        }

        if term == "Rost" {
            assert!(score > 55, "similarity = {}", score);
        }

        if term == "Java" {
            assert!(score < 20, "similarity = {}", score);
        }
    }
}

#[test]
fn test_find_perfect_match_sentence() {
    let mut neko = build_neko("The quick brown fox", "The quick brown fox");
    let result = neko.find();
    assert_eq!(result, 100);
}

#[test]
fn test_find_typo_in_sentence() {
    let mut neko = build_neko("The quick brown fox", "The quikc brown fx");
    let result = neko.find();
    assert!(result >= 60);
}

#[test]
fn test_find_added_word() {
    let mut neko = build_neko("The quick brown fox", "The quick brown fox jumps");
    let result = neko.find();
    assert!(result > 50 && result < 100);
}

#[test]
fn test_find_removed_word() {
    let mut neko = build_neko("The quick brown fox jumps", "The quick brown fox");
    let result = neko.find();
    assert!(result > 50 && result < 100);
}

#[test]
fn test_find_transposed_words() {
    let mut neko = build_neko("The brown quick fox", "The quick brown fox");
    let result = neko.find();
    assert!(result > 50);
}
