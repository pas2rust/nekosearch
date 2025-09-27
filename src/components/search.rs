use super::prelude::*;
use kenzu::Builder;

/// NekoSearch provides fuzzy text search based on a weighted combination of:
/// - **Levenshtein distance** (edit distance normalized as similarity)
/// - **Jaro-Winkler similarity** (good for typos and transpositions)
/// - **N-gram Jaccard similarity** (good for capturing overlapping substrings)
///
/// The weights for each similarity method can be adjusted when building
/// a `NekoSearch` instance.
#[derive(Builder, Debug, Clone)]
pub struct NekoSearch {
    pub txt: String,
    pub term: String,
    pub jaro: Jaro,
    pub jaccard: Jaccard,
    pub levenshtein: Levenshtein,
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

    /// Raw Levenshtein similarity score.
    pub lev_score: f64,

    /// Raw Jaro-Winkler similarity score.
    pub jaro_score: f64,

    /// Raw Jaccard similarity score using N-grams.
    pub jaccard_score: f64,
}

impl NekoSearch {
    pub fn calc(&self, ngram_size: usize) -> f64 {
        let txt = &self.txt;
        let term = &self.term;
        let lev_score = self.levenshtein.calc(txt, term);
        let jaro_score = self.jaro.calc(txt, term);
        let jaccard_score = self.jaccard.calc(
            JaccardValue::new().ngram_size(ngram_size).target(txt),
            JaccardValue::new().ngram_size(ngram_size).target(term),
        );
        let combined_score = lev_score + jaro_score + jaccard_score;
        let total_weight = 3.0;
        let normalized_score = if total_weight == 0.0 {
            0.0
        } else {
            combined_score / total_weight
        };

        normalized_score
    }
    pub fn find(&self, ngram_size: usize) -> Find {
        let txt = &self.txt;
        let term = &self.term;
        let lev_score_raw = self.levenshtein.calc(txt, term);
        let jaro_score_raw = self.jaro.calc(txt, term);
        let jaccard_score_raw = self.jaccard.calc(
            JaccardValue::new().target(txt).ngram_size(ngram_size),
            JaccardValue::new().target(term).ngram_size(ngram_size),
        );

        let lev_score = lev_score_raw * self.levenshtein.weight;
        let jaro_score = jaro_score_raw * self.jaro.weight;
        let jaccard_score = jaccard_score_raw * self.jaccard.weight;

        let total_weight = self.levenshtein.weight + self.jaro.weight + self.jaccard.weight;
        let normalized_score = if total_weight == 0.0 {
            0.0
        } else {
            (lev_score + jaro_score + jaccard_score) / total_weight
        };

        Find::new()
            .term(term)
            .score(normalized_score)
            .lev_score(lev_score_raw)
            .jaro_score(jaro_score_raw)
            .jaccard_score(jaccard_score_raw)
    }
}
