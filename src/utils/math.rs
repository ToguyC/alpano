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
    let diff = (a2 - a1 + PI) % TAU - PI;

    if diff < -PI {
        diff + TAU
    } else {
        diff
    }
}

pub fn bilerp(z00: f64, z10: f64, z01: f64, z11: f64, x: f64, y: f64) -> f64 {
    let x_0_1 = x.lerp(z00..=z10);
    let x_1_2 = x.lerp(z01..=z11);
    y.lerp(x_0_1..=x_1_2)
}

#[cfg(test)]
mod math_tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use rand::{rngs::ThreadRng, Rng};

    fn next_angle(rng: &mut ThreadRng) -> f64 {
        let random: f64 = rng.gen_range(-180.0..180.0);
        random.to_radians()
    }

    #[test]
    fn haversin_is_correct_on_random_values() {
        let mut rng = rand::thread_rng();

        for _ in 0..500 {
            let a = next_angle(&mut rng);
            let h = (1. - a.cos()) / 2.;
            assert_approx_eq!(h, a.haversin(), 1e-10);
        }
    }

    #[test]
    fn angular_distance_is_correct_on_known_angles() {
        let data: Vec<f64> = vec![
            0., 45., 45., 45., 0., -45., 0., 179., 179., 0., 181., -179., 181., 359., 178., 181.,
            2., -179.,
        ];

        for i in (0..data.len()).step_by(3) {
            let a1 = data[i].to_radians();
            let a2 = data[i + 1].to_radians();
            let expected = data[i + 2].to_radians();
            let actual = angular_distance(a1, a2);
            assert_approx_eq!(expected, actual, 1e-10);
        }
    }

    #[test]
    fn angular_distance_is_in_expected_range() {
        let mut rng = rand::thread_rng();

        for _ in 0..500 {
            let a1 = next_angle(&mut rng);
            let a2 = next_angle(&mut rng);
            let d = angular_distance(a1, a2);
            assert!(-PI <= d && d < PI);
        }
    }

    #[test]
    fn angular_distance_is_symmetric() {
        let mut rng = rand::thread_rng();
        for _ in 0..500 {
            let a1 = next_angle(&mut rng);
            let a2 = next_angle(&mut rng);
            assert_approx_eq!(
                0.,
                angular_distance(a1, a2) + angular_distance(a2, a1),
                1e-10
            );
        }
    }

    #[test]
    fn lerp_is_first_value_at_start() {
        let mut rng = rand::thread_rng();
        for _ in 0..500 {
            let v1 = (rng.gen::<f64>() - 0.5) * 1000.;
            let v2 = (rng.gen::<f64>() - 0.5) * 1000.;
            assert_approx_eq!(v1, 0.0.lerp(v1..=v2), 1e-10);
        }
    }

    #[test]
    fn lerp_is_first_value_at_middle() {
        let mut rng = rand::thread_rng();
        for _ in 0..500 {
            let v1 = (rng.gen::<f64>() - 0.5) * 1000.;
            let v2 = (rng.gen::<f64>() - 0.5) * 1000.;
            assert_approx_eq!((v1 + v2) / 2., 0.5.lerp(v1..=v2), 1e-10);
        }
    }

    #[test]
    fn lerp_is_first_value_at_end() {
        let mut rng = rand::thread_rng();
        for _ in 0..500 {
            let v1 = (rng.gen::<f64>() - 0.5) * 1000.;
            let v2 = (rng.gen::<f64>() - 0.5) * 1000.;
            assert_approx_eq!(v2, 1.0.lerp(v1..=v2), 1e-10);
        }
    }

    #[test]
    fn lerp_is_in_expected_range() {
        let mut rng = rand::thread_rng();
        for _ in 0..500 {
            let v1 = (rng.gen::<f64>() - 0.5) * 1000.;
            let v2 = (rng.gen::<f64>() - 0.5) * 1000.;
            let p = rng.gen::<f64>();
            let v = p.lerp(v1..=v2);
            assert!(v1.min(v2) <= v && v <= v1.max(v2));
        }
    }

    #[test]
    fn bilerp_is_in_expected_range() {
        let mut rng = rand::thread_rng();
        for _ in 0..500 {
            let v1 = (rng.gen::<f64>() - 0.5) * 1000.;
            let v2 = (rng.gen::<f64>() - 0.5) * 1000.;
            let v3 = (rng.gen::<f64>() - 0.5) * 1000.;
            let v4 = (rng.gen::<f64>() - 0.5) * 1000.;
            let x = rng.gen::<f64>();
            let y = rng.gen::<f64>();
            let v = bilerp(v1, v2, v3, v4, x, y);
            assert!(v1.min(v2).min(v3).min(v4) <= v && v <= v1.max(v2).max(v3).max(v4));
        }
    }

    #[test]
    fn bilerp_is_correct_in_corners() {
        let mut rng = rand::thread_rng();
        for _ in 0..500 {
            let v1 = rng.gen::<f64>();
            let v2 = rng.gen::<f64>();
            let v3 = rng.gen::<f64>();
            let v4 = rng.gen::<f64>();
            assert_approx_eq!(v1, bilerp(v1, v2, v3, v4, 0., 0.), 1e-10);
            assert_approx_eq!(v2, bilerp(v2, v2, v3, v4, 1., 0.), 1e-10);
            assert_approx_eq!(v3, bilerp(v3, v2, v3, v4, 0., 1.), 1e-10);
            assert_approx_eq!(v4, bilerp(v4, v2, v3, v4, 1., 1.), 1e-10);
        }
    }

    #[test]
    fn bilerp_is_correct_along_sides() {
        let mut rng = rand::thread_rng();
        for _ in 0..500 {
            let v1 = rng.gen::<f64>();
            let v2 = rng.gen::<f64>();
            let v3 = rng.gen::<f64>();
            let v4 = rng.gen::<f64>();
            assert_approx_eq!((v1 + v2) / 2., bilerp(v1, v2, v3, v4, 0.5, 0.), 1e-10);
            assert_approx_eq!((v1 + v3) / 2., bilerp(v1, v2, v3, v4, 0., 0.5), 1e-10);
            assert_approx_eq!((v3 + v4) / 2., bilerp(v1, v2, v3, v4, 0.5, 1.), 1e-10);
            assert_approx_eq!((v2 + v4) / 2., bilerp(v1, v2, v3, v4, 1., 0.5), 1e-10);
        }
    }
}
