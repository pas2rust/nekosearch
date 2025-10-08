use std::fmt::Debug;

pub trait Calc: Debug {
    fn calc(&self, t1: String, t2: String) -> f32;
    fn get_weight(&self) -> f32 {
        1.0
    }
    fn to_box(self) -> Box<dyn Calc>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
    fn get_algo_name(&self) -> String {
        let full_name = std::any::type_name::<Self>();
        full_name
            .split("::")
            .last()
            .unwrap_or("Unknown")
            .to_string()
    }
}
