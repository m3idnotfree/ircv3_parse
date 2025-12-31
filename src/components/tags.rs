use crate::compat::{Display, FmtResult, Formatter, Map, Split, String, Vec};

use crate::{error::TagError, unescape, validators, EQ, SEMICOLON};

type TagPair<'a> = (&'a str, TagValue<'a>);

/// Represents the value of an IRCv3 message tag.
///
/// IRCv3 tags can have three states:
/// - **Flag**: Tag exists without a value (e.g., `subscriber`)
/// - **Empty**: Tag exists with an empty value (e.g., `color=`)
/// - **Value**: Tag has an actual value (e.g., `user-id=123`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TagValue<'a> {
    Flag,
    Empty,
    Value(&'a str),
}

impl<'a> TagValue<'a> {
    /// Returns the tag value as a string slice.
    ///
    /// For [`TagValue::Flag`] and [`TagValue::Empty`], returns an empty string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ircv3_parse::components::TagValue;
    /// assert_eq!(TagValue::Value("123").as_str(), "123");
    /// assert_eq!(TagValue::Flag.as_str(), "");
    /// assert_eq!(TagValue::Empty.as_str(), "");
    /// ```
    pub fn as_str(&self) -> &'a str {
        match self {
            Self::Flag | Self::Empty => "",
            TagValue::Value(value) => value,
        }
    }

    pub fn is_flag(&self) -> bool {
        matches!(self, TagValue::Flag)
    }

    /// Returns `true` if this tag has an explicitly empty value.
    pub fn is_empty(&self) -> bool {
        matches!(self, TagValue::Empty)
    }

    pub fn has_value(&self) -> bool {
        matches!(self, TagValue::Value(_))
    }
}

impl<'a> Display for TagValue<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Value(value) => f.write_str(value),
            _ => f.write_str(""),
        }
    }
}

/// IRCv3 message tags component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tags<'a>(&'a str);

impl<'a> Tags<'a> {
    #[inline]
    pub fn new(val: &'a str) -> Self {
        Self(val)
    }

    /// Returns the raw tags string.
    #[inline]
    pub fn as_str(&self) -> &'a str {
        self.0
    }

    #[inline]
    pub fn count(&self) -> usize {
        self.0.matches(SEMICOLON as char).count() + 1
    }

    #[inline]
    pub fn to_vec(self) -> Vec<TagPair<'a>> {
        self.into_iter().collect()
    }

    /// Checks if a tag exists as a flag (without a value).
    #[inline]
    pub fn get_flag(&self, key: &str) -> bool {
        self.split().any(|tag| tag == key)
    }

    /// Gets the value of a tag by key.
    #[inline]
    pub fn get(&self, key: &str) -> Option<TagValue<'a>> {
        self.split().find_map(|tag| self._get(tag, key))
    }

    #[inline]
    fn _get(&self, tag: &'a str, key: &str) -> Option<TagValue<'a>> {
        if let Some(after_key) = tag.strip_prefix(key) {
            match after_key.chars().next() {
                Some('=') => {
                    let value = &after_key[1..];
                    if value.is_empty() {
                        Some(TagValue::Empty)
                    } else {
                        Some(TagValue::Value(value))
                    }
                }
                None => Some(TagValue::Flag),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Gets a client-only tag (prefixed with `+`).
    #[inline]
    pub fn get_prefix(&self, key: &str) -> Option<TagValue<'a>> {
        self.split().find_map(|tag| {
            if let Some(after_key) = tag.strip_prefix('+') {
                self._get(after_key, key)
            } else {
                None
            }
        })
    }

    /// Splits tags by semicolon separator.
    #[inline]
    pub fn split(&self) -> Split<'a, char> {
        self.0.split(SEMICOLON as char)
    }

    /// Gets a tag value with escape sequences converted to actual characters.
    #[inline]
    pub fn get_unescaped(&self, key: &str) -> Option<String> {
        self.get(key).map(|value| unescape(value.as_str()))
    }

    /// Returns an iterator over tag key-value pairs.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = TagPair<'a>> {
        self.split().map(|tag| {
            if let Some((key, value)) = tag.split_once(EQ as char) {
                if value.is_empty() {
                    (key, TagValue::Empty)
                } else {
                    (key, TagValue::Value(value))
                }
            } else {
                (tag, TagValue::Flag)
            }
        })
    }

    #[inline]
    pub fn contains(&self, key: &str) -> bool {
        self.iter().any(|(k, _)| k == key)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn validate(&self) -> Result<(), TagError> {
        validators::tags(self.as_str())
    }
}

impl Display for Tags<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for Tags<'_> {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl<'a> IntoIterator for Tags<'a> {
    type Item = TagPair<'a>;
    type IntoIter = Map<Split<'a, char>, fn(&'a str) -> TagPair<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.split(SEMICOLON as char).map(|tag| {
            if let Some((key, value)) = tag.split_once(EQ as char) {
                if value.is_empty() {
                    (key, TagValue::Empty)
                } else {
                    (key, TagValue::Value(value))
                }
            } else {
                (tag, TagValue::Flag)
            }
        })
    }
}

impl<'a> IntoIterator for &Tags<'a> {
    type Item = TagPair<'a>;
    type IntoIter = Map<Split<'a, char>, fn(&'a str) -> TagPair<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.split(SEMICOLON as char).map(|tag| {
            if let Some((key, value)) = tag.split_once(EQ as char) {
                if value.is_empty() {
                    (key, TagValue::Empty)
                } else {
                    (key, TagValue::Value(value))
                }
            } else {
                (tag, TagValue::Flag)
            }
        })
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for TagValue<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            TagValue::Flag => serializer.serialize_none(),
            TagValue::Empty => serializer.serialize_str(""),
            TagValue::Value(s) => serializer.serialize_str(s),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Tags<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;

        let mut map = serializer.serialize_map(Some(self.count()))?;
        for (key, value) in self.iter() {
            map.serialize_entry(key, &value)?;
        }
        map.end()
    }
}
