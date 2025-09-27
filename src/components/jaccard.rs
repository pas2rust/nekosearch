use super::prelude::*;
use kenzu::Builder;
use std::collections::HashSet;

#[derive(Debug, Builder, Default, Clone)]
pub struct JaccardValue {
    pub target: String,
    #[set(value = 3_usize)]
    pub ngram_size: usize,
    pub ngrams: HashSet<String>,
}

#[derive(Debug, Builder, Default, Clone)]
pub struct Jaccard {
    #[set(value = 0.2)]
    pub weight: f64,
}

impl Calc<JaccardValue, JaccardValue> for Jaccard {
    fn calc(&self, value1: JaccardValue, value2: JaccardValue) -> f64 {
        let intersection = value1.ngrams.intersection(&value2.ngrams).count() as f64;
        let union = value1.ngrams.union(&value2.ngrams).count() as f64;
        if union == 0.0 {
            0.0
        } else {
            intersection / union
        }
    }
}
