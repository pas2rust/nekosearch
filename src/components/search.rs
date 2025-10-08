use super::prelude::*;
use kenzu::Builder;

#[derive(Builder)]
pub struct NekoSearch {
    pub txt: String,
    pub term: String,
    #[set(value=vec![
       Jaro::new().to_box(),
       Levenshtein::new().to_box(),
       DamerauLevenshtein::new().to_box(),
       Cosine::new().to_box(),
       Jaccard::new().to_box(),
       Metaphone::new().to_box(),
       Lcs::new().to_box()
    ])]
    pub flow: Vec<Box<dyn Calc>>,
    pub results: Vec<Find>,
}

#[derive(Builder, Debug)]
pub struct Find {
    pub algo: String,
    pub term: String,
    pub score: f32,
}

fn normalize(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c.is_whitespace() {
                c
            } else {
                ' '
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
}

impl NekoSearch {
    pub fn calc(&mut self) -> f32 {
        let txt = normalize(&self.txt);
        let term = normalize(&self.term);

        self.results.clear();

        let mut total_score = 0.0f32;
        let mut total_weight = 0.0f32;

        for algo in &self.flow {
            let score = algo.calc(txt.clone(), term.clone()).clamp(0.0, 1.0);
            let weight = algo.get_weight().clamp(0.0, 1.0);

            total_score += score * weight;
            total_weight += weight;

            self.results.push(
                Find::new()
                    .algo(algo.get_algo_name())
                    .term(term.clone())
                    .score(score),
            );
        }

        if total_weight == 0.0 {
            return 0.0;
        }

        (total_score / total_weight).clamp(0.0, 1.0)
    }

    pub fn find(&mut self) -> f32 {
        self.calc()
    }
}
