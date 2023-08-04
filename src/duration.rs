use std::error::Error;
use std::fmt::{self, Display};
use std::num::ParseIntError;
use std::ops::Add;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Duration {
    minutes: i64
}

impl Duration {
    pub fn minutes(minutes: i64) -> Self {
        Duration{minutes}
    }

    pub fn zero() -> Self {
        Duration{minutes: 0}
    }
}

impl Add for Duration {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::minutes(self.minutes + rhs.minutes)
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let h = self.minutes / 60;
        let m = self.minutes - h * 60;
        write!(f, "{}h{:02}", h, m)
    }
}


#[derive(Debug)]
pub enum ParseDurationError{
    WrongFormat,
    ParseError(ParseIntError),
}

impl Error for ParseDurationError {}

impl Display for ParseDurationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error")
    }
}



impl FromStr for Duration {

    type Err = ParseDurationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (h, m) = s.split_once("h").ok_or(ParseDurationError::WrongFormat)?;
        let h: i64 = h.parse().map_err(|e| ParseDurationError::ParseError(e))?;
        let m: i64 = m.parse().map_err(|e| ParseDurationError::ParseError(e))?;
        Ok(Duration::minutes(h * 60 + m))
    }

}

