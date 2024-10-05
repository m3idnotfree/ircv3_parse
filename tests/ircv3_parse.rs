use ircv3_parse::ircv3_parse;
use pretty_assertions::assert_eq;

#[test]
fn ircv3_parse_base() {
    let msg = ":foo!foo@foo.tmi.twitch.tv PRIVMSG #bar :bleedPurple";
    let (tags, prefix, command, params) = ircv3_parse(msg);

    assert!(tags.is_none());
    assert!(prefix.is_some());

    let prefix = prefix.unwrap();
    assert_eq!(prefix.server_nick(), "foo");
    assert_eq!(prefix.user(), Some("foo@foo.tmi.twitch.tv".into()));
    assert_eq!(command, "PRIVMSG");
    assert_eq!(params.channel(), Some("#bar"));
    assert_eq!(params.message(), Some("bleedPurple"));
}

#[test]
fn ircv3_parse_base_rn() {
    let msg = ":foo!foo@foo.tmi.twitch.tv PRIVMSG #bar :bleedPurple\r\n";
    let (tags, prefix, command, params) = ircv3_parse(msg);

    assert!(tags.is_none());
    assert!(prefix.is_some());

    let prefix = prefix.unwrap();
    assert_eq!(prefix.server_nick(), "foo");
    assert_eq!(prefix.user(), Some("foo@foo.tmi.twitch.tv".into()));
    assert_eq!(command, "PRIVMSG");
    assert_eq!(params.channel(), Some("#bar"));
    assert_eq!(params.message(), Some("bleedPurple"));
}

#[test]
fn ircv3_parse_with_tags() {
    let msg = "@badge-info=;badges=turbo/1;color=#0D4200;display-name=ronni;emotes=25:0-4,12-16/1902:6-10;id=b34ccfc7-4977-403a-8a94-33c6bac34fb8;mod=0;room-id=1337;subscriber=0;tmi-sent-ts=1507246572675;turbo=1;user-id=1337;user-type=global_mod :ronni!ronni@ronni.tmi.twitch.tv PRIVMSG #ronni :Kappa Keepo Kappa";
    let (tags, prefix, command, params) = ircv3_parse(msg);

    assert!(tags.is_some());
    let tags = tags.unwrap();

    assert_eq!(tags.get("display-name"), Some("ronni".to_string()));
    assert_eq!(tags.get("none"), None);
    assert_eq!(
        tags.get("id"),
        Some("b34ccfc7-4977-403a-8a94-33c6bac34fb8".to_string())
    );

    assert!(prefix.is_some());

    let prefix = prefix.unwrap();
    assert_eq!(prefix.server_nick(), "ronni");
    assert_eq!(prefix.user(), Some("ronni@ronni.tmi.twitch.tv".into()));
    assert_eq!(command, "PRIVMSG");
    assert_eq!(params.channel(), Some("#ronni"));
    assert_eq!(params.message(), Some("Kappa Keepo Kappa"));
}
