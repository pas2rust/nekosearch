pub trait Calc<T1, T2> {
    fn calc(&self, t1: T1, t2: T2) -> f64;
}
