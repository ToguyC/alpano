mod utils;

use std::f64;
use utils::{distance, math};

fn main() {
    println!("{}", distance::to_rad(1000.));
    println!("{}", distance::to_meter(f64::consts::PI));
    println!("{}", math::haversin(2.0));
    println!("{}", math::lerp(0.5, 0.0..=3.0));
    println!("{}", math::bilerp(0.0, 1.0, 2.0, 3.0, 1.0, 1.0));
    println!("{}", math::angular_distance(f64::consts::FRAC_PI_2, 0.));
}
