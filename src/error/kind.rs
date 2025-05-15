use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorKind {
    Protocol(ProtocolErrorKind),
    Resource(ResourceErrorKind),
    Security(SecurityErrorKind),
    Parser(ParserErrorKind),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolErrorKind {
    Host,
    Tag,
    Command,
    Nick,
    Middle,
    Trailing,
    Crlf,
    Empty,
    Space,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceErrorKind {
    TooManyTags,
    MessageTooLong,
    TooManyParams,
    StackOverflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityErrorKind {
    InvalidChar,
    PotentialInjection,
    RateLimited,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParserErrorKind {
    OneOfSpecial,
    AlphaNumeric,
    Alpha,
    General,
}

impl ErrorKind {
    pub fn description(&self) -> &'static str {
        match self {
            Self::Protocol(kind) => match kind {
                ProtocolErrorKind::Host => "invalid host format",
                ProtocolErrorKind::Tag => "invalid tag format",
                ProtocolErrorKind::Command => "invalid command format",
                ProtocolErrorKind::Nick => "invalid nickname format",
                ProtocolErrorKind::Middle => "invalid middle parameter format",
                ProtocolErrorKind::Trailing => "invalid trailing parameter format",
                ProtocolErrorKind::Crlf => "expected CRLF line ending",
                ProtocolErrorKind::Empty => "unexpected empty value",
                ProtocolErrorKind::Space => "expected space character",
            },
            Self::Resource(kind) => match kind {
                ResourceErrorKind::TooManyTags => "too many message tags",
                ResourceErrorKind::MessageTooLong => "message exceeds length limit",
                ResourceErrorKind::TooManyParams => "too many message parameters",
                ResourceErrorKind::StackOverflow => "parser recursion limit exceeded",
            },
            Self::Security(kind) => match kind {
                SecurityErrorKind::InvalidChar => "invalid or dangerous character",
                SecurityErrorKind::PotentialInjection => "potential command injection",
                SecurityErrorKind::RateLimited => "message rate limit exceeded",
            },
            Self::Parser(kind) => match kind {
                ParserErrorKind::OneOfSpecial => "expected one of special characters",
                ParserErrorKind::AlphaNumeric => "expected alphanumeric character",
                ParserErrorKind::Alpha => "expected alphabetic character",
                ParserErrorKind::General => "parser error",
            },
        }
    }

    pub fn error_code(&self) -> &'static str {
        match self {
            Self::Protocol(kind) => match kind {
                ProtocolErrorKind::Host => "E0101_INVALID_HOST",
                ProtocolErrorKind::Tag => "E0102_INVALID_TAG",
                ProtocolErrorKind::Command => "E0103_INVALID_COMMAND",
                ProtocolErrorKind::Nick => "E0104_INVALID_NICK",
                ProtocolErrorKind::Middle => "E0105_INVALID_MIDDLE",
                ProtocolErrorKind::Trailing => "E0106_INVALID_Trailing",
                ProtocolErrorKind::Crlf => "E0107_INVALID_CRLF",
                ProtocolErrorKind::Empty => "E0108_UNEXPECTED_EMPTY",
                ProtocolErrorKind::Space => "E0109_EXPECTED_SPACE",
            },
            Self::Resource(kind) => match kind {
                ResourceErrorKind::TooManyTags => "E0201_TOO_MANY_TAGS",
                ResourceErrorKind::MessageTooLong => "E0202_MESSAGE_TOO_LONG",
                ResourceErrorKind::TooManyParams => "E0203_TOO_MANY_PARAMS",
                ResourceErrorKind::StackOverflow => "E0204_STACK_OVERFLOW",
            },
            Self::Security(kind) => match kind {
                SecurityErrorKind::InvalidChar => "E0301_INVALID_CHAR",
                SecurityErrorKind::PotentialInjection => "E0302_POTENTIAL_INJECTION",
                SecurityErrorKind::RateLimited => "E0303_RATE_LIMITED",
            },
            Self::Parser(kind) => match kind {
                ParserErrorKind::OneOfSpecial => "E0401_EXPECTED_SPECIAL",
                ParserErrorKind::AlphaNumeric => "E0402_EXPECTED_ALNUM",
                ParserErrorKind::Alpha => "E0403_EXPECTED_ALPHA",
                ParserErrorKind::General => "E0404_PARSER_ERROR",
            },
        }
    }

    pub fn is_alert_worthy(&self) -> bool {
        matches!(
            self,
            Self::Security(_) | Self::Resource(ResourceErrorKind::StackOverflow)
        )
    }

    pub fn suggested_log_level(&self) -> &'static str {
        match self {
            Self::Protocol(_) => "WARN",
            Self::Resource(ResourceErrorKind::StackOverflow) => "ERROR",
            Self::Resource(_) => "WARN",
            Self::Security(_) => "ERROR",
            Self::Parser(_) => "DEBUG",
        }
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.description())
    }
}
