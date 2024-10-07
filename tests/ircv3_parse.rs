use ircv3_parse::ircv3_parse;
use pretty_assertions::assert_eq;

#[test]
fn ircv3_parse_base() {
    let msg = ":foo!foo@foo.tmi.twitch.tv PRIVMSG #bar :bleedPurple";
    let ircvd = ircv3_parse(msg);

    assert!(ircvd.tags.is_none());
    assert!(ircvd.prefix.is_some());

    let prefix = ircvd.prefix.unwrap();
    assert_eq!(prefix.servername_nick, "foo");
    assert_eq!(prefix.user, Some("foo".into()));
    assert_eq!(prefix.host, Some("foo.tmi.twitch.tv".into()));
    assert_eq!(ircvd.command, "PRIVMSG");

    let params = ircvd.params;
    assert_eq!(params.channel, Some("bar".to_string()));
    assert_eq!(params.message, Some("bleedPurple".to_string()));
}

#[test]
fn ircv3_parse_base_rn() {
    let msg = ":foo!foo@foo.tmi.twitch.tv PRIVMSG #bar :bleedPurple\r\n";
    let ircvd = ircv3_parse(msg);

    assert!(ircvd.tags.is_none());
    assert!(ircvd.prefix.is_some());

    let prefix = ircvd.prefix.unwrap();
    assert_eq!(prefix.servername_nick, "foo");
    assert_eq!(prefix.user, Some("foo".into()));
    assert_eq!(prefix.host, Some("foo.tmi.twitch.tv".into()));
    assert_eq!(ircvd.command, "PRIVMSG");

    assert_eq!(ircvd.params.channel, Some("bar".into()));
    assert_eq!(ircvd.params.message, Some("bleedPurple".to_string()));
}

#[test]
fn ircv3_parse_with_tags() {
    let msg = "@badge-info=;badges=turbo/1;color=#0D4200;display-name=ronni;emotes=25:0-4,12-16/1902:6-10;id=b34ccfc7-4977-403a-8a94-33c6bac34fb8;mod=0;room-id=1337;subscriber=0;tmi-sent-ts=1507246572675;turbo=1;user-id=1337;user-type=global_mod :ronni!ronni@ronni.tmi.twitch.tv PRIVMSG #ronni :Kappa Keepo Kappa";
    let ircvd = ircv3_parse(msg);

    assert!(ircvd.tags.is_some());
    let tags = ircvd.tags.unwrap();

    assert_eq!(tags.get("display-name"), Some("ronni".to_string()));
    assert_eq!(tags.get("none"), None);
    assert_eq!(
        tags.get("id"),
        Some("b34ccfc7-4977-403a-8a94-33c6bac34fb8".to_string())
    );

    assert!(ircvd.prefix.is_some());

    let prefix = ircvd.prefix.unwrap();
    assert_eq!(prefix.servername_nick, "ronni".to_string());
    assert_eq!(prefix.user, Some("ronni".into()));
    assert_eq!(prefix.host, Some("ronni.tmi.twitch.tv".into()));
    assert_eq!(ircvd.command, "PRIVMSG");

    assert_eq!(ircvd.params.channel, Some("ronni".to_string()));
    assert_eq!(ircvd.params.message, Some("Kappa Keepo Kappa".to_string()));
}

fn ircv3_parse_twitch() {
    let msg = ":tmi.twitch.tv 001 <user> :Welcome, GLHF!\r\n:tmi.twitch.tv 002 <user> :Your host is tmi.twitch.tv\r\n:tmi.twitch.tv 003 <user> :This server is rather new\r\n:tmi.twitch.tv 004 <user> :-\r\n:tmi.twitch.tv 375 <user> :-\r\n:tmi.twitch.tv 372 <user> :You are in a maze of twisty passages, all alike.\r\n:tmi.twitch.tv 376 <user> :>\r\n@badge-info=;badges=;color=;display-name=<user>;emote-sets=0,300374282;user-id=12345678;user-type= :tmi.twitch.tv GLOBALUSERSTATE\r\n";
    let ircvd = ircv3_parse(msg);

    assert!(ircvd.prefix.is_some());

    let prefix = ircvd.prefix.unwrap();
    assert_eq!("tmi.wtich.tv", prefix.servername_nick);
    assert_eq!("001".to_string(), ircvd.command)
}
