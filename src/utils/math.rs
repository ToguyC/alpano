use std::{
    f64::consts::{PI, TAU},
    ops::RangeInclusive,
};

pub trait FloatTraitOverload {
    /// Compute the haversine value `(sin(x/2))^2`
    fn haversin(&self) -> Self;

    /// Linear interpolation of the current value on the range
    ///
    /// # Arguments
    ///
    /// * `range` - Inclusive range to interpolate on
    fn lerp(&self, range: RangeInclusive<f64>) -> Self;
}

impl FloatTraitOverload for f64 {
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

pub fn first_interval_containing_root(f: fn(f64) -> f64, min_x: f64, max_x: f64, dx: f64) -> f64 {
    let mut i = min_x;

    while i < max_x {
        if let Ok(_) = improve_root(f, i, i + dx, 1e-10) {
            return i;
        }

        i += dx;
    }

    f64::INFINITY
}

pub fn improve_root(f: fn(f64) -> f64, mut x1: f64, mut x2: f64, eps: f64) -> Result<f64, ()> {
    if f(x1).signum() == f(x2).signum() || x1 > x2 {
        return Err(());
    }

    while (x2 - x1) > eps {
        let m = (x1 + x2) / 2.;
        if f(m).signum() == f(x1).signum() {
            x1 = m;
        } else {
            x2 = m;
        }
    }

    Ok(x1)
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

    #[test]
    fn first_interval_containing_root_works_on_sin() {
        let i1 = first_interval_containing_root(|x| x.sin(), -1., 1., 0.1 + 1e-11);
        assert_approx_eq!(-0.1, i1, 1e-10);

        let i2 = first_interval_containing_root(|x| x.sin(), 1., 4., 1.);
        assert_approx_eq!(3., i2, f64::EPSILON);
    }

    #[test]
    fn improve_root_fails_when_interval_does_not_contains_root() {
        match improve_root(|x| x.sin(), 1., 2., 1e-10) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn improve_root_works_on_sin() {
        let pi = improve_root(|x| x.sin(), 3.1, 3.2, 1e-10).unwrap();
        assert_approx_eq!(PI, pi, 1e-10);

        let m_pi = improve_root(|x| x.sin(), -4., -3.1, 1e-10).unwrap();
        assert_approx_eq!(-PI, m_pi, 1e-10);
    }
}
