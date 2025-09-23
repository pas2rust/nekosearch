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

const EPSILON: f64 = 0.00001;

#[test]
fn test_find_perfect_match() {
    let neko = NekoSearch::new().target("Rust".to_string());

    let result = neko.find("Rust");
    assert!((result.score - 1.0).abs() < EPSILON);
}

#[test]
fn test_custom_weights() {
    let neko = NekoSearch::new()
        .target("martha".to_string())
        .lev_weight(0.1)
        .jaro_winkler_weight(0.8)
        .n_gram_weight(0.1);

    let result_low_lev = neko.find("marhta");

    let neko_high_lev = NekoSearch::new()
        .target("martha".to_string())
        .lev_weight(0.8)
        .jaro_winkler_weight(0.1)
        .n_gram_weight(0.1);

    let result_high_lev = neko_high_lev.find("marhta");

    assert!(result_low_lev.score > result_high_lev.score);
}

#[test]
fn test_filter_method() {
    let neko = NekoSearch::new().target("Rust".to_string());
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

```

# ❤️ Donate

[![Monero](https://img.shields.io/badge/88NKLkhZf1nTVpaSU6vwG6dwBwb9tFVSM8Lpj3YqdL1PMt8Gm7opV7aUnMYBaAC9Y6a4kfDc3fLGoMVqeSJKNphyLpLdEvC-FF6600?style=flat&logo=monero&logoColor=white)](https://github.com/pas2rust/pas2rust/blob/main/pas-monero-donate.png)
[![Bitcoin](https://img.shields.io/badge/bc1qnlayyh84e9u5pd4m9g9sf4c5zdzswvkmudmdu5-EAB300?style=flat&logo=bitcoin&logoColor=white)](https://github.com/pas2rust/pas2rust/blob/main/pas-bitcoin-donate.png)