use ircv3_parse::Ircv3Prefix;
use pretty_assertions::assert_eq;

#[test]
fn prefix_base() {
    let msg = ":foo!foo@foo.tmi.twitch.tv JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    let (msg, prefix) = Ircv3Prefix::new(msg).str();
    let expect_prefix = Some(("foo", Some("foo@foo.tmi.twitch.tv")));

    let expect_remain = "JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    assert_eq!(prefix, expect_prefix);
    assert_eq!(msg, expect_remain);
}

#[test]
fn prefix_only_server() {
    let msg = ":foo.tmi.twitch.tv JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    let (msg, prefix) = Ircv3Prefix::new(msg).str();
    let expect_prefix = Some(("foo.tmi.twitch.tv", None));

    let expect_remain = "JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    assert_eq!(msg, expect_remain);
    assert_eq!(prefix, expect_prefix);
}

#[test]
fn prefix_not_exist_string() {
    let msg = "JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    let (msg, prefix) = Ircv3Prefix::new(msg).string();

    let expect_remain = "JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    assert_eq!(msg, expect_remain);
    assert_eq!(prefix, None);
}
#[test]
fn prefix_base_string() {
    let msg = ":foo!foo@foo.tmi.twitch.tv JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    let (msg, prefix) = Ircv3Prefix::new(msg).string();
    let expect_prefix = Some(("foo".to_string(), Some("foo@foo.tmi.twitch.tv".to_string())));

    let expect_remain = "JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    assert_eq!(prefix, expect_prefix);
    assert_eq!(msg, expect_remain);
}

#[test]
fn prefix_only_server_string() {
    let msg = ":foo.tmi.twitch.tv JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    let (msg, prefix) = Ircv3Prefix::new(msg).string();
    let expect_prefix = Some(("foo.tmi.twitch.tv".to_string(), None));

    let expect_remain = "JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    assert_eq!(msg, expect_remain);
    assert_eq!(prefix, expect_prefix);
}

#[test]
fn prefix_not_exist() {
    let msg = "JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    let (msg, prefix) = Ircv3Prefix::new(msg).str();

    let expect_remain = "JOIN #bar\r\n:foo.tmi.twitch.tv 353 foo = #bar :foo\r\n";
    assert_eq!(msg, expect_remain);
    assert_eq!(prefix, None);
}
