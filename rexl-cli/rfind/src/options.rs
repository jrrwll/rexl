use rexl::text::is_number_char;
use std::fmt;
use std::fmt::{Display, Formatter};

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

/// B, KB, K, M, MB, G, GB
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SizeOption {
    Eq, // =
    Ne, // !=, <>
    Ge, // >=
    Gt, // >
    Le, // <=
    Lt, // <
}

impl SizeOption {
    pub fn matched(&self, a: u64, b: u64) -> bool {
        match self {
            SizeOption::Eq => a == b,
            SizeOption::Ne => a != b,
            SizeOption::Ge => a >= b,
            SizeOption::Gt => a > b,
            SizeOption::Le => a <= b,
            SizeOption::Lt => a < b,
        }
    }

    pub fn parse(s: &str) -> Vec<Option<(Self, u64)>> {
        s.split(',').map(|i| Self::parse_one(i)).collect()
    }

    pub fn parse_one(s: &str) -> Option<(Self, u64)> {
        if s.starts_with(">=") {
            SizeOption::Ge.parse_one_by(s)
        } else if s.starts_with(">") {
            SizeOption::Gt.parse_one_by(s)
        } else if s.starts_with("<=") {
            SizeOption::Le.parse_one_by(s)
        } else if s.starts_with("<>") {
            SizeOption::Ne.parse_one_by(s)
        } else if s.starts_with("<") {
            SizeOption::Lt.parse_one_by(s)
        } else if s.starts_with("!=") {
            SizeOption::Ne.parse_one_by(s)
        } else if s.starts_with("=") {
            SizeOption::Eq.parse_one_by(s)
        } else {
            None
        }
    }

    fn parse_one_by(self, s: &str) -> Option<(Self, u64)> {
        if s.len() == 2 {
            return None;
        }
        let s = &s[2..];
        let size = s.len();
        let c = &s[(size - 1)..];
        if c == "B" {
            SizeOption::Ge.parse_unit(&s[..(size - 1)], c)
        } else {
            SizeOption::Ge.parse_unit(s, c)
        }
    }

    fn parse_unit(self, s: &str, c: &str) -> Option<(Self, u64)> {
        let size = s.len();
        if c == "K" {
            self.parse_num(&s[..(size - 1)], 1000.0)
        } else if c == "M" {
            self.parse_num(&s[..(size - 1)], 1000_000.0)
        } else if c == "G" {
            self.parse_num(&s[..(size - 1)], 1000_000_000.0)
        } else if is_number_char(c.chars().next().unwrap()) {
            self.parse_num(s, 1.0)
        } else {
            None
        }
    }

    #[inline]
    fn parse_num(self, s: &str, base: f64) -> Option<(Self, u64)> {
        s.parse::<f64>()
            .map(|n| Some((self, (n * base) as u64)))
            .unwrap_or_else(|_err| None)
    }
}

pub enum TimeOption {}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Options {
    Version,
    Help,
    Verbose,

    All,
    Depth,
    Kind,
    Name,
    NamePattern,
    Size,
    // AccessTime,
    // ModifyTime,
    // ChangeTime,
    Content,
    ContentPattern,
}

impl Display for Options {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
