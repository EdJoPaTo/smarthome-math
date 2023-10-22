use core::time::Duration;

use chrono::{NaiveTime, Timelike};

pub const SECONDS_IN_MINUTE: u32 = 60;
pub const SECONDS_IN_HOUR: u32 = SECONDS_IN_MINUTE * 60;
pub const SECONDS_IN_DAY: u32 = SECONDS_IN_HOUR * 24;

#[allow(clippy::cast_possible_truncation)]
#[must_use]
pub fn minutes_from_midnight(time: NaiveTime) -> u16 {
    let minutes = time.num_seconds_from_midnight() / SECONDS_IN_MINUTE;
    minutes as u16
}

#[must_use]
pub fn calc_hue(time: NaiveTime) -> u16 {
    let minutes_of_day = minutes_from_midnight(time);
    minutes_of_day.rem_euclid(360)
}

#[must_use]
pub fn duration_until_next_full_minute(time: NaiveTime) -> Duration {
    let remaining_seconds = u64::from(59_u32.saturating_sub(time.second()));
    let remaining_nanos = 1_000_000_000_u32.saturating_sub(time.nanosecond());
    Duration::new(remaining_seconds, remaining_nanos)
}

#[must_use]
pub fn duration_until_next_full_second(time: NaiveTime) -> Duration {
    let remaining_nanos = 1_000_000_000_u32.saturating_sub(time.nanosecond());
    Duration::new(0, remaining_nanos)
}

#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn duration_until(now: NaiveTime, target: NaiveTime) -> Duration {
    let mut d = target - now;
    if d.num_seconds() < 0 {
        d = d + chrono::Duration::days(1);
    }
    d.to_std().expect("duration_until always wraps around")
}

#[test]
fn minutes_from_midnight_example() {
    assert_eq!(
        minutes_from_midnight(NaiveTime::from_hms_opt(0, 30, 0).unwrap()),
        30
    );
    assert_eq!(
        minutes_from_midnight(NaiveTime::from_hms_opt(6, 0, 0).unwrap()),
        360
    );
}

#[test]
fn hue_example() {
    assert_eq!(calc_hue(NaiveTime::from_hms_opt(0, 30, 0).unwrap()), 30);
    assert_eq!(calc_hue(NaiveTime::from_hms_opt(6, 0, 0).unwrap()), 0);
}

#[test]
fn duration_till_next_full_minute_example() {
    let time = NaiveTime::from_hms_milli_opt(3, 13, 57, 500).unwrap();
    let remaining = duration_until_next_full_minute(time);
    assert_eq!(remaining, Duration::new(2, 500_000_000));
}

#[test]
fn duration_till_next_full_second_example() {
    let time = NaiveTime::from_hms_milli_opt(3, 13, 57, 500).unwrap();
    let remaining = duration_until_next_full_second(time);
    assert_eq!(remaining, Duration::new(0, 500_000_000));
}

#[test]
fn duration_until_direct() {
    let d = duration_until(
        NaiveTime::from_hms_opt(10, 30, 0).unwrap(),
        NaiveTime::from_hms_opt(13, 37, 0).unwrap(),
    );
    let minutes = d.as_secs() / 60;
    #[cfg(feature = "std")]
    dbg!(d, minutes);
    assert_eq!(minutes, (3 * 60) + 7);
}

#[test]
fn duration_until_wraparound() {
    let d = duration_until(
        NaiveTime::from_hms_opt(23, 45, 0).unwrap(),
        NaiveTime::from_hms_opt(0, 15, 0).unwrap(),
    );
    let minutes = d.as_secs() / 60;
    #[cfg(feature = "std")]
    dbg!(d, minutes);
    assert_eq!(minutes, 30);
}

#[test]
fn duration_until_with_millis() {
    let d = duration_until(
        NaiveTime::from_hms_milli_opt(13, 36, 58, 500).unwrap(),
        NaiveTime::from_hms_opt(13, 37, 0).unwrap(),
    );
    assert_eq!(d, Duration::new(1, 500_000_000));
}
