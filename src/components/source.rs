use crate::compat::{Debug, Display, FmtResult, Formatter};

use crate::{error::SourceError, validators};

/// IRC message source component.
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

#[cfg(feature = "serde")]
impl serde::Serialize for Source<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let field_count = 1 + self.user.is_some() as usize + self.host.is_some() as usize;

        let mut state = serializer.serialize_struct("Source", field_count)?;
        state.serialize_field("name", &self.name)?;

        if let Some(user) = self.user {
            state.serialize_field("user", user)?;
        }

        if let Some(host) = self.host {
            state.serialize_field("host", host)?;
        }

        state.end()
    }
}
