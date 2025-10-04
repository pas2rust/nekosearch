pub trait Calc {
    fn calc(&self, t1: String, t2: String) -> f64;
    fn get_weight(&self) -> f64 {
        1.0
    }
}
