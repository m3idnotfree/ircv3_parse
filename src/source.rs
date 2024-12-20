use nom::{
    bytes::complete::{tag, take_while},
    character::complete::space1,
    combinator::opt,
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[derive(Debug, Clone, PartialEq)]
pub struct IRCv3Source {
    pub servername_nick: String,
    pub user: Option<String>,
    pub host: Option<String>,
}

pub fn source_parse(msg: &str) -> IResult<&str, Option<IRCv3Source>> {
    let (remain, data) = opt(delimited(
        tag(":"),
        tuple((server_nick, opts_user, opts_host)),
        space1,
    ))(msg)?;
    Ok((
        remain,
        data.map(|sources| IRCv3Source {
            servername_nick: sources.0.to_string(),
            user: sources.1.map(String::from),
            host: sources.2.map(String::from),
        }),
    ))
}

fn server_nick(msg: &str) -> IResult<&str, &str> {
    take_while(|c: char| !c.is_whitespace() && c != '!')(msg)
}

fn opts_user(msg: &str) -> IResult<&str, Option<&str>> {
    opt(preceded(
        tag("!"),
        take_while(|c: char| !c.is_whitespace() && c != '@'),
    ))(msg)
}
fn opts_host(msg: &str) -> IResult<&str, Option<&str>> {
    opt(preceded(tag("@"), take_while(|c: char| !c.is_whitespace())))(msg)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::source::source_parse;

    #[test]
    fn prefix_base() {
        let msg =
            ":foo!foo@foo.tmi.twitch.tv JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
        let (remain, source) = source_parse(msg).unwrap();

        let expected_remain = "JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";

        assert_eq!(remain, expected_remain);
        let source = source.unwrap();
        assert_eq!("foo", source.servername_nick);
        assert_eq!(Some("foo".into()), source.user);
        assert_eq!(Some("foo.tmi.twitch.tv".into()), source.host);
    }

    #[test]
    fn prefix_only_server() {
        let msg = ":foo.tmi.twitch.tv JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
        let (remain, source) = source_parse(msg).unwrap();

        let expected_remain = "JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
        assert_eq!(remain, expected_remain);

        assert!(source.is_some());

        let source = source.unwrap();
        assert_eq!("foo.tmi.twitch.tv", source.servername_nick);
        assert_eq!(None, source.user,);
    }

    #[test]
    fn prefix_not_exist() {
        let msg = "JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
        let (remain, source) = source_parse(msg).unwrap();

        let expected_remain = "JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
        assert_eq!(expected_remain, remain);
        assert!(source.is_none());
    }
}
