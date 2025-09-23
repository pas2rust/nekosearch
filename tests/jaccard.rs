use nekosearch::components::prelude::*;

const EPSILON: f64 = 0.000001;

#[test]
fn test_jaccard_with_identical_strings() {
    let s1 = "rustacean";
    let s2 = "rustacean";
    let n = 3;

    let ngrams1 = get_ngrams(s1, n);
    let ngrams2 = get_ngrams(s2, n);
    let similarity = jaccard_similarity(&ngrams1, &ngrams2);

    assert!((similarity - 1.0).abs() < EPSILON);
}

#[test]
fn test_jaccard_with_no_overlap() {
    let s1 = "rust";
    let s2 = "java";
    let n = 2;

    let ngrams1 = get_ngrams(s1, n);
    let ngrams2 = get_ngrams(s2, n);
    let similarity = jaccard_similarity(&ngrams1, &ngrams2);

    assert!((similarity - 0.0).abs() < EPSILON);
}

#[test]
fn test_jaccard_with_partial_overlap() {
    let s1 = "rust lang";
    let s2 = "rustacean";
    let n = 3;

    let ngrams1 = get_ngrams(s1, n);
    let ngrams2 = get_ngrams(s2, n);
    let similarity = jaccard_similarity(&ngrams1, &ngrams2);

    let intersection_count = 2;
    let union_count = ngrams1.len() + ngrams2.len() - intersection_count;
    let expected_similarity = intersection_count as f64 / union_count as f64;
    assert!((similarity - expected_similarity).abs() < EPSILON);
}
