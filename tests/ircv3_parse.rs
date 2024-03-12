use ircv3_parse::ircv3_parse;
use pretty_assertions::assert_eq;

#[test]
fn ircv3_parse_base() {
    let msg = ":foo!foo@foo.tmi.twitch.tv PRIVMSG #bar :bleedPurple";
    let (tags, prefix, command, params) = ircv3_parse(msg);

    assert_eq!(tags.as_ref(), &None);
    assert_eq!(prefix.server_nick(), Some("foo"));
    assert_eq!(prefix.user(), Some("foo@foo.tmi.twitch.tv"));
    assert_eq!(command, "PRIVMSG");
    assert_eq!(params.channel(), "#bar");
    assert_eq!(params.message(), "bleedPurple");
}

#[test]
fn ircv3_parse_base_rn() {
    let msg = ":foo!foo@foo.tmi.twitch.tv PRIVMSG #bar :bleedPurple\r\n";
    let (tags, prefix, command, params) = ircv3_parse(msg);

    let expect_tags = None;
    assert_eq!(tags.as_ref(), &expect_tags);
    assert_eq!(prefix.server_nick(), Some("foo"));
    assert_eq!(prefix.user(), Some("foo@foo.tmi.twitch.tv"));
    assert_eq!(command, "PRIVMSG");
    assert_eq!(params.channel(), "#bar");
    assert_eq!(params.message(), "bleedPurple");
}

#[test]
fn ircv3_parse_with_tags() {
    let msg = "@badge-info=;badges=turbo/1;color=#0D4200;display-name=ronni;emotes=25:0-4,12-16/1902:6-10;id=b34ccfc7-4977-403a-8a94-33c6bac34fb8;mod=0;room-id=1337;subscriber=0;tmi-sent-ts=1507246572675;turbo=1;user-id=1337;user-type=global_mod :ronni!ronni@ronni.tmi.twitch.tv PRIVMSG #ronni :Kappa Keepo Kappa";
    let (tags, prefix, command, params) = ircv3_parse(msg);

    assert_eq!(tags.get("display-name"), Some("ronni"));
    assert_eq!(tags.get("none"), None);
    assert_eq!(tags.get("id"), Some("b34ccfc7-4977-403a-8a94-33c6bac34fb8"));
    assert_eq!(prefix.server_nick(), Some("ronni"));
    assert_eq!(prefix.user(), Some("ronni@ronni.tmi.twitch.tv"));
    assert_eq!(command, "PRIVMSG");
    assert_eq!(params.channel(), "#ronni");
    assert_eq!(params.message(), "Kappa Keepo Kappa");
}
