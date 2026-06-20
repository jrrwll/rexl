use rexl::text::is_number_char;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Debug, Display};
use std::str::FromStr;

pub fn parse_kind(v: Vec<String>) -> Result<u8, String> {
    let mut kind = 0;
    if v.is_empty() {
        return Ok(0b1111_1111);
    }
    for s in v.iter() {
        if s == "f" || s == "file" {
            kind |= 0b0000_0001;
        } else if s == "d" || s == "dir" || s == "directory" {
            kind |= 0b0000_0010;
        } else if s == "l" || s == "link" {
            kind |= 0b0000_0100;
        } else {
            return Err(s.to_string());
        }
    }
    Ok(kind)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CompareOp {
    Eq, // =
    Ne, // !=, <>
    Ge, // >=
    Gt, // >
    Le, // <=
    Lt, // <
}

impl Display for CompareOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompareOp::Eq => write!(f, "="),
            CompareOp::Ne => write!(f, "!="),
            CompareOp::Ge => write!(f, ">="),
            CompareOp::Gt => write!(f, ">"),
            CompareOp::Le => write!(f, "<="),
            CompareOp::Lt => write!(f, "<"),
        }
    }
}

const SIZE_COMPARE_ERROR: &str = "invalid size-compare exr";

#[derive(Copy, Clone, Debug)]
pub struct SizeCompare {
    compare: CompareOp,
    length: u64, // in Bytes
}

impl FromStr for SizeCompare {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 || (s.len() == 2 && !s.starts_with("=")) {
            return Err(SIZE_COMPARE_ERROR.to_string());
        }
        if s.starts_with(">=") {
            Self::parse_one_by(&s[2..], CompareOp::Ge)
        } else if s.starts_with(">") {
            Self::parse_one_by(&s[2..], CompareOp::Gt)
        } else if s.starts_with("<=") {
            Self::parse_one_by(&s[2..], CompareOp::Le)
        } else if s.starts_with("<") {
            Self::parse_one_by(&s[2..], CompareOp::Lt)
        } else if s.starts_with("!=") || s.starts_with("<>") {
            Self::parse_one_by(&s[2..], CompareOp::Ne)
        } else if s.starts_with("=") {
            Self::parse_one_by(&s[1..], CompareOp::Eq)
        } else {
            return Err(SIZE_COMPARE_ERROR.to_string());
        }
    }
}

impl SizeCompare {
    pub fn matched(&self, b: u64) -> bool {
        let a = self.length;
        match self.compare {
            CompareOp::Eq => a == b,
            CompareOp::Ne => a != b,
            CompareOp::Ge => a >= b,
            CompareOp::Gt => a > b,
            CompareOp::Le => a <= b,
            CompareOp::Lt => a < b,
        }
    }

    fn parse_one_by(s: &str, options: CompareOp) -> Result<Self, String> {
        let size = s.len();
        let c = &s[(size - 1)..];

        let length = parse_unit(s, c).ok_or(SIZE_COMPARE_ERROR.to_string())?;
        Ok(Self { compare: options, length })
    }
}

fn parse_unit(s: &str, c: &str) -> Option<u64> {
    let size = s.len();
    if c == "K" {
        parse_num(&s[..(size - 1)], 1000.0)
    } else if c == "M" {
        parse_num(&s[..(size - 1)], 1000_000.0)
    } else if c == "G" {
        parse_num(&s[..(size - 1)], 1000_000_000.0)
    } else if c == "B" {
        parse_num(&s[..(size - 1)], 1.0)
    } else if is_number_char(c.chars().next().unwrap()) {
        parse_num(s, 1.0)
    } else {
        None
    }
}

#[inline]
fn parse_num(s: &str, base: f64) -> Option<u64> {
    s.parse::<f64>()
        .map(|n| Some((n * base) as u64))
        .unwrap_or_else(|_err| None)
}

impl Serialize for SizeCompare {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let s = format!("{}{}", self.compare, self.length);
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for SizeCompare {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        SizeCompare::from_str(&s).map_err(de::Error::custom)
    }
}
