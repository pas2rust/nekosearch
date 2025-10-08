use std::fmt::Debug;

pub trait Calc: Debug {
    fn calc(&self, t1: String, t2: String) -> u8;
    fn get_weight(&self) -> u8 {
        100
    }
    fn to_box(self) -> Box<dyn Calc>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
}
