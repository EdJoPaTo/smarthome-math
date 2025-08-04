#[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
#[must_use]
pub fn u8(start: u8, end: u8, position: f32) -> u8 {
    f32(f32::from(start), f32::from(end), position) as u8
}

#[must_use]
pub fn f32(start: f32, end: f32, position: f32) -> f32 {
    let length = end - start;
    let offset = length * position;
    start + offset
}

#[test]
fn u8_min() {
    assert_eq!(50, u8(50, 100, 0.0));
}

#[test]
fn u8_max() {
    assert_eq!(100, u8(50, 100, 1.0));
}

#[test]
fn u8_between() {
    assert_eq!(60, u8(50, 100, 0.2));
}
