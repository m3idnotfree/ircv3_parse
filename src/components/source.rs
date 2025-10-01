use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use crate::{error::SourceError, validators};

#[derive(Clone, Copy, PartialEq, Eq)]
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

impl Display for Source<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.as_str())
    }
}

impl Debug for Source<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct(stringify!(Source))
            .field("name", &self.name)
            .field("user", &self.user)
            .field("host", &self.host)
            .finish()
    }
}
