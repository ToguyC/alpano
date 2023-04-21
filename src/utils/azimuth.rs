use std::f64::consts::{FRAC_PI_4, TAU};

pub fn is_canonical(azimuth: f64) -> bool {
    (0.0..TAU).contains(&azimuth)
}

pub fn canonicalize(azimuth: f64) -> f64 {
    azimuth.rem_euclid(TAU)
}

pub fn to_math(azimuth: f64) -> Result<f64, ()> {
    if !is_canonical(azimuth) {
        return Err(());
    }

    Ok((TAU - azimuth).rem_euclid(TAU))
}

pub fn from_math(azimuth: f64) -> Result<f64, ()> {
    if !is_canonical(azimuth) {
        return Err(());
    }

    to_math(azimuth)
}

pub fn to_octant_str(azimuth: f64, n: &str, e: &str, s: &str, w: &str) -> Result<String, ()> {
    if !is_canonical(azimuth) {
        return Err(());
    }

    let inc = FRAC_PI_4;
    let val = ((azimuth / inc) + 0.5).floor() as i32;
    let arr: Vec<String> = vec![
        n.to_string(),
        format!("{}{}", n, e),
        e.to_string(),
        format!("{}{}", s, e),
        s.to_string(),
        format!("{}{}", s, w),
        w.to_string(),
        format!("{}{}", n, w),
    ];

    Ok(arr.get((val % 8) as usize).unwrap().to_string())
}

#[cfg(test)]
mod azimuth_tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use rand::Rng;

    /// didn't want to use nightly so I just copied the rust code from
    /// https://doc.rust-lang.org/src/core/num/f64.rs.html#769
    fn next_down(v: f64) -> f64 {
        // We must use strictly integer arithmetic to prevent denormals from
        // flushing to zero after an arithmetic operation on some platforms.
        const NEG_TINY_BITS: u64 = 0x8000_0000_0000_0001; // Smallest (in magnitude) negative f64.
        const CLEAR_SIGN_MASK: u64 = 0x7fff_ffff_ffff_ffff;

        let bits = v.to_bits();
        if v.is_nan() || bits == f64::NEG_INFINITY.to_bits() {
            return v;
        }

        let abs = bits & CLEAR_SIGN_MASK;
        let next_bits = if abs == 0 {
            NEG_TINY_BITS
        } else if bits == abs {
            bits - 1
        } else {
            bits + 1
        };

        f64::from_bits(next_bits)
    }

    #[test]
    fn is_canonical_true_for_0() {
        assert!(is_canonical(0.));
    }

    #[test]
    fn is_canonical_false_for_0_pred() {
        assert!(!(is_canonical(next_down(0.0))));
    }

    #[test]
    fn is_canonical_true_for_2pi_pred() {
        assert!(is_canonical(next_down(TAU)));
    }

    #[test]
    fn is_canonical_false_for_2pi() {
        assert!(!(is_canonical(TAU)));
    }

    #[test]
    fn is_canonical_is_true_for_random_canonical_azimuths() {
        let mut rng = rand::thread_rng();

        for _ in 0..500 {
            assert!(is_canonical(rng.gen::<f64>() * TAU));
        }
    }

    #[test]
    fn canonicalize_works_on_rounded_random_angles() {
        let mut rng = rand::thread_rng();

        for _ in 0..500 {
            let a_deg = rng.gen_range(-5000..5000);
            let a_rad = (a_deg as f64).to_radians();
            let canonical_rad = canonicalize(a_rad);
            assert!((0.0..TAU).contains(&canonical_rad));

            let mut cannonical_a_deg = canonical_rad.to_degrees().round() as i32;
            if cannonical_a_deg == 360 {
                cannonical_a_deg = 0;
            }
            assert_eq!(cannonical_a_deg.rem_euclid(360), cannonical_a_deg);
        }
    }

    #[test]
    fn to_math_correctly_handles_0() {
        assert_approx_eq!(0., to_math(0.).unwrap(), 1e-10);
    }

    #[test]
    fn from_math_correctly_handles_0() {
        assert_approx_eq!(0., from_math(0.).unwrap(), 1e-10);
    }

    #[test]
    fn to_math_works_for_known_values() {
        let vs: Vec<f64> = vec![0., 0., 90., 270., 180., 180., 270., 90.];
        for i in (0..vs.len()).step_by(2) {
            let a = to_math(vs[i].to_radians()).unwrap();
            assert_approx_eq!(vs[i + 1].to_radians(), a, 1e-10);
        }
    }

    #[test]
    fn from_math_works_for_known_values() {
        let vs: Vec<f64> = vec![0., 0., 90., 270., 180., 180., 270., 90.];
        for i in (0..vs.len()).step_by(2) {
            let a = from_math(vs[i].to_radians()).unwrap();
            assert_approx_eq!(vs[i + 1].to_radians(), a, 1e-10);
        }
    }

    #[test]
    fn to_math_and_from_math_are_reversible() {
        let mut rng = rand::thread_rng();

        for _ in 0..500 {
            let a1 = rng.gen::<f64>() * TAU;
            let a2 = from_math(to_math(a1).unwrap()).unwrap();
            assert_approx_eq!(a1, a2, 1e-10);

            let a3 = to_math(from_math(a1).unwrap()).unwrap();
            assert_approx_eq!(a1, a3, 1e-10);
        }
    }

    #[test]
    fn to_math_error_for_2pi() {
        match to_math(TAU) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn from_math_error_for_2pi() {
        match from_math(TAU) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn to_octant_str_error_for_non_cannonical_azimuth() {
        match to_octant_str(-1., "", "", "", "") {
            Err(_) => assert!(true),
            Ok(_) => assert!(false),
        }
    }

    #[test]
    fn to_octant_str_correctly_cycle_through_values() {
        let n = "north";
        let e = "east";
        let s = "south";
        let w = "west";
        let mut expected: Vec<String> = vec![];
    }
}
