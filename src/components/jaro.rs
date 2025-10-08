use kenzu::Builder;

use crate::Calc;

#[derive(Debug, Builder, Clone)]
pub struct Jaro {
    #[set(value = 1.0)]
    pub weight: f32,
    #[set(value = 4_usize)]
    pub chars: usize,
}

impl Calc for Jaro {
    fn calc(&self, s1: String, s2: String) -> f32 {
        let s1_chars: Vec<char> = s1.chars().collect();
        let s2_chars: Vec<char> = s2.chars().collect();
        let s1_len = s1_chars.len();
        let s2_len = s2_chars.len();

        if s1_len == 0 && s2_len == 0 {
            return 1.0;
        }
        if s1_len == 0 || s2_len == 0 {
            return 0.0;
        }

        let max_dist = ((s1_len.max(s2_len)) / 2).saturating_sub(1);

        let mut s2_flags = vec![false; s2_len];
        let mut s1_matches: Vec<char> = Vec::new();

        for (i, c1) in s1_chars.iter().enumerate() {
            let start = i.saturating_sub(max_dist);
            let end = (i + max_dist + 1).min(s2_len);
            for j in start..end {
                if !s2_flags[j] && *c1 == s2_chars[j] {
                    s1_matches.push(*c1);
                    s2_flags[j] = true;
                    break;
                }
            }
        }

        let matches = s1_matches.len();
        if matches == 0 {
            return 0.0;
        }

        let mut s2_matches: Vec<char> = Vec::with_capacity(matches);
        for (j, &c2) in s2_chars.iter().enumerate() {
            if s2_flags[j] {
                s2_matches.push(c2);
            }
        }

        let mut transpositions = 0usize;
        for i in 0..matches {
            if s1_matches[i] != s2_matches[i] {
                transpositions += 1;
            }
        }

        let m = matches as f64;
        let t = (transpositions as f64) / 2.0;

        let jaro = (m / s1_len as f64 + m / s2_len as f64 + (m - t) / m) / 3.0;

        let prefix_len = s1_chars
            .iter()
            .zip(s2_chars.iter())
            .take(self.chars)
            .take_while(|(c1, c2)| c1 == c2)
            .count();
        let jaro_winkler = jaro + (prefix_len as f64) * 0.076 * (1.0 - jaro);

        let mut result = jaro_winkler as f32;
        result *= self.weight;
        result.clamp(0.0, 1.0)
    }

    fn get_weight(&self) -> f32 {
        self.weight
    }
}
