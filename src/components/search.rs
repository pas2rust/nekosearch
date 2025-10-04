use super::prelude::*;
use kenzu::Builder;

/// NekoSearch provides fuzzy text search based on a weighted combination of:
/// - **Levenshtein distance** (edit distance normalized as similarity)
/// - **Jaro-Winkler similarity** (good for typos and transpositions)
///
/// The weights for each similarity method can be adjusted when building
/// a `NekoSearch` instance.
pub struct NekoSearch {
    pub txt: String,
    pub term: String,
    pub flow: Vec<Box<dyn Calc>>,
}

impl Default for NekoSearch {
    fn default() -> Self {
        Self::new()
    }
}

impl NekoSearch {
    pub fn new() -> Self {
        Self {
            txt: String::default(),
            term: String::default(),
            flow: vec![
                Box::new(Jaro::new()) as Box<dyn Calc>,
                Box::new(Levenshtein::new()) as Box<dyn Calc>,
            ],
        }
    }

    pub fn txt(mut self, new: impl Into<String>) -> Self {
        self.txt = new.into();
        self
    }

    pub fn term(mut self, new: impl Into<String>) -> Self {
        self.term = new.into();
        self
    }

    pub fn flow(mut self, new: Vec<Box<dyn Calc>>) -> Self {
        self.flow = new;
        self
    }
}

impl std::fmt::Debug for NekoSearch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NekoSearch")
            .field("txt", &self.txt)
            .field("term", &self.term)
            .field("flow_len", &self.flow.len())
            .finish()
    }
}

/// Result of a fuzzy search comparison.
/// Holds the input term, the final aggregated score,
/// and the individual similarity scores that contributed to it.
#[derive(Builder, Debug)]
pub struct Find {
    /// Input term that was compared to the target.
    pub term: String,

    /// Final normalized similarity score (weighted combination).
    pub score: f64,
}

impl NekoSearch {
    pub fn calc(&self) -> f64 {
        let txt = &self.txt;
        let term = &self.term;

        let mut total_score = 0.0;
        let mut total_weight = 0.0;

        for algo in &self.flow {
            let score = algo.calc(txt.clone(), term.clone());
            let weight = algo.get_weight();
            total_score += score * weight;
            total_weight += weight;
        }

        if total_weight == 0.0 {
            0.0
        } else {
            total_score / total_weight
        }
    }
    pub fn find(&self) -> Find {
        let normalized_score = self.calc();
        Find::new().term(&self.term).score(normalized_score)
    }
}
