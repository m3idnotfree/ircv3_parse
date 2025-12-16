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

#[derive(Clone, PartialEq, thiserror::Error)]
pub enum ExtractError {
    #[error("Invalid command: expected {expected}, got {actual}")]
    InvalidCommand { expected: String, actual: String },
    #[error("Missing required component: {component}")]
    MissingComponent { component: &'static str },
    #[error("Missing required field: {field}")]
    MissingField { field: &'static str },
    #[error("Invalid field value for '{field}': {reason}")]
    InvalidValue { field: &'static str, reason: String },

    #[error("Failed to parse IRC message: {0}")]
    ParseError(#[from] IRCError),
}

impl Debug for ExtractError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "EXTRACT[{}]: {}", self.code(), self)
    }
}

impl ExtractError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidCommand { .. } => "INVALID_COMMAND",
            Self::MissingComponent { .. } => "MISSING_COMPONENT",
            Self::MissingField { .. } => "MISSING_FIELD",
            Self::InvalidValue { .. } => "INVALID_VALUE",
            Self::ParseError(e) => e.code(),
        }
    }

    pub fn is_parse_error(&self) -> bool {
        matches!(self, ExtractError::ParseError(..))
    }

    pub fn is_missing_tags(&self) -> bool {
        matches!(self, ExtractError::MissingComponent { component: "tags" })
    }

    pub fn is_missing_source(&self) -> bool {
        matches!(
            self,
            ExtractError::MissingComponent {
                component: "source"
            }
        )
    }

    pub fn is_missing_param(&self) -> bool {
        matches!(self, ExtractError::MissingComponent { component: "param" })
    }

    pub fn is_invalid_command(&self) -> bool {
        matches!(self, ExtractError::InvalidCommand { .. })
    }

    pub fn invalid_command(expected: impl Into<String>, actual: impl Into<String>) -> Self {
        Self::InvalidCommand {
            expected: expected.into(),
            actual: actual.into(),
        }
    }

    pub fn missing_tags() -> Self {
        Self::MissingComponent { component: "tags" }
    }

    pub fn missing_source() -> Self {
        Self::MissingComponent {
            component: "source",
        }
    }

    pub fn missing_command() -> Self {
        Self::MissingComponent {
            component: "command",
        }
    }

    pub fn missing_param() -> Self {
        Self::MissingComponent { component: "param" }
    }

    pub fn missing_field(field: &'static str) -> Self {
        Self::MissingField { field }
    }

    pub fn invalid_value(field: &'static str, reason: impl Into<String>) -> Self {
        Self::InvalidValue {
            field,
            reason: reason.into(),
        }
    }
}
