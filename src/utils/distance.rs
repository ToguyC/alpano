pub const EARTH_RADIUS: f64 = 6371000.0;

/// Convert a distance given in meters on the surface of the earth (arc's length)
/// to it's radians representation.
pub fn to_rad(dist_in_meters: f64) -> f64 {
    dist_in_meters / EARTH_RADIUS
}

/// Convert a radians to the distance equivalent on the earth surface (arc's length)
pub fn to_meter(rad: f64) -> f64 {
    EARTH_RADIUS * rad
}

#[cfg(test)]
mod distance_tests {
    use super::*;
    use rand::Rng;
    use assert_approx_eq::assert_approx_eq;

    const EARTH_CIRCUMFERENCE: f64 = 40_030_174.0;

    #[test]
    fn to_rad_and_to_meter_are_reversible() {
        let mut rng = rand::thread_rng();

        for _ in 0..500 {
            let rad = std::f64::consts::TAU * rng.gen::<f64>();
            let rad2 = to_rad(to_meter(rad));
            assert_approx_eq!(rad, rad2, 1e-10);
        }
    }

    #[test]
    fn to_meter_is_correct_for_known_values() {
        assert_approx_eq!(0., to_rad(0.));
        assert_approx_eq!(EARTH_CIRCUMFERENCE, to_meter(std::f64::consts::TAU), 0.5);
    }

    #[test]
    fn to_rad_is_correct_for_known_values() {
        assert_approx_eq!(0., to_meter(0.));
        assert_approx_eq!(std::f64::consts::TAU, to_rad(EARTH_CIRCUMFERENCE), 0.5);
    }
}
