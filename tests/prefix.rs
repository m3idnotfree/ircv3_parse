use ircv3_parse::IRCv3Prefix;
use nom::IResult;
use pretty_assertions::assert_eq;

#[test]
fn prefix_base() {
    let msg = ":foo!foo@foo.tmi.twitch.tv JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    let (remain, prefix) = parse(msg).unwrap();

    let expect_remain = "JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";

    assert_eq!(remain, expect_remain);
    assert_eq!(prefix.server_nick(), Some("foo"));
    assert_eq!(prefix.user(), Some("foo@foo.tmi.twitch.tv"));
}

#[test]
fn prefix_only_server() {
    let msg = ":foo.tmi.twitch.tv JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    let (remain, prefix) = parse(msg).unwrap();

    let expect_remain = "JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    assert_eq!(remain, expect_remain);
    assert_eq!(prefix.server_nick(), Some("foo.tmi.twitch.tv"));
    assert_eq!(prefix.user(), None);
}

#[test]
fn prefix_not_exist() {
    let msg = "JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    let (remain, prefix) = parse(msg).unwrap();

    let expect_remain = "JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    assert_eq!(remain, expect_remain);
    assert_eq!(prefix.server_nick(), None);
    assert_eq!(prefix.user(), None);
}

fn parse(msg: &str) -> IResult<&str, IRCv3Prefix> {
    IRCv3Prefix::parse(msg)
}
