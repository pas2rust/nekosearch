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
       Jaccard::new().to_box()
    ])]
    pub flow: Vec<Box<dyn Calc>>,
    pub results: Vec<Find>,
}

#[derive(Builder, Debug)]
pub struct Find {
    pub algo: String,
    pub term: String,
    pub score: u8,
}

impl NekoSearch {
    pub fn calc(&mut self) -> u8 {
        let txt = &self.txt;
        let term = &self.term;

        self.results.clear();

        let mut total_score: f64 = 0.0;
        let mut total_weight: f64 = 0.0;

        for algo in &self.flow {
            let score = algo.calc(txt.clone(), term.clone());
            total_score += score as f64 * algo.get_weight() as f64;
            total_weight += algo.get_weight() as f64;

            self.results.push(
                Find::new()
                    .algo(std::any::type_name::<&Box<dyn Calc>>().to_string())
                    .term(term.clone())
                    .score(score),
            );
        }

        if total_weight == 0.0 {
            return 0;
        }

        let averaged = (total_score / total_weight).round() as i32;
        averaged.clamp(0, 100) as u8
    }

    pub fn find(&mut self) -> u8 {
        self.calc()
    }
}
