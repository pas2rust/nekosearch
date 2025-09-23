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
    /// Target string that all queries will be compared against.
    pub target: String,

    /// Weight for the Levenshtein similarity score.
    #[set(value = 0.2)]
    pub lev_weight: f64,

    /// Weight for the Jaro-Winkler similarity score.
    #[set(value = 0.6)]
    pub jaro_winkler_weight: f64,

    /// Weight for the N-gram Jaccard similarity score.
    #[set(value = 0.2)]
    pub n_gram_weight: f64,
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
    /// Calculate all score components (Levenshtein, Jaro-Winkler, Jaccard)
    /// and return them along with the final weighted score.
    ///
    /// The input string is normalized (trimmed and lowercased) before comparison.
    fn calc_score_components(&self, txt: &str) -> (f64, f64, f64, f64) {
        let target_norm = self.target.trim().to_lowercase();
        let text_norm = txt.trim().to_lowercase();

        let lev_dist = levenshtein_distance(&target_norm, &text_norm);
        let max_len = target_norm.len().max(text_norm.len()) as f64;
        let lev_score = if max_len == 0.0 {
            1.0 
        } else {
            1.0 - (lev_dist as f64 / max_len)
        };

        let jaro_score = jaro_winkler_similarity(&target_norm, &text_norm);

        let n_gram_size = 2;
        let target_ngrams = get_ngrams(&target_norm, n_gram_size);
        let text_ngrams = get_ngrams(&text_norm, n_gram_size);
        let jaccard_score = jaccard_similarity(&target_ngrams, &text_ngrams);

        let combined_score = lev_score * self.lev_weight
            + jaro_score * self.jaro_winkler_weight
            + jaccard_score * self.n_gram_weight;

        let total_weight = self.lev_weight + self.jaro_winkler_weight + self.n_gram_weight;
        let normalized_score = if total_weight == 0.0 {
            0.0
        } else {
            combined_score / total_weight
        };

        (normalized_score, lev_score, jaro_score, jaccard_score)
    }

    /// Compare a single string against the target and return a [`Find`] result
    /// with the aggregated and raw similarity scores.
    pub fn find<T: Into<String>>(&self, text: T) -> Find {
        let txt = text.into();
        let (score, lev_score, jaro_score, jaccard_score) = self.calc_score_components(&txt);

        Find::new()
            .term(txt)
            .score(score)
            .lev_score(lev_score)
            .jaro_score(jaro_score)
            .jaccard_score(jaccard_score)
    }

    /// Compare multiple strings against the target.
    ///
    /// Returns a vector of [`Find`] results, one for each input string.
    pub fn filter<'a, T: Into<Vec<&'a str>>>(&self, text: T) -> Vec<Find> {
        let texts = text.into();
        let mut results = Vec::with_capacity(texts.len());

        for &term in &texts {
            let term_string = term.to_string();
            let (score, lev_score, jaro_score, jaccard_score) =
                self.calc_score_components(&term_string);

            let find_result = Find::new()
                .term(term_string)
                .score(score)
                .lev_score(lev_score)
                .jaro_score(jaro_score)
                .jaccard_score(jaccard_score);

            results.push(find_result);
        }

        results
    }
}
