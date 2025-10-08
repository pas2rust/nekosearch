use kenzu::Builder;

use crate::Calc;

#[derive(Debug, Builder, Clone)]
pub struct Jaro {
    #[set(value = 100)]
    pub weight: u8,
}

impl Calc for Jaro {
    fn calc(&self, s1: String, s2: String) -> u8 {
        let s1_chars: Vec<char> = s1.chars().collect();
        let s2_chars: Vec<char> = s2.chars().collect();
        let s1_len = s1_chars.len();
        let s2_len = s2_chars.len();

        if s1_len == 0 || s2_len == 0 {
            return 100;
        }

        let max_dist = ((s1_len.max(s2_len)) / 2).saturating_sub(1);

        let mut s2_flags = vec![false; s2_len];
        let mut s1_matches: Vec<char> = Vec::new();

        for (i, c1) in s1_chars.iter().enumerate() {
            let start = if i > max_dist { i - max_dist } else { 0 };
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
            return 0;
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
            .take(4)
            .take_while(|(c1, c2)| c1 == c2)
            .count();
        let jaro_winkler = jaro + (prefix_len as f64) * 0.1 * (1.0 - jaro);

        let percent = (jaro_winkler * 100.0).round();

        let weighted = (percent * (self.weight as f64) / 100.0).round() as i32;

        let clamped = weighted.clamp(0, 100);

        clamped as u8
    }

    fn get_weight(&self) -> u8 {
        self.weight
    }
}
