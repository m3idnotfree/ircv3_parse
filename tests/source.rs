use ircv3_parse::source_parse;
use pretty_assertions::assert_eq;

#[test]
fn prefix_base() {
    let msg = ":foo!foo@foo.tmi.twitch.tv JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    let (remain, source) = source_parse(msg).unwrap();

    let expect_remain = "JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";

    assert_eq!(remain, expect_remain);
    let source = source.unwrap();
    assert_eq!("foo", source.servername_nick);
    assert_eq!(Some("foo".into()), source.user);
    assert_eq!(Some("foo.tmi.twitch.tv".into()), source.host);
}

#[test]
fn prefix_only_server() {
    let msg = ":foo.tmi.twitch.tv JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    let (remain, source) = source_parse(msg).unwrap();

    let expect_remain = "JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    assert_eq!(remain, expect_remain);

    assert!(source.is_some());

    let source = source.unwrap();
    assert_eq!("foo.tmi.twitch.tv", source.servername_nick);
    assert_eq!(None, source.user,);
}

#[test]
fn prefix_not_exist() {
    let msg = "JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    let (remain, source) = source_parse(msg).unwrap();

    let expect_remain = "JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    assert_eq!(expect_remain, remain);
    assert!(source.is_none());
}
