use crate::Calc;
use kenzu::Builder;
use rphonetic::{DoubleMetaphone, Encoder};

#[derive(Debug, Builder, Clone)]
pub struct Metaphone {
    #[set(value = 1.0)]
    pub weight: f32,
}

impl Metaphone {
    fn get_metaphone_encoder() -> DoubleMetaphone {
        DoubleMetaphone::default()
    }
}

impl Calc for Metaphone {
    fn calc(&self, s1: String, s2: String) -> f32 {
        let encoder = Metaphone::get_metaphone_encoder();

        let code1_primary = encoder.encode(&s1);
        let code1_alternate = encoder.encode_alternate(&s1);

        let code2_primary = encoder.encode(&s2);
        let code2_alternate = encoder.encode_alternate(&s2);

        let are_similar = code1_primary == code2_primary
            || (!code2_alternate.is_empty() && code1_primary == code2_alternate)
            || (!code1_alternate.is_empty() && code1_alternate == code2_primary)
            || (!code1_alternate.is_empty()
                && !code2_alternate.is_empty()
                && code1_alternate == code2_alternate);

        let mut result = if are_similar { 1.0 } else { 0.0 };
        result *= self.weight;
        result.clamp(0.0, 1.0)
    }

    fn get_weight(&self) -> f32 {
        self.weight
    }
}
