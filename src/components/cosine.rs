use crate::Calc;
use kenzu::Builder;
use std::collections::HashMap;

#[derive(Debug, Builder, Clone)]
pub struct Cosine {
    #[set(value = 100)]
    pub weight: u8,
    #[set(value = 2_usize)]
    pub ngram: usize,
}

impl Calc for Cosine {
    fn calc(&self, s1: String, s2: String) -> u8 {
        fn ngrams(s: &str, n: usize) -> Vec<String> {
            let chars: Vec<char> = s.chars().collect();
            if chars.len() < n {
                return chars.iter().map(|c| c.to_string()).collect();
            }
            chars.windows(n).map(|w| w.iter().collect()).collect()
        }
        let ngram = self.ngram;
        let b1 = ngrams(&s1, ngram);
        let b2 = ngrams(&s2, ngram);

        let mut vec1: HashMap<String, usize> = HashMap::new();
        let mut vec2: HashMap<String, usize> = HashMap::new();

        for bg in b1 {
            *vec1.entry(bg).or_insert(0) += 1;
        }
        for bg in b2 {
            *vec2.entry(bg).or_insert(0) += 1;
        }

        let mut dot = 0usize;
        for (k, v) in &vec1 {
            if let Some(v2) = vec2.get(k) {
                dot += v * v2;
            }
        }

        let norm1: f64 = vec1.values().map(|v| (*v * *v) as f64).sum::<f64>().sqrt();
        let norm2: f64 = vec2.values().map(|v| (*v * *v) as f64).sum::<f64>().sqrt();

        let cosine = if norm1 == 0.0 || norm2 == 0.0 {
            0.0
        } else {
            dot as f64 / (norm1 * norm2)
        };

        let percent = (cosine * 100.0).round();
        let weighted = (percent * (self.weight as f64) / 100.0).round() as i32;
        let clamped = weighted.clamp(0, 100);

        clamped as u8
    }

    fn get_weight(&self) -> u8 {
        self.weight
    }
}
