use std::str::FromStr;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

static DATETIME_FMT_LIST: [&str; 7] = [
    "%Y-%m-%d %H:%M:%S",
    "%Y-%m-%d %H:%M",
    "%Y%m%d%H%M%S",
    "%Y%m%d%H%M",
    "%Y%m%d%H",
    "%Y/%m/%d %H:%M:%S",
    "%Y/%m/%d %H:%M",
];

static DATE_FMT_LIST: [&str; 6] = [
    "%Y-%m-%d",
    "%Y-%m",
    "%Y%m%d",
    "%Y%m",
    "%Y/%m/%d",
    "%Y/%m",
];

pub fn parse_datetime(datetime: &str) -> Option<NaiveDateTime> {
    if let Ok(v) = NaiveDateTime::from_str(datetime) {
        return Some(v);
    };
    for fmt in DATETIME_FMT_LIST {
        if let Ok(v) = NaiveDateTime::parse_from_str(datetime, fmt) {
            return Some(v);
        }
    }
    for fmt in DATE_FMT_LIST {
        if let Ok(v) = NaiveDate::parse_from_str(datetime, fmt) {
            let zero_time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
            return Some(v.and_time(zero_time));
        }
    }
    None
}
