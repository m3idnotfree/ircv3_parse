use std::{collections::HashSet, fmt};

use proptest::{
    collection, option,
    prelude::{Just, Strategy},
    prop_compose, prop_oneof,
};

use super::host::rfc1123_strategy;

#[derive(Debug, Clone, PartialEq)]
pub struct TagsType(Vec<TagType>);

impl TagsType {
    pub fn count(&self) -> usize {
        self.0.len()
    }
    pub fn iter(&self) -> std::slice::Iter<'_, TagType> {
        self.0.iter()
    }
}
impl fmt::Display for TagsType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|tag| tag.to_string())
                .collect::<Vec<_>>()
                .join(";")
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TagType {
    Flag(String),
    KeyValue(String, String),
    KeyEmptyValue(String, String),
}
impl fmt::Display for TagType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Flag(key) => write!(f, "{}", key),
            Self::KeyValue(k, v) => write!(f, "{}={}", k, v),
            Self::KeyEmptyValue(k, _) => write!(f, "{}=", k),
        }
    }
}
impl TagType {
    pub fn expected_key(&self) -> String {
        match self {
            Self::Flag(k) => k.to_string(),
            Self::KeyValue(k, _) => k.to_string(),
            Self::KeyEmptyValue(k, _) => k.to_string(),
        }
    }

    pub fn expected_value(&self) -> Option<String> {
        match self {
            Self::Flag(_) => Some("".to_string()),
            Self::KeyValue(_, v) => Some(v.to_string()),
            Self::KeyEmptyValue(_, _) => Some("".to_string()),
        }
    }
}

prop_compose! {
    pub fn tags_strategy()(
        tags in collection::vec(tag_strategy(), 1..=64)
    ) -> TagsType {
        let mut keys = HashSet::new();
        let mut deduped = Vec::new();

        for tag in tags.into_iter() {
            let key = tag.expected_key();
            if keys.insert(key) {
                deduped.push(tag);
            }
        }

        TagsType(deduped)
    }
}

prop_compose! {
    fn tag_strategy()(
        tags in prop_oneof![
            5 => (tag_key_strategy(), tag_value_strategy()).prop_map(|(k, v)|TagType::KeyValue(k, v)),
            3 => tag_key_strategy().prop_map(TagType::Flag),
            2 => tag_key_strategy().prop_map(|k|TagType::KeyEmptyValue(k, "".to_string())),
    ]) -> TagType { tags }
}

prop_compose! {
    fn tag_key_strategy()(
        client_prefix in option::of(Just("+")),
        vendor in option::of(rfc1123_strategy()),
        key_name in "[a-zA-Z0-9-]+"
    ) -> String {
        let mut result = String::new();
        if client_prefix.is_some(){
            result.push('+');
        }

        if let Some(vendor) = vendor {
            result.push_str(&vendor);
            result.push('/');
        }

        result.push_str(&key_name);
        result
    }
}

prop_compose! {
    fn tag_value_strategy()(
        value in "[^ \0\r\n;]*"
    ) -> String { value }
}
