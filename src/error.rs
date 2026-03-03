use crate::compat::{format, Debug, FmtResult, Formatter, String, ToString};

#[derive(Clone, PartialEq, thiserror::Error)]
pub enum IRCError {
    #[error("cannot parse empty message")]
    EmptyInput,
    #[error("{component} must be followed by a space")]
    MissingSpace { component: &'static str },

    #[error(transparent)]
    Command(#[from] CommandError),
    #[error(transparent)]
    Param(#[from] ParamError),
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
            Self::MissingSpace { component } => component,
            Self::Command(cmd) => cmd.code(),
            Self::Param(param) => param.code(),
        }
    }

    pub(crate) fn missing_space(component: &'static str) -> Self {
        Self::MissingSpace { component }
    }

    pub(crate) fn empty_command() -> Self {
        Self::Command(CommandError::Empty)
    }

    pub(crate) fn invalid_first_char_command(c: char) -> Self {
        Self::Command(CommandError::InvalidFirstChar { char: c })
    }

    pub(crate) fn wrong_digit_count_command(actual: usize) -> Self {
        Self::Command(CommandError::WrongDigitCount { actual })
    }

    pub(crate) fn empty_middle_param() -> Self {
        Self::Param(ParamError::EmptyMiddle)
    }
}

#[derive(Clone, PartialEq, thiserror::Error)]
pub enum SerError {
    #[error("command is required")]
    MissingCommand,
    #[error("command already set")]
    DuplicateCommand,

    #[error(transparent)]
    Tag(#[from] TagError),
    #[error(transparent)]
    Source(#[from] SourceError),
    #[error(transparent)]
    Param(#[from] ParamError),
}

impl Debug for SerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "IRC-SERIALIZER[{}]: {}", self.code(), self)
    }
}

impl SerError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::MissingCommand | Self::DuplicateCommand => "COMMAND",
            Self::Tag(tag) => tag.code(),
            Self::Source(src) => src.code(),
            Self::Param(param) => param.code(),
        }
    }

    pub fn is_missing_command(&self) -> bool {
        matches!(self, Self::MissingCommand)
    }

    pub fn is_duplicate_command(&self) -> bool {
        matches!(self, Self::DuplicateCommand)
    }

    pub(crate) fn missing_nick() -> Self {
        Self::Source(SourceError::MissingNick)
    }
}

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum TagError {
    #[error("tags cannot be empty")]
    Empty,

    #[error("tag key cannot be empty")]
    EmptyKey,

    #[error("tag key contains invalid character '{char}' at position {position}")]
    InvalidKeyChar { char: char, position: usize },
    #[error("tag value contains invalid character '{char}' at position {position}")]
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
    #[error("source cannot be empty")]
    Empty,
    #[error("nickname cannot be empty")]
    EmptyNick,
    #[error("username cannot be empty")]
    EmptyUser,
    #[error("nickname must start with a letter, got '{char}'")]
    InvalidNickFirstChar { char: char },
    #[error("nickname contains invalid character '{char}' at position {position} (only letters, digits, and special chars allowed)")]
    InvalidNickChar { char: char, position: usize },

    #[error("username contains invalid character '{char}' at position {position} (control characters not allowed)")]
    InvalidUserChar { char: char, position: usize },

    #[error(transparent)]
    Hostname(#[from] HostnameError),

    #[error("missing source name")]
    MissingNick,
    #[error("source {component} already set")]
    DublicateComponent { component: &'static str },
}

impl SourceError {
    pub fn code(&self) -> &'static str {
        "SOURCE"
    }

    pub(crate) fn duplicate_component(component: &'static str) -> Self {
        Self::DublicateComponent { component }
    }
}

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum CommandError {
    #[error("command cannot be empty")]
    Empty,
    #[error("command must start with a letter or digit, got '{char}'")]
    InvalidFirstChar { char: char },
    #[error("command must be all letters or 3-digit number, got '{input}' at position {position}")]
    InvalidCommand { input: String, position: usize },
    #[error("numeric command must be exactly 3 digits, got {actual} digits")]
    WrongDigitCount { actual: usize },
}

impl CommandError {
    pub fn code(&self) -> &'static str {
        "CMD"
    }
}

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum ParamError {
    #[error("parameter middle cannot be empty")]
    EmptyMiddle,
    #[error("parameter middle contains invalid character '{char}' at position {position}")]
    InvalidMiddleChar { char: char, position: usize },
    #[error("parameter contains forbidden control character (colon, space, CR, LF, NUL)")]
    ContainsControlChar,
}

impl ParamError {
    pub fn code(&self) -> &'static str {
        "PARAM"
    }
}

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum HostnameError {
    #[error("hostname cannot be empty")]
    Empty,
    #[error("hostname label cannot be empty")]
    EmptyLabel,

    #[error("hostname label must start with a letter or digit, got '{char}'")]
    InvalidFirstChar { char: char },
    #[error("hostname label cannot end with hyphen, got '{char}'")]
    InvalidLastChar { char: char },
    #[error("hostname label contains invalid character '{char}'")]
    InvalidChar { char: char },

    #[error("hostname label exceeds maximum length (max {max} characters, got {actual})")]
    LabelTooLong { max: usize, actual: usize },
    #[error("hostname exceeds maximum depth (max {max} labels, got {actual})")]
    TooManyLabels { max: usize, actual: usize },
    #[error("hostname exceeds maximum total length (max {max} characters, got {actual})")]
    TooLong { max: usize, actual: usize },
}

impl HostnameError {
    pub fn code(&self) -> &'static str {
        "RFC952"
    }
}

#[derive(Clone, PartialEq, thiserror::Error)]
pub enum DeError {
    #[error("command mismatch: expected `{expected}`, got `{actual}`")]
    CommandMismatch { expected: String, actual: String },

    #[error("component not found: {component}")]
    ComponentNotFound { component: &'static str },

    #[error("not found `{component}` value: `{key}`{}", .context.as_ref().map(|context| format!(" ({context})")).unwrap_or_default())]
    NotFound {
        component: &'static str,
        key: String,
        context: Option<String>,
    },

    #[error("failed to parse IRC message: {0}")]
    ParseError(#[from] IRCError),
}

impl Debug for DeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "IRC-DESERIALIZER[{}]: {}", self.code(), self)
    }
}

impl DeError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::CommandMismatch { .. } => "COMMAND_MISMATCH",
            Self::ComponentNotFound { .. } => "COMPONENT_NOT_FOUND",
            Self::NotFound { .. } => "NOT_FOUND",
            Self::ParseError(e) => e.code(),
        }
    }

    pub fn is_parse_error(&self) -> bool {
        matches!(self, DeError::ParseError(..))
    }

    pub fn is_tags_component_not_found(&self) -> bool {
        matches!(self, DeError::ComponentNotFound { component: "tags" })
    }

    pub fn is_source_component_not_found(&self) -> bool {
        matches!(
            self,
            DeError::ComponentNotFound {
                component: "source"
            }
        )
    }

    pub fn is_param_component_not_found(&self) -> bool {
        matches!(self, DeError::ComponentNotFound { component: "param" })
    }

    pub fn is_trailing_component_not_found(&self) -> bool {
        matches!(
            self,
            DeError::ComponentNotFound {
                component: "trailing"
            }
        )
    }

    pub fn is_not_found_tag(&self) -> bool {
        matches!(
            self,
            DeError::NotFound {
                component: "tag",
                ..
            }
        )
    }

    pub fn is_not_found_source(&self) -> bool {
        matches!(
            self,
            DeError::NotFound {
                component: "source",
                ..
            }
        )
    }

    pub fn is_not_found_param(&self) -> bool {
        matches!(
            self,
            DeError::NotFound {
                component: "param",
                ..
            }
        )
    }

    pub fn is_not_found_command(&self) -> bool {
        matches!(
            self,
            DeError::NotFound {
                component: "command",
                ..
            }
        )
    }

    pub fn is_not_found_trailing(&self) -> bool {
        matches!(
            self,
            DeError::NotFound {
                component: "trailing",
                ..
            }
        )
    }

    pub fn is_command_mismatch(&self) -> bool {
        matches!(self, DeError::CommandMismatch { .. })
    }

    pub fn command_mismatch(expected: impl Into<String>, actual: impl Into<String>) -> Self {
        Self::CommandMismatch {
            expected: expected.into(),
            actual: actual.into(),
        }
    }

    pub fn tags_component_not_found() -> Self {
        Self::ComponentNotFound { component: "tags" }
    }

    pub fn source_component_not_found() -> Self {
        Self::ComponentNotFound {
            component: "source",
        }
    }

    pub fn param_component_not_found() -> Self {
        Self::ComponentNotFound { component: "param" }
    }

    pub fn not_found(component: &'static str, key: impl Into<String>) -> Self {
        Self::NotFound {
            component,
            key: key.into(),
            context: None,
        }
    }

    pub fn not_found_with_context(
        component: &'static str,
        key: impl Into<String>,
        context: impl Into<String>,
    ) -> Self {
        Self::NotFound {
            component,
            key: key.into(),
            context: Some(context.into()),
        }
    }

    pub fn not_found_tag(key: impl Into<String>) -> Self {
        Self::not_found("tag", key)
    }

    pub fn not_found_source(component: &'static str) -> Self {
        Self::not_found("source", component)
    }

    pub fn not_found_param(index: usize) -> Self {
        Self::not_found("param", index.to_string())
    }

    pub fn not_found_trailing() -> Self {
        Self::ComponentNotFound {
            component: "trailing",
        }
    }

    pub fn not_found_variant(
        component: &'static str,
        actual: impl Into<String>,
        expected: impl Into<String>,
    ) -> Self {
        Self::not_found_with_context(
            component,
            actual,
            format!("expected one of: {}", expected.into()),
        )
    }
}
