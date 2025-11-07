use std::f64::consts::PI;

pub trait Shape {
    fn area(&self) -> f64;
}

pub struct Circle {
    pub diameter: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        return PI * self.diameter.powi(2);
    }
}
