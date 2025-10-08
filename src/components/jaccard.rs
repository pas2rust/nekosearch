use crate::Calc;
use kenzu::Builder;
use std::collections::HashSet;

#[derive(Debug, Builder, Clone)]
pub struct Jaccard {
    #[set(value = 100)]
    pub weight: u8,
    #[set(value = 2_usize)]
    pub ngram: usize,
}

impl Calc for Jaccard {
    fn calc(&self, s1: String, s2: String) -> u8 {
        fn ngrams(s: &str, n: usize) -> HashSet<String> {
            let chars: Vec<char> = s.chars().collect();
            if chars.is_empty() || n == 0 {
                return HashSet::new();
            }

            let n = n.min(chars.len());

            if n == 1 {
                return chars.iter().map(|c| c.to_string()).collect();
            }

            chars.windows(n).map(|w| w.iter().collect()).collect()
        }

        let n = self.ngram.max(1);
        let set1 = ngrams(&s1, n);
        let set2 = ngrams(&s2, n);

        let intersection = set1.intersection(&set2).count() as f64;
        let union = set1.union(&set2).count() as f64;

        let similarity = if union == 0.0 {
            0.0
        } else {
            intersection / union
        };
        let percent = (similarity * 100.0).round();

        ((percent * self.weight as f64 / 100.0).round() as i32).clamp(0, 100) as u8
    }

    fn get_weight(&self) -> u8 {
        self.weight
    }
}
