use crate::{Duration, Time};
use std::time;

#[test]
fn from_nanos_works() {
    let time = Time::from_nanos(123456789987654321);
    assert_eq!(time.sec, 123456789);
    assert_eq!(time.nsec, 987654321);
    let time = Duration::from_nanos(123456789987654321);
    assert_eq!(time.sec, 123456789);
    assert_eq!(time.nsec, 987654321);
    let time = Duration::from_nanos(-123456789987654321);
    assert_eq!(time.sec, -123456789);
    assert_eq!(time.nsec, -987654321);
}

#[test]
fn nanos_works() {
    let time = Time {
        sec: 123456789,
        nsec: 987654321,
    };
    assert_eq!(time.nanos(), 123456789987654321);
    let time = Duration {
        sec: 123456789,
        nsec: 987654321,
    };
    assert_eq!(time.nanos(), 123456789987654321);
}

#[test]
fn duration_works_with_negative() {
    let time = Duration::from_nanos(-123456789987654321);
    assert_eq!(time.sec, -123456789);
    assert_eq!(time.nsec, -987654321);
    assert_eq!(time.nanos(), -123456789987654321);
}

#[test]
fn duration_from_std_works() {
    let std_duration = time::Duration::new(123, 456);
    let msg_duration = Duration::from(std_duration);
    assert_eq!(msg_duration.sec, 123);
    assert_eq!(msg_duration.nsec, 456);

    let std_duration2 = time::Duration::new(9876, 54321);
    let msg_duration2: Duration = std_duration2.into();
    assert_eq!(msg_duration2.sec, 9876);
    assert_eq!(msg_duration2.nsec, 54321);
}

#[test]
fn duration_to_std_works() {
    let msg_duration = Duration {
        sec: 123,
        nsec: 456,
    };
    let std_duration = time::Duration::from(msg_duration);
    assert_eq!(std_duration.as_secs(), 123);
    assert_eq!(std_duration.subsec_nanos(), 456);

    let msg_duration2 = Duration {
        sec: 9876,
        nsec: 54321,
    };
    let std_duration2: time::Duration = msg_duration2.into();
    assert_eq!(std_duration2.as_secs(), 9876);
    assert_eq!(std_duration2.subsec_nanos(), 54321);
}

#[test]
fn time_from_std_works() {
    let std_time = time::SystemTime::UNIX_EPOCH + time::Duration::new(123, 456);
    let msg_time = Time::from(std_time);
    assert_eq!(msg_time.sec, 123);
    assert_eq!(msg_time.nsec, 456);

    let std_time2 = time::SystemTime::UNIX_EPOCH + time::Duration::new(9876, 54321);
    let msg_time2: Time = std_time2.into();
    assert_eq!(msg_time2.sec, 9876);
    assert_eq!(msg_time2.nsec, 54321);
}

#[test]
fn time_to_std_works() {
    let msg_time = Time {
        sec: 123,
        nsec: 456,
    };
    let std_time = time::SystemTime::from(msg_time);
    assert_eq!(
        (std_time.duration_since(time::SystemTime::UNIX_EPOCH))
            .unwrap()
            .as_secs(),
        123
    );
    assert_eq!(
        (std_time.duration_since(time::SystemTime::UNIX_EPOCH))
            .unwrap()
            .subsec_nanos(),
        456
    );

    let msg_time2 = Time {
        sec: 9876,
        nsec: 54321,
    };
    let std_time2: time::SystemTime = msg_time2.into();
    assert_eq!(
        (std_time2.duration_since(time::SystemTime::UNIX_EPOCH))
            .unwrap()
            .as_secs(),
        9876
    );
    assert_eq!(
        (std_time2.duration_since(time::SystemTime::UNIX_EPOCH))
            .unwrap()
            .subsec_nanos(),
        54321
    );
}

#[test]
fn display_zero() {
    let time = Time::from_nanos(0);
    assert_eq!(format!("{}", time), "0.000000000");
    let time = Duration::from_nanos(0);
    assert_eq!(format!("{}", time), "0.000000000");
}

#[test]
fn display_full() {
    let time = Time::from_nanos(123456789987654321);
    assert_eq!(format!("{}", time), "123456789.987654321");
    let time = Duration::from_nanos(123456789987654321);
    assert_eq!(format!("{}", time), "123456789.987654321");
    let time = Duration::from_nanos(-123456789987654321);
    assert_eq!(format!("{}", time), "-123456789.987654321");
}

#[test]
fn display_trailing_zeros() {
    let time = Time::from_nanos(123456789987654321);
    assert_eq!(format!("{}", time), "123456789.987654321");
    let time = Time::from_nanos(123456789987654000);
    assert_eq!(format!("{}", time), "123456789.987654000");
    let time = Time::from_nanos(123456789000000000);
    assert_eq!(format!("{}", time), "123456789.000000000");
    let time = Time::from_nanos(123456700000000000);
    assert_eq!(format!("{}", time), "123456700.000000000");

    let time = Duration::from_nanos(-123456789987654321);
    assert_eq!(format!("{}", time), "-123456789.987654321");
    let time = Duration::from_nanos(-123456789987654000);
    assert_eq!(format!("{}", time), "-123456789.987654000");
    let time = Duration::from_nanos(-123456789000000000);
    assert_eq!(format!("{}", time), "-123456789.000000000");
    let time = Duration::from_nanos(-123456700000000000);
    assert_eq!(format!("{}", time), "-123456700.000000000");

    let time = Duration::from_nanos(-123456789987654321);
    assert_eq!(format!("{}", time), "-123456789.987654321");
    let time = Duration::from_nanos(-123456789987654000);
    assert_eq!(format!("{}", time), "-123456789.987654000");
    let time = Duration::from_nanos(-123456789000000000);
    assert_eq!(format!("{}", time), "-123456789.000000000");
    let time = Duration::from_nanos(-123456700000000000);
    assert_eq!(format!("{}", time), "-123456700.000000000");
}

#[test]
fn display_decimals() {
    let time = Time::from_nanos(9987654321);
    assert_eq!(format!("{}", time), "9.987654321");
    let time = Time::from_nanos(987654321);
    assert_eq!(format!("{}", time), "0.987654321");
    let time = Time::from_nanos(654321);
    assert_eq!(format!("{}", time), "0.000654321");
    let time = Time::from_nanos(9987654000);
    assert_eq!(format!("{}", time), "9.987654000");
    let time = Time::from_nanos(987654000);
    assert_eq!(format!("{}", time), "0.987654000");
    let time = Time::from_nanos(654000);
    assert_eq!(format!("{}", time), "0.000654000");

    let time = Duration::from_nanos(-9987654321);
    assert_eq!(format!("{}", time), "-9.987654321");
    let time = Duration::from_nanos(-987654321);
    assert_eq!(format!("{}", time), "-0.987654321");
    let time = Duration::from_nanos(-654321);
    assert_eq!(format!("{}", time), "-0.000654321");
    let time = Duration::from_nanos(-9987654000);
    assert_eq!(format!("{}", time), "-9.987654000");
    let time = Duration::from_nanos(-987654000);
    assert_eq!(format!("{}", time), "-0.987654000");
    let time = Duration::from_nanos(-654000);
    assert_eq!(format!("{}", time), "-0.000654000");

    let time = Duration::from_nanos(-9987654321);
    assert_eq!(format!("{}", time), "-9.987654321");
    let time = Duration::from_nanos(-987654321);
    assert_eq!(format!("{}", time), "-0.987654321");
    let time = Duration::from_nanos(-654321);
    assert_eq!(format!("{}", time), "-0.000654321");
    let time = Duration::from_nanos(-9987654000);
    assert_eq!(format!("{}", time), "-9.987654000");
    let time = Duration::from_nanos(-987654000);
    assert_eq!(format!("{}", time), "-0.987654000");
    let time = Duration::from_nanos(-654000);
    assert_eq!(format!("{}", time), "-0.000654000");

    let time = Duration { sec: -1, nsec: 1 };
    assert_eq!(format!("{}", time), "-0.999999999");
    let time = Duration { sec: -1, nsec: -1 };
    assert_eq!(format!("{}", time), "-1.000000001");
}
