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
    let mut neko = NekoSearch::new().txt(txt).term(term);
    let score = neko.calc();
    let results = &neko.results;
    println!("{results:#?} {score}");
    neko
}

#[test]
fn test_find_perfect_match() {
    let mut neko = build_neko("Rust", "Rust");
    let result = neko.find();
    assert_eq!(result, 1.0);
}

#[test]
fn test_find_typo() {
    let mut neko = build_neko("Python", "Piton");
    let result = neko.find();
    assert!(result >= 0.5);
}

#[test]
fn test_find_transposed_letters() {
    let mut neko = build_neko("martha", "marhta");
    let result = neko.find();
    assert!(result > 0.6);
}

#[test]
fn test_find_common_prefix() {
    let mut neko = build_neko("Rustacean", "Rust");
    let result = neko.find();
    assert!(result >= 0.4);
}

#[test]
fn test_find_no_match() {
    let mut neko = build_neko("Rust", "C++");
    let result = neko.find();
    assert!(result < 0.2);
}

#[test]
fn test_find_different_lengths() {
    let mut neko = build_neko("development", "dev");
    let result = neko.find();
    assert!((0.3..0.7).contains(&result));
}

#[test]
fn test_custom_weights() {
    let mut neko_low_lev = NekoSearch::new().txt("martha").term("marhta").flow(vec![
        Jaro::new().weight(0.9).to_box(),
        Levenshtein::new().weight(1.0).to_box(),
    ]);

    let result_low_lev = neko_low_lev.find();

    let mut neko_high_lev = NekoSearch::new().txt("martha").term("marhta").flow(vec![
        Jaro::new().weight(1.0).to_box(),
        Levenshtein::new().weight(0.9).to_box(),
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
            assert_eq!(score, 1.0);
        }

        if term == "Rost" {
            assert!(score > 0.55, "similarity = {}", score);
        }

        if term == "Java" {
            assert!(score < 0.2, "similarity = {}", score);
        }
    }
}

#[test]
fn test_find_perfect_match_sentence() {
    let mut neko = build_neko("The quick brown fox", "The quick brown fox");
    let result = neko.find();
    assert_eq!(result, 1.0);
}

#[test]
fn test_find_typo_in_sentence() {
    let mut neko = build_neko("The quick brown fox", "The quikc brown fx");
    let result = neko.find();
    assert!(result >= 0.6);
}

#[test]
fn test_find_added_word() {
    let mut neko = build_neko("The quick brown fox", "The quick brown fox jumps");
    let result = neko.find();
    assert!(result > 0.5 && result < 1.0);
}

#[test]
fn test_find_removed_word() {
    let mut neko = build_neko("The quick brown fox jumps", "The quick brown fox");
    let result = neko.find();
    assert!(result > 0.5 && result < 1.0);
}

#[test]
fn test_find_transposed_words() {
    let mut neko = build_neko("The brown quick fox", "The quick brown fox");
    let result = neko.find();
    assert!(result > 0.5);
}

#[test]
fn test_long_paragraph_minor_edits() {
    let txt = "Rust is a systems programming language focused on safety, speed, and concurrency. It achieves memory safety without a garbage collector and has a rich type system.";
    let term = "Rust is a systems programming language focused on safety, speed and concurrency. It achieves memory-safety without a garbage collector and has a rich type system.";
    let mut neko = build_neko(txt, term);
    let result = neko.find();
    assert!(
        result > 0.9,
        "expected very high similarity, got {}",
        result
    );
}

#[test]
fn test_long_paragraph_added_sentence() {
    let txt = "Rust is a systems programming language focused on safety, speed, and concurrency. It has excellent tooling and a growing ecosystem.";
    let term = "Rust is a systems programming language focused on safety, speed, and concurrency. It has excellent tooling and a growing ecosystem. Many projects use it for networking and embedded development.";
    let mut neko = build_neko(txt, term);
    let result = neko.find();
    assert!(
        result > 0.7 && result < 1.0,
        "expected mid-high similarity, got {}",
        result
    );
}

#[test]
fn test_swapped_sentences_long_text() {
    let txt = "First sentence. Second sentence. Third sentence. Fourth sentence.";
    let term = "Third sentence. First sentence. Fourth sentence. Second sentence.";
    let mut neko = build_neko(txt, term);
    let result = neko.find();
    assert!(
        result > 0.5,
        "order changed but content similar, got {}",
        result
    );
}

#[test]
fn test_case_and_punctuation_variation() {
    let txt = "The Quick, Brown Fox: jumps over the lazy dog!";
    let term = "the quick brown fox jumps over the lazy dog";
    let mut neko = build_neko(txt, term);
    let result = neko.find();
    assert!(
        result > 0.85,
        "case/punctuation should not drop similarity too much, got {}",
        result
    );
}

#[test]
fn test_repeated_phrases_in_text() {
    let txt = "lorem ipsum dolor sit amet lorem ipsum dolor sit amet lorem ipsum dolor sit amet";
    let term = "lorem ipsum dolor sit amet lorem ipsum dolor sit amet";
    let mut neko = build_neko(txt, term);
    let result = neko.find();
    assert!(
        result >= 0.84,
        "repeated content should remain highly similar, got {}",
        result
    );
}

#[test]
fn test_many_typos_in_long_text() {
    let txt = "Concurrency and safe memory management are core to Rust's design. The language also prizes explicitness and performance.";
    let term = "Concurency and saft memry managment ar core to Rusts design. The lnguage also prise explicitnes and performnce.";
    let mut neko = build_neko(txt, term);
    let result = neko.find();
    assert!(
        result >= 0.45,
        "many typos but same words should keep some similarity, got {}",
        result
    );
}

#[test]
fn test_inserted_noise_words() {
    let txt = "Search algorithms compare strings and quantify similarity.";
    let term = "Search algorithms compare strings and quantify similarity. pineapple moonlight bicycle unicorn foobar baz";
    let mut neko = build_neko(txt, term);
    let result = neko.find();
    assert!(
        result > 0.6,
        "noise words added but core sentence present, got {}",
        result
    );
}

#[test]
fn test_completely_different_paragraphs() {
    let txt = "In a distant galaxy, stars collapsed into black holes and time flowed strangely.";
    let term = "The standard library contains modules for I/O, collections, and concurrency primitives in Rust.";
    let mut neko = build_neko(txt, term);
    let result = neko.find();
    assert!(
        result <= 0.41,
        "different topics should yield low similarity, got {}",
        result
    );
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