use std::collections::HashSet;

pub fn get_ngrams(s: &str, n: usize) -> HashSet<String> {
    let mut ngrams = HashSet::new();
    let chars: Vec<char> = s.chars().collect();

    if chars.len() < n {
        return ngrams;
    }

    for i in 0..=chars.len() - n {
        let ngram = chars[i..i + n].iter().collect();
        ngrams.insert(ngram);
    }
    ngrams
}

pub fn jaccard_similarity(set1: &HashSet<String>, set2: &HashSet<String>) -> f64 {
    let intersection = set1.intersection(set2).count() as f64;
    let union = set1.union(set2).count() as f64;

    if union == 0.0 {
        return 0.0;
    }
    intersection / union
}
