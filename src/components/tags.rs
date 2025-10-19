use crate::compat::{Display, FmtResult, Formatter, Map, Split, String, Vec};

use crate::{error::TagError, unescaped_to_escaped, validators};

const SEMICOLON_CHAR: char = ';';
const EQUAL_CHAR: char = '=';

type TagPair<'a> = (&'a str, TagValue<'a>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TagValue<'a> {
    Flag,
    Empty,
    Value(&'a str),
}

impl<'a> TagValue<'a> {
    pub fn as_str(&self) -> &'a str {
        match self {
            Self::Flag | Self::Empty => "",
            TagValue::Value(value) => value,
        }
    }

    pub fn is_flag(&self) -> bool {
        matches!(self, TagValue::Flag)
    }

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tags<'a>(&'a str);

impl<'a> Tags<'a> {
    #[inline]
    pub fn new(val: &'a str) -> Self {
        Self(val)
    }

    #[inline]
    pub fn as_str(&self) -> &'a str {
        self.0
    }

    #[inline]
    pub fn count(&self) -> usize {
        self.0.matches(SEMICOLON_CHAR).count() + 1
    }

    #[inline]
    pub fn to_vec(self) -> Vec<TagPair<'a>> {
        self.into_iter().collect()
    }

    #[inline]
    pub fn get_flag(&self, key: &str) -> bool {
        self.split().any(|tag| tag == key)
    }

    // Gets the raw value for a key in the tag list without unescaping.
    //
    // Returns:
    // * `None` if the key doesn't exist
    // * `Some(TagValue::Empty)` if the key exists with an empty value
    // * `Some(TagValue::Value(value))` if the key exists with a value
    // * `Some(TagValue::Flag)` if the key exists but flag
    #[inline]
    pub fn get(&'a self, key: &str) -> Option<TagValue<'a>> {
        self.split().find_map(|tag| self._get(tag, key))
    }

    #[inline]
    fn _get(&'a self, tag: &'a str, key: &str) -> Option<TagValue<'a>> {
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

    #[inline]
    pub fn get_prefix(&'a self, key: &str) -> Option<TagValue<'a>> {
        self.split().find_map(|tag| {
            if let Some(after_key) = tag.strip_prefix('+') {
                self._get(after_key, key)
            } else {
                None
            }
        })
    }

    #[inline]
    pub fn split(&'a self) -> Split<'a, char> {
        self.0.split(SEMICOLON_CHAR)
    }

    #[inline]
    pub fn get_escaped(&self, key: &str) -> Option<String> {
        self.get(key)
            .map(|value| unescaped_to_escaped(value.as_str()))
    }

    #[inline]
    pub fn iter(&'a self) -> impl Iterator<Item = TagPair<'a>> {
        self.split().map(|tag| {
            if let Some((key, value)) = tag.split_once(EQUAL_CHAR) {
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
        self.0.split(SEMICOLON_CHAR).map(|tag| {
            if let Some((key, value)) = tag.split_once(EQUAL_CHAR) {
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
        self.0.split(SEMICOLON_CHAR).map(|tag| {
            if let Some((key, value)) = tag.split_once(EQUAL_CHAR) {
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
