use nekosearch::components::prelude::*;
use std::collections::HashSet;

const EPSILON: f64 = 0.000001;

fn build_jaccard_value(target: &str, ngram_size: usize) -> JaccardValue {
    let ngrams: HashSet<String> = target
        .chars()
        .collect::<Vec<_>>()
        .windows(ngram_size)
        .map(|w| w.iter().collect())
        .collect();

    JaccardValue {
        target: target.to_string(),
        ngram_size,
        ngrams,
    }
}

#[test]
fn test_jaccard_identical_strings() {
    let value1 = build_jaccard_value("rustacean", 3);
    let value2 = build_jaccard_value("rustacean", 3);

    let jaccard = Jaccard::default();
    let similarity = jaccard.calc(value1, value2);

    assert!((similarity - 1.0).abs() < EPSILON);
}

#[test]
fn test_jaccard_no_overlap() {
    let value1 = build_jaccard_value("rust", 2);
    let value2 = build_jaccard_value("java", 2);

    let jaccard = Jaccard::default();
    let similarity = jaccard.calc(value1, value2);

    assert!((similarity - 0.0).abs() < EPSILON);
}

#[test]
fn test_jaccard_partial_overlap() {
    let value1 = build_jaccard_value("rust lang", 3);
    let value2 = build_jaccard_value("rustacean", 3);

    let jaccard = Jaccard::default();
    let similarity = jaccard.calc(value1.clone(), value2.clone());

    let intersection_count = value1.ngrams.intersection(&value2.ngrams).count();
    let union_count = value1.ngrams.union(&value2.ngrams).count();
    let expected_similarity = intersection_count as f64 / union_count as f64;

    assert!((similarity - expected_similarity).abs() < EPSILON);
}
