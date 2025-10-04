# nekosearch

[![Crates.io](https://img.shields.io/crates/v/nekosearch.svg)](https://crates.io/crates/nekosearch)
[![Docs.rs](https://docs.rs/nekosearch/badge.svg)](https://docs.rs/nekosearch)
[![License](https://img.shields.io/crates/l/nekosearch.svg)](https://github.com/pas2rust/nekosearch/blob/main/LICENSE)
![GitHub top language](https://img.shields.io/github/languages/top/pas2rust/nekosearch?color=orange&logo=rust&style=flat&logoColor=white)
![GitHub stars](https://img.shields.io/github/stars/pas2rust/nekosearch?color=success&style=flat&logo=github)
![GitHub forks](https://img.shields.io/github/forks/pas2rust/nekosearch?color=orange&logo=Furry%20Network&style=flat&logoColor=white)
![Tests](https://raw.githubusercontent.com/pas2rust/badges/main/nekosearch-tests.svg)
![Crates.io downloads](https://img.shields.io/crates/d/nekosearch.svg)
![GitHub last commit](https://img.shields.io/github/last-commit/pas2rust/nekosearch?color=ff69b4&label=update&logo=git&style=flat&logoColor=white)

**`NekoSearch`**  is a small, focused fuzzy string comparator designed to produce a single, explainable similarity score from a weighted combination of multiple metrics. It’s useful for typo-tolerant matching, fuzzy lookup, autocomplete ranking, and any scenario where you want a compact similarity result plus the breakdown of contributing signals.


## ⚙️ Installation

Enable only the features you need:

```bash
cargo add nekosearch
```

## 🚀 Usage 

```rust
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
fn test_find_perfect_match_sentence_en() {
    let neko = build_neko("The quick brown fox", "The quick brown fox");
    let result = neko.find();
    assert!((result.score - 1.0).abs() < EPSILON);
}

#[test]
fn test_find_typo_in_sentence_en() {
    let neko = build_neko("The quick brown fox", "The quikc brown fx");
    let result = neko.find();
    println!("{result:#?}");
    assert!(result.score > 0.6);
}

#[test]
fn test_find_added_word_en() {
    let neko = build_neko("The quick brown fox", "The quick brown fox jumps");
    let result = neko.find();
    assert!(result.score > 0.5 && result.score < 1.0);
}

#[test]
fn test_find_removed_word_en() {
    let neko = build_neko("The quick brown fox jumps", "The quick brown fox");
    let result = neko.find();
    assert!(result.score > 0.5 && result.score < 1.0);
}

#[test]
fn test_find_transposed_words_en() {
    let neko = build_neko("The brown quick fox", "The quick brown fox");
    let result = neko.find();
    assert!(result.score > 0.5);
}

```

---

<h2 align="center">
  <strong>❤️ Donate</strong>
</h2>

<p align="center">
  <a href="https://github.com/pas2rust/pas2rust/blob/main/pas-monero-donate.png" style="text-decoration:none; color:inherit;">
    <img src="https://img.shields.io/badge/Monero%20QR-FF6600?style=flat&logo=monero&logoColor=white" alt="Monero QR"/>
  </a>
  <a href="https://github.com/pas2rust/pas2rust/blob/main/pas-bitcoin-donate.png" style="text-decoration:none; color:inherit;">
    <img src="https://img.shields.io/badge/BTC%20QR-EAB300?style=flat&logo=bitcoin&logoColor=white" alt="BTC QR"/>
  </a>
  <a href="https://revolut.me/pas2rust" style="text-decoration:none; color:inherit;">
    <img src="https://img.shields.io/badge/Revolut%20QR-Blue?style=flat&logo=revolut&logoColor=white" alt="Revolut QR"/>
  </a>
  <a href="https://wise.com/pay/me/pedroaugustos99" style="text-decoration:none; color:inherit;">
    <img src="https://img.shields.io/badge/Wise%20QR-1CA0F2?style=flat&logo=wise&logoColor=white" alt="Wise QR"/>
  </a>
</p>


---