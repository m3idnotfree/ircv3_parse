use ircv3_parse::IRCv3;
use pretty_assertions::assert_eq;

#[test]
fn ircv3_parse_base() {
    let msg = ":foo!foo@foo.tmi.twitch.tv PRIVMSG #bar :bleedPurple";
    let ircv3 = IRCv3::parse(msg);

    assert!(ircv3.tags.is_none());
    assert!(ircv3.source.is_some());

    let source = ircv3.source.unwrap();
    assert_eq!("foo", source.servername_nick);
    assert_eq!(Some("foo".into()), source.user);
    assert_eq!(Some("foo.tmi.twitch.tv".into()), source.host,);
    assert_eq!("PRIVMSG", ircv3.command,);

    let params = ircv3.params;
    assert_eq!("#bar".to_string(), params.channel.unwrap().name);
    assert_eq!(Some("bleedPurple".to_string()), params.message,);
}

#[test]
fn ircv3_parse_base_rn() {
    let msg = ":foo!foo@foo.tmi.twitch.tv PRIVMSG #bar :bleedPurple\r\n";
    let ircv3 = IRCv3::parse(msg);

    assert!(ircv3.tags.is_none());
    assert!(ircv3.source.is_some());

    let source = ircv3.source.unwrap();
    assert_eq!("foo", source.servername_nick);
    assert_eq!(Some("foo".into()), source.user);
    assert_eq!(Some("foo.tmi.twitch.tv".into()), source.host);
    assert_eq!("PRIVMSG", ircv3.command,);

    let params = ircv3.params;

    assert_eq!("#bar".to_string(), params.channel.unwrap().name);
    assert_eq!(Some("bleedPurple".to_string()), params.message,);
}

#[test]
fn ircv3_parse_with_tags() {
    let msg = "@badge-info=;badges=turbo/1;color=#0D4200;display-name=ronni;emotes=25:0-4,12-16/1902:6-10;id=b34ccfc7-4977-403a-8a94-33c6bac34fb8;mod=0;room-id=1337;subscriber=0;tmi-sent-ts=1507246572675;turbo=1;user-id=1337;user-type=global_mod :ronni!ronni@ronni.tmi.twitch.tv PRIVMSG #ronni :Kappa Keepo Kappa";
    let ircv3 = IRCv3::parse(msg);

    assert!(ircv3.tags.is_some());
    let tags = ircv3.tags.unwrap();

    // assert_eq!(Some("ronni".to_string()), tags.get("display-name"));
    // assert_eq!(None, tags.get("none"));
    // assert_eq!(
    //     Some("b34ccfc7-4977-403a-8a94-33c6bac34fb8".to_string()),
    //     tags.get("id")
    // );

    assert!(ircv3.source.is_some());

    let source = ircv3.source.unwrap();
    assert_eq!("ronni".to_string(), source.servername_nick);
    assert_eq!(Some("ronni".into()), source.user);
    assert_eq!(Some("ronni.tmi.twitch.tv".into()), source.host);
    assert_eq!("PRIVMSG", ircv3.command);

    let params = ircv3.params;
    assert_eq!("#ronni".to_string(), params.channel.unwrap().name);
    assert_eq!(Some("Kappa Keepo Kappa".to_string()), params.message,);
}

#[test]
fn ircv3_params_parse() {
    let msg = ":foo!foo@foo.tmi.twitch.tv PRIVMSG bar = #bar :bleedPurple";
    let ircv3 = IRCv3::parse(msg);

    let channel = ircv3.params.channel.unwrap();
    assert_eq!("#bar".to_string(), channel.name);
    assert_eq!(Some("bar".to_string()), channel.alt);

    let msg = ":foo!foo@foo.tmi.twitch.tv PRIVMSG bar #bar :bleedPurple";
    let ircv3 = IRCv3::parse(msg);

    let channel = ircv3.params.channel.unwrap();
    assert_eq!("#bar".to_string(), channel.name);
    assert_eq!(Some("bar".to_string()), channel.alt);
}

#[test]
fn unknown_params_test() {
    let msg = ":foo!foo@foo.tmi.twitch.tv PRIVMSG guest w :bleedPurple";
    let result = IRCv3::parse(msg);

    assert_eq!("guest w".to_string(), result.params.unknwon)
}
