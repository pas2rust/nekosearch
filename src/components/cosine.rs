use crate::Calc;
use kenzu::Builder;
use std::collections::HashMap;

#[derive(Debug, Builder, Clone)]
pub struct Cosine {
    #[set(value = 1.0)]
    pub weight: f32,
    #[set(value = 2_usize)]
    pub ngram: usize,
}

impl Calc for Cosine {
    fn calc(&self, s1: String, s2: String) -> f32 {
        fn ngrams(s: &str, n: usize) -> Vec<String> {
            let chars: Vec<char> = s.chars().collect();
            if chars.len() < n {
                return chars.into_iter().map(|c| c.to_string()).collect();
            }
            chars
                .windows(n)
                .map(|w| w.iter().cloned().collect::<String>())
                .collect()
        }

        let n = self.ngram.max(1);
        let b1 = ngrams(&s1, n);
        let b2 = ngrams(&s2, n);

        let mut vec1: HashMap<String, usize> = HashMap::new();
        let mut vec2: HashMap<String, usize> = HashMap::new();

        for bg in b1 {
            *vec1.entry(bg).or_insert(0) += 1;
        }
        for bg in b2 {
            *vec2.entry(bg).or_insert(0) += 1;
        }

        let mut dot: usize = 0;
        for (k, v) in &vec1 {
            if let Some(v2) = vec2.get(k) {
                dot += v * v2;
            }
        }

        let norm1: f64 = vec1.values().map(|v| (*v * *v) as f64).sum::<f64>().sqrt();
        let norm2: f64 = vec2.values().map(|v| (*v * *v) as f64).sum::<f64>().sqrt();

        let cosine: f64 = if norm1 == 0.0 || norm2 == 0.0 {
            0.0
        } else {
            (dot as f64) / (norm1 * norm2)
        };

        let mut result = cosine as f32;
        result *= self.weight;
        result.clamp(0.0, 1.0)
    }

    fn get_weight(&self) -> f32 {
        self.weight
    }
}
