use chrono::{DateTime, TimeZone};

/// Calculate the relative brightness of the given `NaiveDateTime` between 0.0 and 1.0
/// # Panics
/// Panics when the calculation result is not between 0.0 and 1.0 which indicates a code error.
#[allow(clippy::cast_precision_loss)]
#[must_use]
pub fn calc_relative_brightness_of_time<Tz: TimeZone>(
    datetime: &DateTime<Tz>,
    latitude: f64,
    longitude: f64,
    height: Option<f64>,
) -> f32 {
    let timestamp = datetime.timestamp_millis();
    let times = suncalc::get_times(suncalc::Timestamp(timestamp), latitude, longitude, height);

    // let pos = suncalc::get_position(suncalc::Timestamp(timestamp), latitude, longitude);
    // dbg!(&pos.altitude);

    // dbg!(chrono::Local.timestamp_millis_opt(times.dawn.0));
    // dbg!(chrono::Local.timestamp_millis_opt(times.solar_noon.0));
    // dbg!(chrono::Local.timestamp_millis_opt(times.dusk.0));

    let begin = times.dawn.0;
    let end = times.dusk.0;
    let noon = times.solar_noon.0;

    if begin == 0 || end == 0 {
        // Above polar circle. Time of date is irrelevant as its either 24h or 0h
        let pos = suncalc::get_position(suncalc::Timestamp(timestamp), latitude, longitude);
        return if pos.altitude > 0.0 { 1.0 } else { 0.0 };
    }

    if timestamp < begin || timestamp > end {
        return 0.0;
    }

    let max_distance = (noon - begin) as f32;
    let current_distance = (noon - timestamp).abs() as f32;
    let relative_distance = current_distance / max_distance;
    let brightness_factor = 1.0 - (relative_distance.powi(3));
    assert!(
        (0.0..=1.0).contains(&brightness_factor),
        "brightness_factor is not between 0.0 and 1.0: {brightness_factor}",
    );
    brightness_factor
}

#[cfg(test)]
fn test_relative_brightness(datetime: &str, expected: f32) {
    let datetime = DateTime::parse_from_rfc3339(datetime).unwrap();
    let relative_brightness = calc_relative_brightness_of_time(&datetime, 53.5, 10.0, Some(5.0));
    #[cfg(feature = "std")]
    dbg!(datetime, relative_brightness);
    float_eq::assert_float_eq!(expected, relative_brightness, abs <= 0.05);
}

#[test]
fn brightness_of_night() {
    test_relative_brightness("2021-01-01T02:00:00+01:00", 0.0);
}

#[test]
fn brightness_of_dawn() {
    test_relative_brightness("2023-02-17T05:36:00+01:00", 0.0);
}
#[test]
fn brightness_of_sunrise() {
    test_relative_brightness("2023-02-17T07:33:00+01:00", 0.3);
}
#[test]
fn brightness_of_noon() {
    test_relative_brightness("2023-02-17T12:32:00+01:00", 1.0);
}
#[test]
fn brightness_of_summer_noon() {
    test_relative_brightness("2024-06-20T13:22:00+02:00", 1.0);
}
#[test]
fn brightness_of_winter_noon() {
    test_relative_brightness("2022-12-21T12:19:00+01:00", 1.0);
}
#[test]
fn brightness_of_sunset() {
    test_relative_brightness("2023-02-17T17:32:00+01:00", 0.3);
}
#[test]
fn brightness_of_dusk() {
    test_relative_brightness("2023-02-17T19:29:00+01:00", 0.0);
}
