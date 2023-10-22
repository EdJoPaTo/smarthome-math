use crate::angle_distance::angle_distance;

/// `f32::abs` requires `std`
fn abs(n: f32) -> f32 {
    if n.is_sign_positive() {
        n
    } else {
        -n
    }
}

/// use `rem_euclid` instead but it requires `std`
pub(crate) fn normalize_hue(hue: f32) -> f32 {
    let hue = hue % 360.0;
    if hue.is_sign_negative() {
        hue + 360.0
    } else {
        hue
    }
}

#[must_use]
pub fn approach_linear(current: f32, target: f32, step_size: f32) -> f32 {
    let distance = target - current;
    if distance.is_nan() || abs(distance) <= step_size {
        return target;
    }

    if distance.is_sign_positive() {
        current + step_size
    } else {
        current - step_size
    }
    .clamp(0.0, 100.0)
}

#[test]
fn approach_linear_positive() {
    float_eq::assert_float_eq!(51.0, approach_linear(50.0, 100.0, 1.0), abs <= 0.1);
}

#[test]
fn approach_linear_negative() {
    float_eq::assert_float_eq!(49.0, approach_linear(50.0, 0.0, 1.0), abs <= 0.1);
}

#[test]
fn approach_linear_nan() {
    float_eq::assert_float_eq!(50.0, approach_linear(f32::NAN, 50.0, 1.0), abs <= 0.1);
}

#[test]
fn approach_linear_infinity() {
    float_eq::assert_float_eq!(100.0, approach_linear(f32::INFINITY, 0.0, 1.0), abs <= 0.1);
}

#[test]
fn approach_linear_negative_infinity() {
    float_eq::assert_float_eq!(
        0.0,
        approach_linear(f32::NEG_INFINITY, 100.0, 1.0),
        abs <= 0.1
    );
}

#[must_use]
pub fn approach_hue(current: f32, target: f32, step_size: f32) -> f32 {
    let distance = angle_distance(current, target);
    if distance.is_nan() || abs(distance) <= step_size {
        return target;
    }

    let next = if distance.is_sign_positive() {
        current + step_size
    } else {
        current - step_size
    };
    normalize_hue(next)
}

#[test]
fn approach_hue_positive() {
    float_eq::assert_float_eq!(approach_hue(0.0, 10.0, 1.0), 1.0, abs <= 0.1);
}

#[test]
fn approach_hue_negative() {
    float_eq::assert_float_eq!(approach_hue(10.0, 0.0, 1.0), 9.0, abs <= 0.1);
}

#[test]
fn approach_hue_positive_over_zero() {
    float_eq::assert_float_eq!(approach_hue(359.0, 10.0, 1.0), 0.0, abs <= 0.1);
}

#[test]
fn approach_hue_negative_over_zero() {
    float_eq::assert_float_eq!(approach_hue(0.0, 350.0, 1.0), 359.0, abs <= 0.1);
}

#[test]
fn approach_hue_infinity() {
    float_eq::assert_float_eq!(approach_hue(f32::INFINITY, 90.0, 1.0), 90.0, abs <= 0.1);
}

#[test]
fn approach_hue_negative_infinity() {
    float_eq::assert_float_eq!(approach_hue(f32::NEG_INFINITY, 90.0, 1.0), 90.0, abs <= 0.1);
}

#[test]
fn approach_hue_nan() {
    float_eq::assert_float_eq!(approach_hue(f32::NAN, 90.0, 1.0), 90.0, abs <= 0.1);
}
