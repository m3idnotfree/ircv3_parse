use ircv3_parse_derive::FromMessage;

#[test]
fn basic() {
    #[derive(FromMessage, Debug, PartialEq)]
    #[irc(tag_flag = "moderator")]
    enum Moderator {
        #[irc(present)]
        Present,
        Absent,
    }

    let msg: Moderator = ircv3_parse::from_str("@moderator PRIVMSG #channel :hi").unwrap();
    assert_eq!(Moderator::Present, msg);

    let msg: Moderator = ircv3_parse::from_str("@moderator=m3id PRIVMSG #channel :hi").unwrap();
    assert_eq!(Moderator::Absent, msg);

    let msg: Moderator = ircv3_parse::from_str("@other PRIVMSG #channel :hi").unwrap();
    assert_eq!(Moderator::Absent, msg);

    let err = ircv3_parse::from_str::<Moderator>("PRIVMSG #channel :hi").unwrap_err();
    assert!(err.is_tags_component_not_found());
}

#[test]
fn default() {
    #[derive(FromMessage, Debug, PartialEq)]
    #[irc(tag_flag = "moderator", default = "Absent")]
    enum Moderator {
        #[irc(present)]
        Present,
        Absent,
    }

    let msg: Moderator = ircv3_parse::from_str("@moderator PRIVMSG #channel :hi").unwrap();
    assert_eq!(Moderator::Present, msg);

    let msg: Moderator = ircv3_parse::from_str("@moderator=m3id PRIVMSG #channel :hi").unwrap();
    assert_eq!(Moderator::Absent, msg);

    let msg: Moderator = ircv3_parse::from_str("@other PRIVMSG #channel :hi").unwrap();
    assert_eq!(Moderator::Absent, msg);

    let msg: Moderator = ircv3_parse::from_str("PRIVMSG #channel :hi").unwrap();
    assert_eq!(Moderator::Absent, msg);

    let msg: Moderator = ircv3_parse::from_str("PRIVMSG #channel :hi").unwrap();
    assert_eq!(Moderator::Absent, msg);
}
