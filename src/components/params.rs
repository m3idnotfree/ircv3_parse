use crate::compat::{
    format, Debug, Display, FmtResult, Formatter, SplitAsciiWhitespace, String, Vec,
};

use crate::{error::ParamError, validators};

/// IRC message parameters.
#[derive(Clone, Copy)]
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

    /// Returns the parameters formatted as they would appear in a message.
    ///
    /// Includes the leading space and `:` prefix for trailing parameter.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let msg = ircv3_parse::parse("PRIVMSG #channel :Hello")?;
    /// let params = msg.params();
    ///
    /// assert_eq!(params.message(), " #channel :Hello");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
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

impl Display for Params<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match (self.middles.is_empty(), self.trailing.is_some()) {
            (false, true) => write!(f, "{} :{}", self.middles, self.trailing),
            (true, false) => f.write_str(""),
            (false, false) => f.write_str(self.middles.as_str()),
            (true, true) => f.write_str(self.trailing.as_str()),
        }
    }
}

impl Debug for Params<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct(stringify!(Params))
            .field("middles", &self.middles.as_str())
            .field("trailing", &self.trailing.raw())
            .finish()
    }
}

/// Middle parameters
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    pub fn to_vec(&self) -> Vec<&'a str> {
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

impl Display for Middles<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for Middles<'_> {
    fn as_ref(&self) -> &str {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    /// Returns the trailing parameter as a string.
    ///
    /// Returns an empty string if there's no trailing parameter.
    #[inline]
    pub fn as_str(&self) -> &'a str {
        self.0.unwrap_or("")
    }

    /// Returns the raw trailing parameter as an `Option`.
    ///
    /// Use this when you need to distinguish between an empty trailing
    /// parameter and no trailing parameter at all.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let msg1 = ircv3_parse::parse("PRIVMSG #channel :Hello")?;
    /// assert_eq!(msg1.params().trailing.raw(), Some("Hello"));
    ///
    /// let msg2 = ircv3_parse::parse("PRIVMSG #channel :")?;  // Empty trailing
    /// assert_eq!(msg2.params().trailing.raw(), Some(""));
    ///
    /// let msg3 = ircv3_parse::parse("JOIN #channel")?;  // No trailing
    /// assert_eq!(msg3.params().trailing.raw(), None);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn raw(&self) -> Option<&'a str> {
        self.0
    }
}

impl<'a> Display for Trailing<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Params<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let field_count = (!self.middles.is_empty()) as usize + self.trailing.is_some() as usize;

        let mut state = serializer.serialize_struct("Params", field_count)?;

        if !self.middles.is_empty() {
            state.serialize_field("middles", &self.middles)?;
        }

        if self.trailing.is_some() {
            state.serialize_field("trailing", &self.trailing)?;
        }

        state.end()
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Middles<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(Some(self.count()))?;
        for param in self.iter() {
            seq.serialize_element(param)?;
        }
        seq.end()
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Trailing<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.0 {
            Some(s) => serializer.serialize_str(s),
            None => serializer.serialize_none(),
        }
    }
}
