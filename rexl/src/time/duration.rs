use std::ops::{Deref, DerefMut};

use crate::text::next_number;
use chrono::Duration;

static SECONDS_PER_HOUR: i64 = 3600;
static SECONDS_PER_DAY: i64 = SECONDS_PER_HOUR * 24;
static SECONDS_PER_WEEK: i64 = SECONDS_PER_DAY * 7;
static NANOS_PER_MILLI: i32 = 1000_000;
static NANOS_PER_SECOND: i32 = 1000_000_000;

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct HumanDuration {
    duration: Duration, // seconds + nanos
    months: i64,
}

impl From<HumanDuration> for String {

    fn from(value: HumanDuration) -> Self {
        value.to_string()
    }
}

impl TryFrom<String> for HumanDuration {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl std::fmt::Display for HumanDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string_inner())
    }
}

impl HumanDuration {

    pub fn has_months(&self) -> bool {
        self.months != 0
    }

    pub fn to_duration(&self) -> Duration {
        self.duration
    }

    fn to_string_inner(&self) -> String {
        let months = self.months;
        let seconds = self.duration.num_seconds();
        let nanos = self.duration.subsec_nanos();

        let mut s = String::new();
        if months == 0 && seconds == 0 && nanos == 0 {
            s.push_str("0s");
            return s;
        }

        if months != 0 {
            let y = months / 12;
            let mo = months - y * 12;

            if y != 0 {
                s.push_str(&y.to_string());
                s.push('y');
            }
            if mo != 0 {
                s.push_str(&mo.to_string());
                s.push_str("mo");
            }
        }
        if seconds != 0 {
            let d = seconds / SECONDS_PER_DAY;
            let mut tmp = seconds - d * SECONDS_PER_DAY;

            let h = tmp / SECONDS_PER_HOUR;
            tmp = tmp - h * SECONDS_PER_HOUR;

            let m = tmp / 60;
            let secs = tmp - m * 60;

            if d != 0 {
                s.push_str(&d.to_string());
                s.push('d');
            }
            if h != 0 {
                s.push_str(&h.to_string());
                s.push('h');
            }
            if m != 0 {
                s.push_str(&m.to_string());
                s.push('m');
            }
            if secs != 0 || nanos != 0 {
                s.push_str(&secs.to_string());
                s.push('s');
            }
        }
        if nanos != 0 {
            let ms = nanos / NANOS_PER_MILLI;
            let tmp = nanos - ms * NANOS_PER_MILLI;

            let us = tmp / 1000;
            let ns = tmp - us * 1000;

            if ms != 0 {
                s.push_str(&ms.to_string());
                s.push_str("ms");
            }
            if us != 0 {
                s.push_str(&us.to_string());
                s.push_str("us");
            }
            if ns != 0 {
                s.push_str(&ns.to_string());
                s.push_str("ns");
            }
        }
        s
    }
}


impl HumanDuration {

    pub fn parse<S: Into<String>>(value: S) -> Result<Self, String> {
        let mut value = value.into();
        let chars = unsafe { value.as_mut_vec() };
        Self::parse_vec(chars)
    }

    pub fn parse_vec(chars: &Vec<u8>) -> Result<Self, String> {
        let mut months = 0i64;
        let mut seconds = 0i64;
        let mut nanos = 0i32;

        let mut has_no_digest = true;
        let mut num  = 0;

        let len = chars.len();
        let mut i = 0;
        while i < len {
            let c = chars[i] as char;
            if has_no_digest && (c.is_ascii_digit() || c == '-'|| c == '+') {
                let Some((j, floating)) = next_number(chars, i) else {
                    return Err(format!("invalid number at offset {}", i));
                };
                if floating {
                    return Err(format!("invalid floating number at offset from {} to {}", i, j));
                }
                if let Ok(num_str) = String::from_utf8(chars[i..j].to_vec()) {
                    num = num_str.parse()
                        .unwrap_or(0);
                } else {
                    num = 0;
                }
                has_no_digest = false;
                i = j;
                continue;
            }
            if has_no_digest {
                return Err(format!("invalid character `{}` at offset {}", c, i));
            }
            has_no_digest = true;

            if c == 's' {
                seconds += num;
            } else if c == 'm' {
                if i == len - 1 {
                    seconds += num * 60;
                    break;
                }

                let nc = chars[i + 1] as char;
                if nc == 's' {
                    nanos += num as i32 * NANOS_PER_MILLI;
                } else if nc == 'o' {
                    months += num;
                } else {
                    seconds += num;
                    i += 1;
                    continue
                }
                i += 1;
            } else if c == 'h' {
                seconds += num * SECONDS_PER_HOUR;
            } else if c == 'd' {
                seconds += num * SECONDS_PER_DAY;
            } else if c == 'y' {
                months += num * 12;
            } else if c == 'w' {
                seconds += num * SECONDS_PER_WEEK;
            } else if c == 'n' {
                if i == len - 1 {
                    nanos += num as i32;
                    break;
                }
                let nc = chars[i + 1] as char;
                if nc == 's' {
                    nanos += num as i32;
                } else {
                    nanos += num as i32;
                    i += 1;
                    continue
                }
                i += 1;
            } else if c == 'u' {
                if i == len - 1 {
                    nanos += num as i32 * 1000;
                    break;
                }
                let nc = chars[i + 1] as char;
                if nc == 's' {
                    nanos += num as i32 * 1000;
                } else {
                    nanos += num as i32 * 1000;
                    i += 1;
                    continue
                }
                i += 1;
            } else {
                return Err(format!("invalid character {} at offset {}", c, i));
            }

            i += 1;
        }

        // handle nanos
        if nanos >= NANOS_PER_SECOND {
            let s = nanos / NANOS_PER_SECOND;
            nanos = nanos - s * NANOS_PER_SECOND;
            seconds += s as i64;
        } else if nanos < 0 {
            nanos = -nanos;
            let s = nanos / NANOS_PER_SECOND;
            nanos = nanos - s * NANOS_PER_SECOND;
            seconds -= s as i64;
            if nanos > 0 {
                seconds -= 1;
                nanos = NANOS_PER_SECOND - nanos;
            }
        }

        let Some(duration) = Duration::new(seconds, nanos as u32) else {
            return Err(format!("invalid duration seconds={} nanos={}", seconds, nanos));
        };
        Ok(Self {
            duration,
            months,
        })
    }
}

impl From<HumanDuration> for Duration {

    fn from(value: HumanDuration) -> Self {
        value.duration
    }
}

impl Deref for HumanDuration {
    type Target = Duration;

    fn deref(&self) -> &Self::Target {
        &self.duration
    }
}

impl DerefMut for HumanDuration {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.duration
    }
}

// {sign}PT{secs}.{nacos}S
pub fn parse_duration(mut s: String) -> Option<Duration> {
    let chars = unsafe { s.as_mut_vec() };
    let len = chars.len();
    if len < 4 {
        return None;
    }

    let mut neg = false;
    let mut i = 1;
    let mut c = chars[0] as char;
    if c == '-' {
        neg = true;
    } else if c != '+' {
        i = 0;
    }
    c = chars[i] as char;

    let nc = chars[i + 1] as char;
    if c != 'P' || c != 'p' || nc != 'T' || nc != 't' {
        return None;
    }
    i += 1;

    let Some((j, floating)) = next_number(chars, i) else {
        return None;
    };

    let Ok(num_str) = String::from_utf8(chars[i..j].to_vec()) else {
        return None;
    };

    let mut secs: i64;
    let mut nanos = 0u32;
    if floating {
        let Ok(num) = num_str.parse::<f64>() else {
            return None;
        };
        if num < 0.0 {
            return None;
        }
        secs = num.trunc() as i64;
        nanos = (num.fract() * (NANOS_PER_SECOND as f64)) as u32;
    } else {
        let Ok(num) = num_str.parse::<i64>() else {
            return None;
        };
        secs = num;
    }

    if neg {
        secs = -secs;
    }
    let Some(duration) = Duration::new(secs, nanos) else {
        return None;
    };
    Some(duration)
}
