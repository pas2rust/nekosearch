use crate::Calc;
use kenzu::Builder;
use std::cmp::min;

#[derive(Debug, Builder, Clone)]
pub struct DamerauLevenshtein {
    #[set(value = 100)]
    pub weight: u8,
}

impl Calc for DamerauLevenshtein {
    fn calc(&self, s1: String, s2: String) -> u8 {
        let s1_chars: Vec<char> = s1.chars().collect();
        let s2_chars: Vec<char> = s2.chars().collect();
        let len1 = s1_chars.len();
        let len2 = s2_chars.len();

        if len1 == 0 && len2 == 0 {
            return 100;
        }

        let mut matrix = vec![vec![0usize; len2 + 1]; len1 + 1];

        for (i, row) in matrix.iter_mut().enumerate().take(len1 + 1) {
            row[0] = i;
        }

        for (j, cell) in matrix[0].iter_mut().enumerate().take(len2 + 1) {
            *cell = j;
        }

        for i in 1..=len1 {
            for j in 1..=len2 {
                let cost = if s1_chars[i - 1] == s2_chars[j - 1] {
                    0
                } else {
                    1
                };
                let mut val = min(
                    min(matrix[i - 1][j] + 1, matrix[i][j - 1] + 1),
                    matrix[i - 1][j - 1] + cost,
                );

                if i > 1
                    && j > 1
                    && s1_chars[i - 1] == s2_chars[j - 2]
                    && s1_chars[i - 2] == s2_chars[j - 1]
                {
                    val = min(val, matrix[i - 2][j - 2] + cost);
                }

                matrix[i][j] = val;
            }
        }

        let distance = matrix[len1][len2] as f64;
        let max_len = len1.max(len2) as f64;
        let similarity = if max_len == 0.0 {
            1.0
        } else {
            1.0 - (distance / max_len)
        };

        let percent = (similarity * 100.0).round();
        let weighted = (percent * (self.weight as f64) / 100.0).round() as i32;
        let clamped = weighted.clamp(0, 100);

        clamped as u8
    }

    fn get_weight(&self) -> u8 {
        self.weight
    }
}
