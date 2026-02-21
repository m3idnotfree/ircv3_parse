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

#[test]
fn unit_struct() {
    #[derive(FromMessage, Debug, PartialEq)]
    #[irc(source = "name")]
    struct Nick;

    let input = ":nick!user@example.com PRIVMSG #channel :hi";
    let msg: Nick = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Nick, msg);
}

#[test]
fn name_default_trait_no_component() {
    #[derive(FromMessage)]
    struct Source {
        #[irc(source, default)]
        nick: String,
    }

    let input = "PRIVMSG #channel :hi";
    let msg: Source = ircv3_parse::from_str(input).unwrap();
    assert_eq!("", msg.nick);
}

#[test]
fn name_default_fn_no_component() {
    fn default_nick() -> String {
        "anonymous".to_string()
    }

    #[derive(FromMessage)]
    struct Source {
        #[irc(source, default = "default_nick")]
        nick: String,
    }

    let input = "PRIVMSG #channel :hi";
    let msg: Source = ircv3_parse::from_str(input).unwrap();
    assert_eq!("anonymous", msg.nick);
}

#[test]
fn user_default_trait_no_component() {
    #[derive(FromMessage)]
    struct Source {
        #[irc(source = "user", default)]
        user: String,
    }

    let input = "PRIVMSG #channel :hi";
    let msg: Source = ircv3_parse::from_str(input).unwrap();
    assert_eq!("", msg.user);
}

#[test]
fn user_default_fn_no_component() {
    fn default_user() -> String {
        "anonymous".to_string()
    }

    #[derive(FromMessage)]
    struct Source {
        #[irc(source = "user", default = "default_user")]
        user: String,
    }

    let input = "PRIVMSG #channel :hi";
    let msg: Source = ircv3_parse::from_str(input).unwrap();
    assert_eq!("anonymous", msg.user);
}

#[test]
fn name_default_trait_present() {
    #[derive(FromMessage)]
    struct Source {
        #[irc(source, default)]
        nick: String,
    }

    let input = ":nick!user@example.com PRIVMSG #channel :hi";
    let msg: Source = ircv3_parse::from_str(input).unwrap();
    assert_eq!("nick", msg.nick);
}

#[test]
fn user_default_trait_present() {
    #[derive(FromMessage)]
    struct Source {
        #[irc(source = "user", default)]
        user: String,
    }

    let input = ":nick!user@example.com PRIVMSG #channel :hi";
    let msg: Source = ircv3_parse::from_str(input).unwrap();
    assert_eq!("user", msg.user);
}

#[test]
fn name_optional_with_default_no_component() {
    #[derive(FromMessage)]
    struct Source {
        #[irc(source, default)]
        user: Option<String>,
    }

    let input = "PRIVMSG #channel :hi";
    let msg: Source = ircv3_parse::from_str(input).unwrap();
    assert_eq!(None, msg.user);
}

#[test]
fn user_optional_with_default_no_component() {
    #[derive(FromMessage)]
    struct Source {
        #[irc(source = "user", default)]
        user: Option<String>,
    }

    let input = "PRIVMSG #channel :hi";
    let msg: Source = ircv3_parse::from_str(input).unwrap();
    assert_eq!(None, msg.user);
}

#[test]
fn name_optional_with_default_present() {
    #[derive(FromMessage)]
    struct Source {
        #[irc(source, default)]
        nick: Option<String>,
    }

    let input = ":nick!user@example.com PRIVMSG #channel :hi";
    let msg: Source = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Some("nick".to_string()), msg.nick);
}

#[test]
fn user_optional_with_default_present() {
    #[derive(FromMessage)]
    struct Source {
        #[irc(source = "user", default)]
        user: Option<String>,
    }

    let input = ":nick!user@example.com PRIVMSG #channel :hi";
    let msg: Source = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Some("user".to_string()), msg.user);
}

#[test]
fn unnamed_name_default_trait() {
    #[derive(FromMessage)]
    struct Source(#[irc(source, default)] String);

    let input = "PRIVMSG #channel :hi";
    let msg: Source = ircv3_parse::from_str(input).unwrap();
    assert_eq!("", msg.0);
}

#[test]
fn unnamed_user_default_trait() {
    #[derive(FromMessage)]
    struct Source(#[irc(source = "user", default)] String);

    let input = ":nick PRIVMSG #channel :hi";
    let msg: Source = ircv3_parse::from_str(input).unwrap();
    assert_eq!("", msg.0);
}
