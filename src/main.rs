mod utils;

use std::f64;
use utils::{distance, math};

use crate::utils::math::Math;

fn main() {
    println!("{}", distance::to_rad(1000.));
    println!("{}", distance::to_meter(f64::consts::PI));
    println!("{}", 2.0.haversin());
    println!("{}", 0.5.lerp(0.0..=3.0));
    println!("{}", math::bilerp(0.0, 1.0, 2.0, 3.0, 1.0, 1.0));
    println!("{}", math::angular_distance(f64::consts::FRAC_PI_2, 0.));
}
