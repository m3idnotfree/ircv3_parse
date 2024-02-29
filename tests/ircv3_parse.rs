use ircv3_parse::{self, Ircv3Parse};
use ircv3_tags::Ircv3TagsParse;

use pretty_assertions::assert_eq;

#[test]
fn ircv3_parse_base() {
    let msg = ":foo!foo@foo.tmi.twitch.tv PRIVMSG #bar :bleedPurple";
    let result = Ircv3Parse::new(msg);
    let (remain, c_m) = result.params.channel_n_message().unwrap();

    let expect_tags = None;
    let expect_prefix = Some(("foo".to_string(), Some("foo@foo.tmi.twitch.tv".to_string())));
    assert_eq!(result.tags.hashmap_str(), expect_tags);
    assert_eq!(result.prefix.to_string(), expect_prefix);
    assert_eq!(result.command, "PRIVMSG");
    assert_eq!(c_m.channel, "#bar");
    assert_eq!(c_m.message, "bleedPurple");
    assert_eq!(remain, "");
}
#[test]
fn ircv3_parse_base_rn() {
    let msg = ":foo!foo@foo.tmi.twitch.tv PRIVMSG #bar :bleedPurple\r\n";
    let result = Ircv3Parse::new(msg);
    let (remain, c_m) = result.params.channel_n_message().unwrap();

    let expect_tags = None;
    let expect_prefix = Some(("foo".to_string(), Some("foo@foo.tmi.twitch.tv".to_string())));
    assert_eq!(result.tags.hashmap_string(), expect_tags);
    assert_eq!(result.prefix.to_string(), expect_prefix);
    assert_eq!(result.command, "PRIVMSG");
    assert_eq!(c_m.channel, "#bar");
    assert_eq!(c_m.message, "bleedPurple");
    assert_eq!(remain, "\r\n");
}

#[test]
fn ircv3_parse_with_tags() {
    let msg = "@badge-info=;badges=turbo/1;color=#0D4200;display-name=ronni;emotes=25:0-4,12-16/1902:6-10;id=b34ccfc7-4977-403a-8a94-33c6bac34fb8;mod=0;room-id=1337;subscriber=0;tmi-sent-ts=1507246572675;turbo=1;user-id=1337;user-type=global_mod :ronni!ronni@ronni.tmi.twitch.tv PRIVMSG #ronni :Kappa Keepo Kappa";
    let result = Ircv3Parse::new(msg);
    let tags = Ircv3TagsParse::new("@badge-info=;badges=turbo/1;color=#0D4200;display-name=ronni;emotes=25:0-4,12-16/1902:6-10;id=b34ccfc7-4977-403a-8a94-33c6bac34fb8;mod=0;room-id=1337;subscriber=0;tmi-sent-ts=1507246572675;turbo=1;user-id=1337;user-type=global_mod ").hashmap_string();

    let expect_prefix = Some((
        "ronni".to_string(),
        Some("ronni@ronni.tmi.twitch.tv".to_string()),
    ));

    let (remain, c_m) = result.params.channel_n_message().unwrap();
    assert_eq!(result.tags.hashmap_string(), tags);
    assert_eq!(result.prefix.to_string(), expect_prefix);
    assert_eq!(result.command, "PRIVMSG");
    assert_eq!(c_m.channel, "#ronni");
    assert_eq!(c_m.message, "Kappa Keepo Kappa");
    assert_eq!(remain, "");
}
