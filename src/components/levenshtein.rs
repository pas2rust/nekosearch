use std::cmp::min;

use kenzu::Builder;

use crate::Calc;

#[derive(Debug, Builder, Clone)]
pub struct Levenshtein {
    #[set(value = 1.0)]
    pub weight: f32,
}

impl Calc for Levenshtein {
    fn calc(&self, s1: String, s2: String) -> f32 {
        let s1_chars: Vec<char> = s1.chars().collect();
        let s2_chars: Vec<char> = s2.chars().collect();

        let len1 = s1_chars.len();
        let len2 = s2_chars.len();

        if len1 == 0 && len2 == 0 {
            return 1.0;
        }

        let mut matrix = vec![vec![0usize; len2 + 1]; len1 + 1];

        for (i, row) in matrix.iter_mut().enumerate() {
            row[0] = i;
        }

        for (j, cell) in matrix[0].iter_mut().enumerate() {
            *cell = j;
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
        let similarity = if max_len == 0.0 {
            1.0
        } else {
            1.0 - (distance / max_len)
        };

        let mut result = similarity as f32;
        result *= self.weight;
        result.clamp(0.0, 1.0)
    }

    fn get_weight(&self) -> f32 {
        self.weight
    }
}
