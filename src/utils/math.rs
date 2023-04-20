use std::{
    f64::consts::{PI, TAU},
    ops::RangeInclusive,
};

pub trait Math {
    fn haversin(&self) -> Self;
    fn lerp(&self, range: RangeInclusive<f64>) -> Self;
}

impl Math for f64 {
    fn haversin(&self) -> Self {
        (*self / 2.).sin().powi(2)
    }

    fn lerp(&self, range: RangeInclusive<f64>) -> Self {
        *range.start() * (1.0 - *self) + (*range.end() * *self)
    }
}

pub fn angular_distance(a1: f64, a2: f64) -> f64 {
    ((a2 - a1 + PI) % TAU) - PI
}

pub fn bilerp(z00: f64, z10: f64, z01: f64, z11: f64, x: f64, y: f64) -> f64 {
    let x_0_1 = x.lerp(z00..=z10);
    let x_1_2 = x.lerp(z01..=z11);
    y.lerp(x_0_1..=x_1_2)
}
