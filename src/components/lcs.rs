use crate::Calc;
use kenzu::Builder;
use std::cmp::max;

#[derive(Debug, Builder, Clone)]
pub struct Lcs {
    #[set(value = 1.0)]
    pub weight: f32,
}

impl Calc for Lcs {
    fn calc(&self, s1: String, s2: String) -> f32 {
        let a: Vec<char> = s1.chars().collect();
        let b: Vec<char> = s2.chars().collect();

        let n = a.len();
        let m = b.len();

        if n == 0 || m == 0 {
            return 0.0;
        }

        let mut matrix: Vec<Vec<usize>> = vec![vec![0_usize; m + 1]; n + 1];

        for i in 1..=n {
            for j in 1..=m {
                if a[i - 1] == b[j - 1] {
                    matrix[i][j] = matrix[i - 1][j - 1] + 1;
                } else {
                    matrix[i][j] = max(matrix[i - 1][j], matrix[i][j - 1]);
                }
            }
        }

        let lcs_length = matrix[n][m] as f32;

        let max_len = max(n, m) as f32;

        let normalized_score = if max_len == 0.0 {
            0.0
        } else {
            lcs_length / max_len
        };

        let mut result = normalized_score;
        result *= self.weight;
        result.clamp(0.0, 1.0)
    }

    fn get_weight(&self) -> f32 {
        self.weight
    }

    fn get_algo_name(&self) -> String {
        let full_name = std::any::type_name::<Self>();
        full_name.split("::").last().unwrap_or("Lcs").to_string()
    }
}
