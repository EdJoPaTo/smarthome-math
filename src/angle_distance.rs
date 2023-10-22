#[must_use]
pub fn angle_distance(start: f32, end: f32) -> f32 {
    let difference = end - start;
    let difference = difference % 360.0;
    if difference < -180.0 {
        difference + 360.0
    } else if difference > 180.0 {
        difference - 360.0
    } else {
        difference
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive() {
        float_eq::assert_float_eq!(angle_distance(0.0, 10.0), 10.0, abs <= 0.1);
    }

    #[test]
    fn negative() {
        float_eq::assert_float_eq!(angle_distance(10.0, 0.0), -10.0, abs <= 0.1);
    }

    #[test]
    fn positive_over_0() {
        float_eq::assert_float_eq!(angle_distance(-10.0, 20.0), 30.0, abs <= 0.1);
    }

    #[test]
    fn negative_over_0() {
        float_eq::assert_float_eq!(angle_distance(20.0, -10.0), -30.0, abs <= 0.1);
    }

    #[test]
    fn positive_over_0_with_positive_degree() {
        float_eq::assert_float_eq!(angle_distance(350.0, 20.0), 30.0, abs <= 0.1);
    }

    #[test]
    fn negative_over_0_with_positive_degree() {
        float_eq::assert_float_eq!(angle_distance(20.0, 350.0), -30.0, abs <= 0.1);
    }
}
