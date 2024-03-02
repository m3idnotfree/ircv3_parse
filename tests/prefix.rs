use ircv3_parse::Ircv3Prefix;
use pretty_assertions::assert_eq;

#[test]
fn prefix_base() {
    let msg = ":foo!foo@foo.tmi.twitch.tv JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    let (remain, prefix) = Ircv3Prefix::parse(msg).unwrap();
    let expect_prefix = Some(("foo", Some("foo@foo.tmi.twitch.tv")));

    let expect_remain = "JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    assert_eq!(remain, expect_remain);
    assert_eq!(prefix.to_str(), expect_prefix);
}

#[test]
fn prefix_only_server() {
    let msg = ":foo.tmi.twitch.tv JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    let (remain, prefix) = Ircv3Prefix::parse(msg).unwrap();
    let expect_prefix = Some(("foo.tmi.twitch.tv", None));

    let expect_remain = "JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    assert_eq!(remain, expect_remain);
    assert_eq!(prefix.to_str(), expect_prefix);
}

#[test]
fn prefix_not_exist_to_string() {
    let msg = "JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    let (remain, prefix) = Ircv3Prefix::parse(msg).unwrap();

    let expect_remain = "JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    assert_eq!(remain, expect_remain);
    assert_eq!(prefix.to_string(), None);
}
#[test]
fn prefix_base_to_string() {
    let msg = ":foo!foo@foo.tmi.twitch.tv JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    let (remain, prefix) = Ircv3Prefix::parse(msg).unwrap();
    let expect_prefix = Some(("foo".to_string(), Some("foo@foo.tmi.twitch.tv".to_string())));

    let expect_remain = "JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    assert_eq!(remain, expect_remain);
    assert_eq!(prefix.to_string(), expect_prefix);
}

#[test]
fn prefix_only_server_to_string() {
    let msg = ":foo.tmi.twitch.tv JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    let (remain, prefix) = Ircv3Prefix::parse(msg).unwrap();
    let expect_prefix = Some(("foo.tmi.twitch.tv".to_string(), None));

    let expect_remain = "JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    assert_eq!(remain, expect_remain);
    assert_eq!(prefix.to_string(), expect_prefix);
}

#[test]
fn prefix_not_exist() {
    let msg = "JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    let (remain, prefix) = Ircv3Prefix::parse(msg).unwrap();

    let expect_remain = "JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    assert_eq!(remain, expect_remain);
    assert_eq!(prefix.to_string(), None);
}
