use ircv3_parse_derive::FromMessage;

#[test]
fn name() {
    #[derive(FromMessage)]
    struct Source {
        #[irc(source = "name")]
        nick: String,
    }

    let input = ":nick!user@example.com PRIVMSG #channel :hi";
    let msg: Source = ircv3_parse::from_str(input).unwrap();
    assert_eq!("nick", msg.nick);
}

#[test]
fn user() {
    #[derive(FromMessage)]
    struct Source {
        #[irc(source = "user")]
        user: Option<String>,
    }

    let input = ":nick!user@example.com PRIVMSG #channel :hi";
    let msg: Source = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Some("user".to_string()), msg.user);
}

#[test]
fn user_missing() {
    #[derive(FromMessage)]
    struct Source {
        #[irc(source = "user")]
        user: Option<String>,
    }

    let input = ":nick PRIVMSG #channel :hi";
    let msg: Source = ircv3_parse::from_str(input).unwrap();
    assert_eq!(None, msg.user);
}

#[test]
fn host() {
    #[derive(FromMessage)]
    struct Source {
        #[irc(source = "host")]
        host: Option<String>,
    }

    let input = ":nick!user@host PRIVMSG #channel :hi";
    let msg: Source = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Some("host".to_string()), msg.host);
}

#[test]
fn host_missing() {
    #[derive(FromMessage)]
    struct Source {
        #[irc(source = "host")]
        host: Option<String>,
    }

    let input = ":nick PRIVMSG #channel :hi";
    let msg: Source = ircv3_parse::from_str(input).unwrap();
    assert_eq!(None, msg.host);
}

#[test]
fn default() {
    #[derive(FromMessage)]
    struct Source {
        #[irc(source)]
        nick: String,
    }

    let input = ":nick!user@example.com PRIVMSG #channel :hi";
    let msg: Source = ircv3_parse::from_str(input).unwrap();
    assert_eq!("nick", msg.nick);
}

#[test]
fn all_fields() {
    #[derive(FromMessage)]
    struct Source {
        #[irc(source = "name")]
        nick: String,
        #[irc(source = "user")]
        user: Option<String>,
        #[irc(source = "host")]
        host: Option<String>,
    }

    let input = ":nick!user@example.com PRIVMSG #channel :hi";
    let msg: Source = ircv3_parse::from_str(input).unwrap();
    assert_eq!("nick", msg.nick);
    assert_eq!(Some("user".to_string()), msg.user);
    assert_eq!(Some("example.com".to_string()), msg.host);
}

#[test]
fn unnamed_name() {
    #[derive(FromMessage)]
    struct Source(#[irc(source = "name")] String);

    let input = ":nick!user@example.com PRIVMSG #channel :hi";
    let msg: Source = ircv3_parse::from_str(input).unwrap();
    assert_eq!("nick", msg.0);
}

#[test]
fn unnamed_user() {
    #[derive(FromMessage)]
    struct Source(#[irc(source = "user")] Option<String>);

    let input = ":nick!user@example.com PRIVMSG #channel :hi";
    let msg: Source = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Some("user".to_string()), msg.0);
}

#[test]
fn unnamed_user_missing() {
    #[derive(FromMessage)]
    struct Source(#[irc(source = "user")] Option<String>);

    let input = ":nick PRIVMSG #channel :hi";
    let msg: Source = ircv3_parse::from_str(input).unwrap();
    assert_eq!(None, msg.0);
}

#[test]
fn unnamed_host() {
    #[derive(FromMessage)]
    struct Source(#[irc(source = "host")] Option<String>);

    let input = ":nick!user@example.com PRIVMSG #channel :hi";
    let msg: Source = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Some("example.com".to_string()), msg.0);
}

#[test]
fn unnamed_host_missing() {
    #[derive(FromMessage)]
    struct Source(#[irc(source = "host")] Option<String>);

    let input = ":nick PRIVMSG #channel :hi";
    let msg: Source = ircv3_parse::from_str(input).unwrap();
    assert_eq!(None, msg.0);
}

#[test]
fn unnamed_default() {
    #[derive(FromMessage)]
    struct Source(#[irc(source)] String);

    let input = ":nick!user@example.com PRIVMSG #channel :hi";
    let msg: Source = ircv3_parse::from_str(input).unwrap();
    assert_eq!("nick", msg.0);
}

#[test]
fn unnamed_all_fields() {
    #[derive(FromMessage)]
    struct Source(
        #[irc(source = "name")] String,
        #[irc(source = "user")] Option<String>,
        #[irc(source = "host")] Option<String>,
    );

    let input = ":nick!user@example.com PRIVMSG #channel :hi";
    let msg: Source = ircv3_parse::from_str(input).unwrap();
    assert_eq!("nick", msg.0);
    assert_eq!(Some("user".to_string()), msg.1);
    assert_eq!(Some("example.com".to_string()), msg.2);
}

#[test]
fn nested_name() {
    #[derive(FromMessage, Debug, PartialEq)]
    struct Nick(#[irc(source = "name")] String);

    #[derive(FromMessage)]
    struct Message {
        nick: Nick,
    }

    let input = ":nick!user@example.com PRIVMSG #channel :hi";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Nick("nick".to_string()), msg.nick);
}

#[test]
fn nested_user() {
    #[derive(FromMessage, Debug, PartialEq)]
    struct User(#[irc(source = "user")] Option<String>);

    #[derive(FromMessage)]
    struct Message {
        user: User,
    }

    let input = ":nick!user@example.com PRIVMSG #channel :hi";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(User(Some("user".to_string())), msg.user);
}

#[test]
fn nested_optional() {
    #[derive(FromMessage, Debug, PartialEq)]
    struct Nick(#[irc(source = "name")] String);

    #[derive(FromMessage)]
    struct Message {
        nick: Option<Nick>,
    }

    let input = ":nick!user@example.com PRIVMSG #channel :hi";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Some(Nick("nick".to_string())), msg.nick);
}

#[test]
fn nested_outer_attribute_ignored() {
    #[derive(FromMessage, Debug, PartialEq)]
    struct Nick(#[irc(source = "name")] String);

    #[derive(FromMessage)]
    struct Message {
        #[irc(source = "host")]
        nick: Option<Nick>,
    }

    let input = ":nick!user@example.com PRIVMSG #channel :hi";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Some(Nick("nick".to_string())), msg.nick);
}
