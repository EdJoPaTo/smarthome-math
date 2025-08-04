use crate::angle_distance;
use crate::light::normalize_hue;

#[derive(Clone, Copy)]
pub struct Hsv {
    /// Hue from 0.0 to 360.0
    pub hue: f32,
    /// Saturation from 0.0 to 100.0
    pub saturation: f32,
    /// Brightness / Value from 0.0 to 100.0
    pub brightness: f32,
}

impl Hsv {
    #[must_use]
    pub const fn from_hue(hue: f32) -> Self {
        Self {
            hue,
            saturation: 100.0,
            brightness: 100.0,
        }
    }

    #[must_use]
    fn calculate_distance_to(&self, target: &Self) -> Self {
        Self {
            hue: angle_distance(self.hue, target.hue),
            saturation: target.saturation - self.saturation,
            brightness: target.brightness - self.brightness,
        }
    }

    #[expect(clippy::suboptimal_flops, reason = "requires std")]
    #[must_use]
    pub fn calculate_interpolated(start: &Self, end: &Self, position: f32) -> Self {
        if position <= 0.0 {
            *start
        } else if position >= 1.0 {
            *end
        } else {
            let distances = start.calculate_distance_to(end);
            Self {
                hue: normalize_hue((distances.hue * position) + start.hue),
                saturation: (distances.saturation * position) + start.saturation,
                brightness: (distances.brightness * position) + start.brightness,
            }
        }
    }

    /// Converts to `u8` RGB values
    #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    #[cfg(feature = "bracket-color")]
    #[must_use]
    pub fn to_rgb_u8(self) -> (u8, u8, u8) {
        let hsv = bracket_color::hsv::HSV::from_f32(
            self.hue / 360.0,
            self.saturation / 100.0,
            self.brightness / 100.0,
        );
        let rgb = hsv.to_rgb();
        let red = (rgb.r * 255.0) as u8;
        let green = (rgb.g * 255.0) as u8;
        let blue = (rgb.b * 255.0) as u8;
        (red, green, blue)
    }
}

#[cfg(test)]
mod tests {
    use super::Hsv;

    #[test]
    fn before_zero() {
        let start = Hsv {
            hue: 0.0,
            saturation: 0.0,
            brightness: 0.0,
        };
        let end = Hsv {
            hue: 10.0,
            saturation: 10.0,
            brightness: 10.0,
        };
        let result = Hsv::calculate_interpolated(&start, &end, -1.0);
        float_eq::assert_float_eq!(result.hue, 0.0, abs <= 0.1);
        float_eq::assert_float_eq!(result.saturation, 0.0, abs <= 0.1);
        float_eq::assert_float_eq!(result.brightness, 0.0, abs <= 0.1);
    }

    #[test]
    fn zero() {
        let start = Hsv {
            hue: 0.0,
            saturation: 0.0,
            brightness: 0.0,
        };
        let end = Hsv {
            hue: 10.0,
            saturation: 10.0,
            brightness: 10.0,
        };
        let result = Hsv::calculate_interpolated(&start, &end, 0.0);
        float_eq::assert_float_eq!(result.hue, 0.0, abs <= 0.1);
        float_eq::assert_float_eq!(result.saturation, 0.0, abs <= 0.1);
        float_eq::assert_float_eq!(result.brightness, 0.0, abs <= 0.1);
    }

    #[test]
    fn one() {
        let start = Hsv {
            hue: 0.0,
            saturation: 0.0,
            brightness: 0.0,
        };
        let end = Hsv {
            hue: 10.0,
            saturation: 10.0,
            brightness: 10.0,
        };
        let result = Hsv::calculate_interpolated(&start, &end, 1.0);
        float_eq::assert_float_eq!(result.hue, 10.0, abs <= 0.1);
        float_eq::assert_float_eq!(result.saturation, 10.0, abs <= 0.1);
        float_eq::assert_float_eq!(result.brightness, 10.0, abs <= 0.1);
    }

    #[test]
    fn quarter() {
        let start = Hsv {
            hue: 0.0,
            saturation: 0.0,
            brightness: 0.0,
        };
        let end = Hsv {
            hue: 10.0,
            saturation: 10.0,
            brightness: 10.0,
        };
        let result = Hsv::calculate_interpolated(&start, &end, 0.25);
        float_eq::assert_float_eq!(result.hue, 2.5, abs <= 0.1);
        float_eq::assert_float_eq!(result.saturation, 2.5, abs <= 0.1);
        float_eq::assert_float_eq!(result.brightness, 2.5, abs <= 0.1);
    }

    #[test]
    fn after_one() {
        let start = Hsv {
            hue: 0.0,
            saturation: 0.0,
            brightness: 0.0,
        };
        let end = Hsv {
            hue: 10.0,
            saturation: 10.0,
            brightness: 10.0,
        };
        let result = Hsv::calculate_interpolated(&start, &end, 2.0);
        float_eq::assert_float_eq!(result.hue, 10.0, abs <= 0.1);
        float_eq::assert_float_eq!(result.saturation, 10.0, abs <= 0.1);
        float_eq::assert_float_eq!(result.brightness, 10.0, abs <= 0.1);
    }
}

#[cfg(all(test, feature = "bracket-color"))]
mod rgb_tests {
    use super::*;

    fn hsv_to_rgb(hue: f32, saturation: f32, brightness: f32) -> (u8, u8, u8) {
        Hsv {
            hue,
            saturation,
            brightness,
        }
        .to_rgb_u8()
    }

    #[test]
    fn everything_zero_is_black() {
        let (red, green, blue) = hsv_to_rgb(0.0, 0.0, 0.0);
        assert_eq!(red, 0);
        assert_eq!(green, 0);
        assert_eq!(blue, 0);
    }

    #[test]
    fn sat_0_brightness_100_is_white() {
        let (red, green, blue) = hsv_to_rgb(0.0, 0.0, 100.0);
        assert_eq!(red, 255);
        assert_eq!(green, 255);
        assert_eq!(blue, 255);
    }

    #[test]
    fn sat_0_brightness_50_is_everything_half() {
        let (red, green, blue) = hsv_to_rgb(0.0, 0.0, 50.0);
        assert_eq!(red, 127);
        assert_eq!(green, 127);
        assert_eq!(blue, 127);
    }

    #[test]
    fn brightness_1_is_visible() {
        let (red, green, blue) = hsv_to_rgb(0.0, 0.0, 1.0);
        assert!(red > 0);
        assert!(green > 0);
        assert!(blue > 0);
    }

    #[test]
    fn hue_0_is_red() {
        let (red, green, blue) = hsv_to_rgb(0.0, 100.0, 100.0);
        assert_eq!(red, 255);
        assert_eq!(green, 0);
        assert_eq!(blue, 0);
    }

    #[test]
    fn hue_120_is_green() {
        let (red, green, blue) = hsv_to_rgb(120.0, 100.0, 100.0);
        assert_eq!(red, 0);
        assert_eq!(green, 255);
        assert_eq!(blue, 0);
    }

    #[test]
    fn hue_240_is_blue() {
        let (red, green, blue) = hsv_to_rgb(240.0, 100.0, 100.0);
        assert_eq!(red, 0);
        assert_eq!(green, 0);
        assert_eq!(blue, 255);
    }

    #[test]
    fn hue_360_is_red() {
        let (red, green, blue) = hsv_to_rgb(360.0, 100.0, 100.0);
        assert_eq!(red, 255);
        assert_eq!(green, 0);
        assert_eq!(blue, 0);
    }

    #[test]
    fn hue_negative_360_is_red() {
        let (red, green, blue) = hsv_to_rgb(-360.0, 100.0, 100.0);
        assert_eq!(red, 255);
        assert_eq!(green, 0);
        assert_eq!(blue, 0);
    }
}
