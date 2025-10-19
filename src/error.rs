use crate::compat::{Debug, FmtResult, Formatter, String};

#[derive(Clone, PartialEq, thiserror::Error)]
pub enum IRCError {
    #[error("Cannot parse empty message")]
    EmptyInput,
    #[error("{component} must be followed by a space")]
    MissingSpace { component: &'static str },

    #[error(transparent)]
    Tag(#[from] TagError),
    #[error(transparent)]
    Source(#[from] SourceError),
    #[error(transparent)]
    Command(#[from] CommandError),
    #[error(transparent)]
    Param(#[from] ParamError),

    #[error(transparent)]
    Hostname(#[from] HostnameError),
}

impl Debug for IRCError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "IRC-PARSER[{}]: {}", self.code(), self)
    }
}

impl IRCError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::EmptyInput => "SCAN",
            Self::Command(cmd) => cmd.code(),
            Self::Tag(tag) => tag.code(),
            Self::Source(src) => src.code(),
            Self::Param(param) => param.code(),
            Self::Hostname(host) => host.code(),
            Self::MissingSpace { component } => component,
        }
    }
}

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum TagError {
    #[error("Tags cannot be empty")]
    Empty,

    #[error("Tag key cannot be empty")]
    EmptyKey,

    #[error("Tag key contains invalid character '{char}' at position {position}")]
    InvalidKeyChar { char: char, position: usize },
    #[error("Tag value contains invalid character '{char}' at position {position}")]
    InvalidValueChar { char: char, position: usize },

    #[error(transparent)]
    Hostname(#[from] HostnameError),
}

impl TagError {
    pub fn code(&self) -> &'static str {
        "TAG"
    }
}

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum SourceError {
    #[error("Source cannot be empty")]
    Empty,
    #[error("Nickname cannot be empty")]
    EmptyNick,
    #[error("Nickname must start with letter, got '{char}'")]
    InvalidNickFirstChar { char: char },
    #[error("Nickname contains invalid character '{char}' at position {position} (only letters, digits, and special chars allowed)")]
    InvalidNickChar { char: char, position: usize },

    #[error("Username contains invalid character '{char}' at position {position} (control characters not allowed)")]
    InvalidUserChar { char: char, position: usize },

    #[error(transparent)]
    Hostname(#[from] HostnameError),
}

impl SourceError {
    pub fn code(&self) -> &'static str {
        "SOURCE"
    }
}

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum CommandError {
    #[error("Command cannot be empty")]
    Empty,
    #[error("Command must start with letter or digit, got '{char}'")]
    InvalidFirstChar { char: char },
    #[error("Command must be letters or 3-digit number, got '{input}' at position {position}")]
    InvalidCommand { input: String, position: usize },
    #[error("Numeric command must be exactly 3 digits, got {actual} digits")]
    WrongDigitCount { actual: usize },
}
impl CommandError {
    pub fn code(&self) -> &'static str {
        "CMD"
    }
}

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum ParamError {
    #[error("Parameter middle cannot be empty")]
    EmptyMiddle,
    #[error("Parameter middle contains invalid character '{char}' at position {position}")]
    InvalidMiddleChar { char: char, position: usize },
    #[error("Parameter contains forbidden control character (colon, space, CR, LF, NUL)")]
    ContainsControlChar,
}
impl ParamError {
    pub fn code(&self) -> &'static str {
        "PARAM"
    }
}

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum HostnameError {
    #[error("Hostname cannot be empty")]
    Empty,
    #[error("Hostname label cannot be empty")]
    EmptyLabel,

    #[error("Hostname label must start with letter or digit, got '{char}'")]
    InvalidFirstChar { char: char },
    #[error("Hostname label cannot end with hypen, got '{char}'")]
    InvalidLastChar { char: char },
    #[error("Hostname label contains invalid character '{char}'")]
    InvalidChar { char: char },

    #[error("Hostname label exceeds maximum length (max {max} characters, got {actual})")]
    LabelTooLong { max: usize, actual: usize },
    #[error("Hostname exceeds maximum depth (max {max} labels, got {actual})")]
    TooManyLabels { max: usize, actual: usize },
    #[error("Hostname exceeds maximum total length (max {max} characters, got {actual})")]
    TooLong { max: usize, actual: usize },
}

impl HostnameError {
    pub fn code(&self) -> &'static str {
        "RFC952"
    }
}
