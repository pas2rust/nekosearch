use super::prelude::*;
use kenzu::Builder;

/// NekoSearch provides fuzzy text search based on a weighted combination of:
/// - **Levenshtein distance** (edit distance normalized as similarity)
/// - **Jaro-Winkler similarity** (good for typos and transpositions)
///
/// The weights for each similarity method can be adjusted when building
/// a `NekoSearch` instance.
#[derive(Builder)]
pub struct NekoSearch {
    pub txt: String,
    pub term: String,
    #[set(value=vec![
                Box::new(Jaro::new()) as Box<dyn Calc>,
                Box::new(Levenshtein::new()) as Box<dyn Calc>,
            ])]
    pub flow: Vec<Box<dyn Calc>>,
}

/// Result of a fuzzy search comparison.
/// Holds the input term, the final aggregated score,
/// and the individual similarity scores that contributed to it.
#[derive(Builder, Debug)]
pub struct Find {
    /// Input term that was compared to the target.
    pub term: String,

    /// Final normalized similarity score (weighted combination).
    pub score: u8,
}

impl NekoSearch {
    pub fn calc(&self) -> u8 {
        let txt = &self.txt;
        let term = &self.term;

        let mut total_score: f64 = 0.0;
        let mut total_weight: f64 = 0.0;

        for algo in &self.flow {
            let score = algo.calc(txt.clone(), term.clone()) as f64;
            let weight = algo.get_weight() as f64;
            total_score += score * weight;
            total_weight += weight;
        }

        if total_weight == 0.0 {
            return 0;
        }

        let averaged = (total_score / total_weight).round() as i32;
        averaged.clamp(0, 100) as u8
    }

    pub fn find(&self) -> Find {
        let normalized_score = self.calc();
        Find::new().term(&self.term).score(normalized_score)
    }
}
