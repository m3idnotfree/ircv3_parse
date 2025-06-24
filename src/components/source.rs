use std::fmt;

use crate::{error::SourceError, validators};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Source<'a> {
    input: &'a str,
    pub name: &'a str,
    pub user: Option<&'a str>,
    pub host: Option<&'a str>,
}

impl<'a> Source<'a> {
    pub fn parse(input: &'a str) -> Self {
        let mut name = input;
        let mut user = None;
        let mut host = None;

        if let Some((left, h)) = input.rsplit_once('@') {
            host = Some(h);
            name = left;
        }

        if let Some((n, u)) = name.split_once('!') {
            name = n;
            user = Some(u);
        }

        Self {
            input,
            name,
            user,
            host,
        }
    }

    #[inline]
    pub fn as_str(&self) -> &'a str {
        self.input
    }

    pub fn validate(&self) -> Result<(), SourceError> {
        validators::source(self.as_str())
    }
}

impl fmt::Display for Source<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
