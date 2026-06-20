use regex::Regex;
use serde::*;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct RegexStr(Regex);

impl FromStr for RegexStr {
    type Err = regex::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Regex::new(s).map(RegexStr)
    }
}

impl fmt::Display for RegexStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.as_str())
    }
}

impl std::ops::Deref for RegexStr {
    type Target = Regex;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for RegexStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for RegexStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        RegexStr::from_str(&s).map_err(de::Error::custom)
    }
}
