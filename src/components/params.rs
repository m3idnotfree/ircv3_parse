use std::{fmt, str::SplitAsciiWhitespace};

use crate::{error::ParamError, validators};

#[derive(Debug, Clone, Copy)]
pub struct Params<'a> {
    input: &'a str,
    pub middles: Middles<'a>,
    pub trailing: Trailing<'a>,
}

impl<'a> Params<'a> {
    #[inline]
    pub fn new(input: &'a str, middles: &'a str, trailing: Option<&'a str>) -> Self {
        Self {
            input,
            middles: Middles(middles),
            trailing: Trailing(trailing),
        }
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        self.input
    }

    #[inline]
    pub fn message(&self) -> String {
        match (self.middles.is_empty(), self.trailing.is_some()) {
            (false, true) => format!(" {} :{}", self.middles, self.trailing),
            (true, false) => String::new(),
            (false, false) => format!(" {}", self.middles),
            (true, true) => format!(" :{}", self.trailing),
        }
    }
}

impl fmt::Display for Params<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.middles.is_empty(), self.trailing.is_some()) {
            (false, true) => write!(f, "{} :{}", self.middles, self.trailing),
            (true, false) => write!(f, ""),
            (false, false) => write!(f, "{}", self.middles),
            (true, true) => write!(f, "{}", self.trailing),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Middles<'a>(&'a str);

impl<'a> Middles<'a> {
    #[inline]
    pub fn as_str(&self) -> &'a str {
        self.0
    }

    #[inline]
    pub fn count(&self) -> usize {
        if self.is_empty() {
            0
        } else {
            self.0.matches(' ').count() + 1
        }
    }

    #[inline]
    pub fn first(&self) -> Option<&'a str> {
        self.iter().next()
    }

    #[inline]
    pub fn second(&self) -> Option<&'a str> {
        self.iter().nth(1)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn to_vec(self) -> Vec<&'a str> {
        self.iter().collect()
    }

    pub fn validate(&self) -> Result<(), ParamError> {
        validators::params(self.as_str())
    }

    #[inline]
    pub fn iter(&self) -> SplitAsciiWhitespace<'a> {
        self.0.split_ascii_whitespace()
    }
}

impl fmt::Display for Middles<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Trailing<'a>(Option<&'a str>);

impl<'a> Trailing<'a> {
    #[inline]
    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }

    #[inline]
    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }

    #[inline]
    pub fn as_str(&self) -> &'a str {
        self.0.unwrap_or("")
    }

    #[inline]
    pub fn raw(&self) -> Option<&'a str> {
        self.0
    }
}

impl<'a> fmt::Display for Trailing<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
