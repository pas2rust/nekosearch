use std::cmp::min;

use kenzu::Builder;

use crate::Calc;

#[derive(Debug, Builder, Default, Clone)]
pub struct Levenshtein {
    #[set(value = 0.5)]
    pub weight: f64,
}

impl Calc for Levenshtein {
    fn calc(&self, s1: String, s2: String) -> f64 {
        let s1_chars: Vec<char> = s1.chars().collect();
        let s2_chars: Vec<char> = s2.chars().collect();

        let len1 = s1_chars.len();
        let len2 = s2_chars.len();

        if len1 == 0 && len2 == 0 {
            return 1.0;
        }

        let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

        for i in 0..=len1 {
            matrix[i][0] = i;
        }
        for j in 0..=len2 {
            matrix[0][j] = j;
        }

        for i in 1..=len1 {
            for j in 1..=len2 {
                let cost = if s1_chars[i - 1] == s2_chars[j - 1] {
                    0
                } else {
                    1
                };

                matrix[i][j] = min(
                    min(matrix[i - 1][j] + 1, matrix[i][j - 1] + 1),
                    matrix[i - 1][j - 1] + cost,
                );
            }
        }

        let distance = matrix[len1][len2] as f64;
        let max_len = (len1.max(len2)) as f64;
        // 🎯 MUST RETURN SIMILARITY (0.0 to 1.0)
        1.0 - (distance / max_len)
    }
    fn get_weight(&self) -> f64 {
        self.weight
    }
}
