use ircv3_parse::{prefix_parse, IRCv3Source};
use nom::IResult;
use pretty_assertions::assert_eq;

#[test]
fn prefix_base() {
    let msg = ":foo!foo@foo.tmi.twitch.tv JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    let (remain, prefix) = parse(msg).unwrap();

    let expect_remain = "JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";

    assert_eq!(remain, expect_remain);
    let prefix = prefix.unwrap();
    assert_eq!(prefix.servername_nick, "foo");
    assert_eq!(prefix.user, Some("foo".into()));
    assert_eq!(prefix.host, Some("foo.tmi.twitch.tv".into()));
}

#[test]
fn prefix_only_server() {
    let msg = ":foo.tmi.twitch.tv JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    let (remain, prefix) = parse(msg).unwrap();

    let expect_remain = "JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    assert_eq!(remain, expect_remain);

    assert!(prefix.is_some());

    let prefix = prefix.unwrap();
    assert_eq!(prefix.servername_nick, "foo.tmi.twitch.tv");
    assert_eq!(prefix.user, None);
}

#[test]
fn prefix_not_exist() {
    let msg = "JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    let (remain, prefix) = parse(msg).unwrap();

    let expect_remain = "JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    assert_eq!(remain, expect_remain);
    assert!(prefix.is_none());
}

fn parse(msg: &str) -> IResult<&str, Option<IRCv3Source>> {
    prefix_parse(msg)
}
