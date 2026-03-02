use ircv3_parse_derive::{FromMessage, ToMessage};

#[test]
fn basic() {
    #[derive(Debug, PartialEq, FromMessage, ToMessage)]
    #[irc(tag_flag = "moderator")]
    enum Moderator {
        #[irc(present)]
        Present,
        Absent,
    }

    let msg: Moderator = ircv3_parse::from_str("@moderator PRIVMSG #channel :hi").unwrap();
    assert_eq!(Moderator::Present, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("@moderator ", output);

    let msg: Moderator = ircv3_parse::from_str("@moderator=m3id PRIVMSG #channel :hi").unwrap();
    assert_eq!(Moderator::Absent, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("", output);

    let msg: Moderator = ircv3_parse::from_str("@other PRIVMSG #channel :hi").unwrap();
    assert_eq!(Moderator::Absent, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("", output);

    let err = ircv3_parse::from_str::<Moderator>("PRIVMSG #channel :hi").unwrap_err();
    assert!(err.is_tags_component_not_found());
}

#[test]
fn default() {
    #[derive(Debug, PartialEq, FromMessage, ToMessage)]
    #[irc(tag_flag = "moderator", default = "Absent")]
    enum Moderator {
        #[irc(present)]
        Present,
        Absent,
    }

    let msg: Moderator = ircv3_parse::from_str("@moderator PRIVMSG #channel :hi").unwrap();
    assert_eq!(Moderator::Present, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("@moderator ", output);

    let msg: Moderator = ircv3_parse::from_str("@moderator=m3id PRIVMSG #channel :hi").unwrap();
    assert_eq!(Moderator::Absent, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("", output);

    let msg: Moderator = ircv3_parse::from_str("@other PRIVMSG #channel :hi").unwrap();
    assert_eq!(Moderator::Absent, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("", output);

    let msg: Moderator = ircv3_parse::from_str("PRIVMSG #channel :hi").unwrap();
    assert_eq!(Moderator::Absent, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("", output);

    let msg: Moderator = ircv3_parse::from_str("PRIVMSG #channel :hi").unwrap();
    assert_eq!(Moderator::Absent, msg);
}
