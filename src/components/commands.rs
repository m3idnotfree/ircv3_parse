use crate::compat::{Display, FmtResult, Formatter, String, ToString};

use crate::{error::CommandError, validators};

/// IRC command types following RFC 1459 and RFC 2812
///
/// **Important:** all IRC commands are case-insensitive.
///
/// - [`Commands::from()`] accepts any case (PRIVMSG, privmsg, Privmsg)
/// - Comparisons via [`PartialEq`] are case-insensitive
/// - [`Commands::as_str()`] always returns uppercase (canonical form)
///
/// # Example
///
/// let cmd = Commands::from("PRIVMSG");
#[derive(Debug, Clone, Copy, Eq, Hash)]
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
    pub fn as_str(&self) -> &'a str {
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
    pub fn as_bytes(&self) -> &'a [u8] {
        match self {
            Self::NUMERIC(num) => num.as_bytes(),
            Self::CAP => b"CAP",
            Self::AUTHENTICATE => b"AUTHENTICATE",
            Self::PASS => b"PASS",
            Self::NICK => b"NICK",
            Self::USER => b"USER",
            Self::PING => b"PING",
            Self::PONG => b"PONG",
            Self::OPER => b"OPER",
            Self::QUIT => b"QUIT",
            Self::ERROR => b"ERROR",
            Self::JOIN => b"JOIN",
            Self::PART => b"PART",
            Self::TOPIC => b"TOPIC",
            Self::NAMES => b"NAMES",
            Self::LIST => b"LIST",
            Self::INVITE => b"INVITE",
            Self::KICK => b"KICK",
            Self::MOTD => b"MOTD",
            Self::VERSION => b"VERSION",
            Self::ADMIN => b"ADMIN",
            Self::CONNECT => b"CONNECT",
            Self::LUSERS => b"LUSERS",
            Self::TIME => b"TIME",
            Self::STATS => b"STATS",
            Self::HELP => b"HELP",
            Self::INFO => b"INFO",
            Self::MODE => b"MODE",
            Self::PRIVMSG => b"PRIVMSG",
            Self::NOTICE => b"NOTICE",
            Self::WHO => b"WHO",
            Self::WHOIS => b"WHOIS",
            Self::WHOWAS => b"WHOWAS",
            Self::KILL => b"KILL",
            Self::REHASH => b"REHASH",
            Self::RESTART => b"RESTART",
            Self::SQUIT => b"SQUIT",
            Self::AWAY => b"AWAY",
            Self::LINKS => b"LINKS",
            Self::USERHOST => b"USERHOST",
            Self::WALLOPS => b"WALLOPS",
            Self::CUSTOM(unknown) => unknown.as_bytes(),
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

macro_rules! parse_command {
    ($value:expr, $($pattern:literal => $command:expr),* $(,)?) => {
        $(
            if $value.eq_ignore_ascii_case($pattern) {
                return $command;
        }
        )*
    };
}

impl<'a> From<&'a str> for Commands<'a> {
    fn from(value: &'a str) -> Self {
        parse_command!(value,
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
        );

        if value.parse::<u16>().is_ok() {
            return Self::NUMERIC(value);
        }

        Self::CUSTOM(value)
    }
}

impl Display for Commands<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for Commands<'_> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl PartialEq for Commands<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<&str> for Commands<'_> {
    fn eq(&self, other: &&str) -> bool {
        self.as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<String> for Commands<'_> {
    fn eq(&self, other: &String) -> bool {
        self.as_str().eq_ignore_ascii_case(other.as_str())
    }
}

impl PartialEq<Commands<'_>> for &str {
    fn eq(&self, other: &Commands<'_>) -> bool {
        other.as_str().eq_ignore_ascii_case(self)
    }
}

impl PartialEq<Commands<'_>> for String {
    fn eq(&self, other: &Commands<'_>) -> bool {
        other.as_str().eq_ignore_ascii_case(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    pub fn as_str(&self) -> &str {
        match self {
            Self::LS => "LS",
            Self::LIST => "LIST",
            Self::REQ => "REQ",
            Self::ACK => "ACK",
            Self::NAK => "NAK",
            Self::END => "END",
            Self::NEW => "NEW",
            Self::DEL => "DEL",
            Self::UNKNOWN(s) => s,
        }
    }

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

impl Display for CapSubCommands {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Commands<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for CapSubCommands {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::components::Commands;

    #[test]
    fn from_case_insensitive() {
        assert_eq!(Commands::from("PRIVMSG"), Commands::PRIVMSG);
        assert_eq!(Commands::from("PING"), Commands::PING);

        assert_eq!(Commands::from("privmsg"), Commands::PRIVMSG);
        assert_eq!(Commands::from("ping"), Commands::PING);

        assert_eq!(Commands::from("PrivMsg"), Commands::PRIVMSG);
        assert_eq!(Commands::from("PiNg"), Commands::PING);
    }

    #[test]
    fn partialeq_case_insensitive() {
        let cmd = Commands::PRIVMSG;

        assert!(cmd == "PRIVMSG");
        assert!(cmd == "privmsg");
        assert!(cmd == "PrivMsg");
        assert!(cmd == "PRIVMSG");

        assert!(cmd != "PING");
        assert!(cmd != "ping");
    }

    #[test]
    fn string_comparison() {
        let cmd = Commands::PRIVMSG;
        let s = String::from("privmsg");

        assert!(cmd == s);
        assert!(s == cmd);
    }
}
