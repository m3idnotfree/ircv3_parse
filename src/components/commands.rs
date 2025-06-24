use std::fmt;

use crate::{error::CommandError, validators};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Commands<'a> {
    NUMERIC(&'a str),
    // Connection Messages
    CAP,
    AUTHENTICATE,
    PASS,
    NICK,
    USER,
    PING,
    PONG,
    OPER,
    QUIT,
    ERROR,
    // Channel Operations
    JOIN,
    PART,
    TOPIC,
    NAMES,
    LIST,
    INVITE,
    KICK,
    // Server Queries and Commands
    MOTD,
    VERSION,
    ADMIN,
    CONNECT,
    LUSERS,
    TIME,
    STATS,
    HELP,
    INFO,
    MODE,
    // Sending Messages
    PRIVMSG,
    NOTICE,
    // User-Based Queries
    WHO,
    WHOIS,
    WHOWAS,
    // Operator Messages
    KILL,
    REHASH,
    RESTART,
    SQUIT,
    // Optional Messages
    AWAY,
    LINKS,
    USERHOST,
    WALLOPS,
    CUSTOM(&'a str),
}

impl<'a> Commands<'a> {
    #[inline]
    pub fn as_str(&self) -> &str {
        match self {
            Self::NUMERIC(num) => num,
            Self::CAP => "CAP",
            Self::AUTHENTICATE => "AUTHENTICATE",
            Self::PASS => "PASS",
            Self::NICK => "NICK",
            Self::USER => "USER",
            Self::PING => "PING",
            Self::PONG => "PONG",
            Self::OPER => "OPER",
            Self::QUIT => "QUIT",
            Self::ERROR => "ERROR",
            Self::JOIN => "JOIN",
            Self::PART => "PART",
            Self::TOPIC => "TOPIC",
            Self::NAMES => "NAMES",
            Self::LIST => "LIST",
            Self::INVITE => "INVITE",
            Self::KICK => "KICK",
            Self::MOTD => "MOTD",
            Self::VERSION => "VERSION",
            Self::ADMIN => "ADMIN",
            Self::CONNECT => "CONNECT",
            Self::LUSERS => "LUSERS",
            Self::TIME => "TIME",
            Self::STATS => "STATS",
            Self::HELP => "HELP",
            Self::INFO => "INFO",
            Self::MODE => "MODE",
            Self::PRIVMSG => "PRIVMSG",
            Self::NOTICE => "NOTICE",
            Self::WHO => "WHO",
            Self::WHOIS => "WHOIS",
            Self::WHOWAS => "WHOWAS",
            Self::KILL => "KILL",
            Self::REHASH => "REHASH",
            Self::RESTART => "RESTART",
            Self::SQUIT => "SQUIT",
            Self::AWAY => "AWAY",
            Self::LINKS => "LINKS",
            Self::USERHOST => "USERHOST",
            Self::WALLOPS => "WALLOPS",
            Self::CUSTOM(custom) => custom,
        }
    }

    #[inline]
    pub fn is_ping(&self) -> bool {
        *self == Self::PING
    }

    #[inline]
    pub fn is_pong(&self) -> bool {
        *self == Self::PONG
    }

    #[inline]
    pub fn is_privmsg(&self) -> bool {
        *self == Self::PRIVMSG
    }

    #[inline]
    pub fn is_notice(&self) -> bool {
        *self == Self::NOTICE
    }

    pub fn validate(&self) -> Result<(), CommandError> {
        validators::command(self.as_str())
    }
}

impl<'a> From<&'a str> for Commands<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            "CAP" => Self::CAP,
            "AUTHENTICATE" => Self::AUTHENTICATE,
            "PASS" => Self::PASS,
            "NICK" => Self::NICK,
            "USER" => Self::USER,
            "PING" => Self::PING,
            "PONG" => Self::PONG,
            "OPER" => Self::OPER,
            "QUIT" => Self::QUIT,
            "ERROR" => Self::ERROR,
            "JOIN" => Self::JOIN,
            "PART" => Self::PART,
            "TOPIC" => Self::TOPIC,
            "NAMES" => Self::NAMES,
            "LIST" => Self::LIST,
            "INVITE" => Self::INVITE,
            "KICK" => Self::KICK,
            "MOTD" => Self::MOTD,
            "VERSION" => Self::VERSION,
            "ADMIN" => Self::ADMIN,
            "CONNECT" => Self::CONNECT,
            "LUSERS" => Self::LUSERS,
            "TIME" => Self::TIME,
            "STATS" => Self::STATS,
            "HELP" => Self::HELP,
            "INFO" => Self::INFO,
            "MODE" => Self::MODE,
            "PRIVMSG" => Self::PRIVMSG,
            "NOTICE" => Self::NOTICE,
            "WHO" => Self::WHO,
            "WHOIS" => Self::WHOIS,
            "WHOWAS" => Self::WHOWAS,
            "KILL" => Self::KILL,
            "REHASH" => Self::REHASH,
            "RESTART" => Self::RESTART,
            "SQUIT" => Self::SQUIT,
            "AWAY" => Self::AWAY,
            "LINKS" => Self::LINKS,
            "USERHOST" => Self::USERHOST,
            "WALLOPS" => Self::WALLOPS,
            _ => match value.parse::<u16>() {
                Ok(_) => Self::NUMERIC(value),
                Err(_) => Self::CUSTOM(value),
            },
        }
    }
}

impl fmt::Display for Commands<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapSubCommands {
    LS,
    LIST,
    REQ,
    ACK,
    NAK,
    END,
    NEW, // 302
    DEL, // 302
    UNKNOWN(String),
}

impl CapSubCommands {
    pub fn get_description(&self) -> &'static str {
        match self {
            Self::LS => "List available capabilities",
            Self::LIST => "List currently enabled capabilities",
            Self::REQ => "Request capabilities",
            Self::ACK => "Acknowledge capabilities",
            Self::NAK => "Reject capability request",
            Self::END => "End capability negotiation",
            Self::NEW => "Advertise new capabilities (CAP 302)",
            Self::DEL => "Remove previously available capabilities (CAP 302)",
            Self::UNKNOWN(_) => "Unknown CAP subcommand",
        }
    }
}

impl From<&str> for CapSubCommands {
    fn from(value: &str) -> Self {
        match value {
            "LS" => Self::LS,
            "LIST" => Self::LIST,
            "REQ" => Self::REQ,
            "ACK" => Self::ACK,
            "NAK" => Self::NAK,
            "END" => Self::END,
            "NEW" => Self::NEW,
            "DEL" => Self::DEL,
            _ => Self::UNKNOWN(value.to_string()),
        }
    }
}

impl fmt::Display for CapSubCommands {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LS => write!(f, "LS"),
            Self::LIST => write!(f, "LIST"),
            Self::REQ => write!(f, "REQ"),
            Self::ACK => write!(f, "ACK"),
            Self::NAK => write!(f, "NAK"),
            Self::END => write!(f, "END"),
            Self::NEW => write!(f, "NEW"),
            Self::DEL => write!(f, "DEL"),
            Self::UNKNOWN(s) => write!(f, "{}", s),
        }
    }
}
