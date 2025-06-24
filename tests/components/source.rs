use std::fmt;

use proptest::prelude::{prop_compose, prop_oneof, Strategy};

use super::host::rfc1123_strategy;

#[derive(Debug, Clone, PartialEq)]
pub enum SourceType {
    ServerName(String),
    Full(String, String, String),
    NickUser(String, String),
    NickHost(String, String),
    Nick(String),
}

impl fmt::Display for SourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ServerName(server) => write!(f, "{}", server),
            Self::Full(nick, user, host) => write!(f, "{}!{}@{}", nick, user, host),
            Self::NickUser(nick, user) => write!(f, "{}!{}", nick, user),
            Self::NickHost(nick, host) => write!(f, "{}@{}", nick, host),
            Self::Nick(nick) => write!(f, "{}", nick),
        }
    }
}

impl SourceType {
    pub fn expected_name(&self) -> &str {
        match self {
            Self::ServerName(server) => server,
            Self::Full(nick, _, _) => nick,
            Self::NickUser(nick, _) => nick,
            Self::NickHost(nick, _) => nick,
            Self::Nick(nick) => nick,
        }
    }

    pub fn expected_user(&self) -> Option<&str> {
        match self {
            Self::ServerName(_) => None,
            Self::Full(_, user, _) => Some(user),
            Self::NickUser(_, user) => Some(user),
            Self::NickHost(_, _) => None,
            Self::Nick(_) => None,
        }
    }

    pub fn expected_host(&self) -> Option<&str> {
        match self {
            Self::ServerName(_) => None,
            Self::Full(_, _, host) => Some(host),
            Self::NickUser(_, _) => None,
            Self::NickHost(_, host) => Some(host),
            Self::Nick(_) => None,
        }
    }
}

prop_compose! {
    pub fn source_strategy()(
        source in prop_oneof![
            rfc1123_strategy().prop_map(SourceType::ServerName),

            (nickname_strategy(), username_with_at_strategy(), rfc1123_strategy())
                .prop_map(|(n, u, h)|SourceType::Full(n, u, h)),

            (nickname_strategy(), username_without_at_strategy())
                .prop_map(|(n, u)| SourceType::NickUser(n, u)),

            (nickname_strategy(), rfc1123_strategy())
                .prop_map(|(n, h)| SourceType::NickHost(n, h)) ,

            nickname_strategy().prop_map(SourceType::Nick),
        ]) -> SourceType { source }
}

prop_compose! {
    pub fn nickname_strategy()(
        nickname in "[a-zA-Z][a-zA-Z0-9\\[\\]`\\^{}\\\\]*",
    ) -> String { nickname }
}

prop_compose! {
    pub fn username_with_at_strategy()(
        username in "[^ \r\n\0]+",
    ) -> String { username }
}

prop_compose! {
    pub fn username_without_at_strategy()(
        username in "[^ \r\n\0@]+",
    ) -> String { username }
}
