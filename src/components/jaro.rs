use kenzu::Builder;

use crate::Calc;

#[derive(Debug, Builder, Default, Clone)]
pub struct Jaro {
    #[set(value = 0.6)]
    pub weight: f64,
}

impl Calc<&str, &str> for Jaro {
    fn calc(&self, s1: &str, s2: &str) -> f64 {
        let s1_chars: Vec<char> = s1.chars().collect();
        let s2_chars: Vec<char> = s2.chars().collect();
        let s1_len = s1_chars.len();
        let s2_len = s2_chars.len();

        if s1_len == 0 || s2_len == 0 {
            return 0.0;
        }

        let max_dist = ((s1_len.max(s2_len)) / 2).saturating_sub(1);
        let mut s2_flags = vec![false; s2_len];
        let mut s1_matches = Vec::new();

        for (i, c1) in s1_chars.iter().enumerate() {
            for (j, c2) in s2_chars.iter().enumerate() {
                if !s2_flags[j] && (i as i32 - j as i32).abs() <= max_dist as i32 && c1 == c2 {
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

        let mut s2_matches = Vec::new();
        for (j, &c2) in s2_chars.iter().enumerate() {
            if s2_flags[j] {
                s2_matches.push(c2);
            }
        }

        let mut transpositions = 0.0;
        for i in 0..matches {
            if s1_matches[i] != s2_matches[i] {
                transpositions += 1.0;
            }
        }
        transpositions /= 2.0;

        let jaro = (matches as f64 / s1_len as f64
            + matches as f64 / s2_len as f64
            + (matches as f64 - transpositions) / matches as f64)
            / 3.0;

        let prefix_len = s1_chars
            .iter()
            .zip(s2_chars.iter())
            .take(4)
            .take_while(|(c1, c2)| c1 == c2)
            .count();
        let p = prefix_len as f64;
        jaro + p * 0.1 * (1.0 - jaro)
    }
}
