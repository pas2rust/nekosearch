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