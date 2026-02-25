use ircv3_parse_derive::FromMessage;

#[test]
fn basic() {
    #[derive(FromMessage, Debug, PartialEq)]
    #[irc(tag = "color")]
    enum Color {
        Red,
        Green,
        Blue,
        DarkGreen,
    }

    let msg: Color = ircv3_parse::from_str("@color=red PRIVMSG").unwrap();
    assert_eq!(Color::Red, msg);

    let msg: Color = ircv3_parse::from_str("@color=green PRIVMSG").unwrap();
    assert_eq!(Color::Green, msg);

    let msg: Color = ircv3_parse::from_str("@color=blue PRIVMSG").unwrap();
    assert_eq!(Color::Blue, msg);

    let msg: Color = ircv3_parse::from_str("@color=darkgreen PRIVMSG").unwrap();
    assert_eq!(Color::DarkGreen, msg);

    let err = ircv3_parse::from_str::<Color>("@color=dark-green PRIVMSG").unwrap_err();
    assert!(err.is_not_found_tag());

    let err = ircv3_parse::from_str::<Color>("@color=purple PRIVMSG").unwrap_err();
    assert!(err.is_not_found_tag());

    let err = ircv3_parse::from_str::<Color>("PRIVMSG").unwrap_err();
    assert!(err.is_tags_component_not_found());
}

#[test]
fn value() {
    #[derive(FromMessage, Debug, PartialEq)]
    #[irc(tag = "color")]
    enum Color {
        #[irc(value = "light-red")]
        LightRed,
        #[irc(value = "dark-green")]
        DarkGreen,
    }

    let msg: Color = ircv3_parse::from_str("@color=light-red PRIVMSG").unwrap();
    assert_eq!(Color::LightRed, msg);

    let msg: Color = ircv3_parse::from_str("@color=dark-green PRIVMSG").unwrap();
    assert_eq!(Color::DarkGreen, msg);

    let err = ircv3_parse::from_str::<Color>("@color=red PRIVMSG").unwrap_err();
    assert!(err.is_not_found_tag());

    let err = ircv3_parse::from_str::<Color>("PRIVMSG").unwrap_err();
    assert!(err.is_tags_component_not_found());
}

#[test]
fn rename_kebab_case() {
    #[derive(FromMessage, Debug, PartialEq)]
    #[irc(tag = "color", rename = "kebab-case")]
    enum Color {
        LightRed,
        DarkGreen,
    }

    let msg: Color = ircv3_parse::from_str("@color=light-red PRIVMSG").unwrap();
    assert_eq!(Color::LightRed, msg);

    let msg: Color = ircv3_parse::from_str("@color=dark-green PRIVMSG").unwrap();
    assert_eq!(Color::DarkGreen, msg);

    let err = ircv3_parse::from_str::<Color>("PRIVMSG").unwrap_err();
    assert!(err.is_tags_component_not_found());
}

#[test]
fn value_overrides_rename() {
    #[derive(FromMessage, Debug, PartialEq)]
    #[irc(tag = "color", rename = "kebab-case")]
    enum Color {
        #[irc(value = "crimson")]
        Red,
        DarkGreen,
    }

    let msg: Color = ircv3_parse::from_str("@color=crimson PRIVMSG").unwrap();
    assert_eq!(Color::Red, msg);

    let msg: Color = ircv3_parse::from_str("@color=dark-green PRIVMSG").unwrap();
    assert_eq!(Color::DarkGreen, msg);
}

#[test]
fn default() {
    #[derive(FromMessage, Debug, PartialEq)]
    #[irc(tag = "color", default = "Black")]
    enum Color {
        Red,
        Green,
        Black,
    }

    let msg: Color = ircv3_parse::from_str("@color=red PRIVMSG").unwrap();
    assert_eq!(Color::Red, msg);

    let msg: Color = ircv3_parse::from_str("@color=purple PRIVMSG").unwrap();
    assert_eq!(Color::Black, msg);

    let msg: Color = ircv3_parse::from_str("PRIVMSG").unwrap();
    assert_eq!(Color::Black, msg);
}

#[test]
fn named_fields() {
    #[derive(FromMessage, Debug, PartialEq)]
    #[irc(tag = "action")]
    enum Action {
        Join {
            #[irc(param)]
            channel: String,
        },
        Quit,
    }

    let msg: Action = ircv3_parse::from_str("@action=join PRIVMSG #channel :hi").unwrap();
    assert_eq!(
        Action::Join {
            channel: "#channel".to_string()
        },
        msg
    );

    let msg: Action = ircv3_parse::from_str("@action=quit PRIVMSG #channel :hi").unwrap();
    assert_eq!(Action::Quit, msg);

    let err = ircv3_parse::from_str::<Action>("@action=kick PRIVMSG").unwrap_err();
    assert!(err.is_not_found_tag());

    let err = ircv3_parse::from_str::<Action>("PRIVMSG").unwrap_err();
    assert!(err.is_tags_component_not_found());
}

#[test]
fn multiple_values() {
    #[derive(FromMessage, Debug, PartialEq)]
    #[irc(tag = "color")]
    enum Color {
        #[irc(value = "red")]
        #[irc(value = "crimson")]
        Red,
        #[irc(value = "green")]
        #[irc(value = "lime")]
        Green,
    }

    let msg: Color = ircv3_parse::from_str("@color=red PRIVMSG").unwrap();
    assert_eq!(Color::Red, msg);

    let msg: Color = ircv3_parse::from_str("@color=crimson PRIVMSG").unwrap();
    assert_eq!(Color::Red, msg);

    let msg: Color = ircv3_parse::from_str("@color=green PRIVMSG").unwrap();
    assert_eq!(Color::Green, msg);

    let msg: Color = ircv3_parse::from_str("@color=lime PRIVMSG").unwrap();
    assert_eq!(Color::Green, msg);

    let err = ircv3_parse::from_str::<Color>("@color=blue PRIVMSG").unwrap_err();
    assert!(err.is_not_found_tag());
}

#[test]
fn single_unnamed_field() {
    #[derive(FromMessage, Debug, PartialEq)]
    #[irc(tag = "level")]
    enum Level {
        High(String),
        Low(String),
    }

    let msg: Level = ircv3_parse::from_str("@level=high PRIVMSG").unwrap();
    assert_eq!(Level::High("high".to_string()), msg);

    let msg: Level = ircv3_parse::from_str("@level=low PRIVMSG").unwrap();
    assert_eq!(Level::Low("low".to_string()), msg);

    let err = ircv3_parse::from_str::<Level>("@level=medium PRIVMSG").unwrap_err();
    assert!(err.is_not_found_tag());

    let err = ircv3_parse::from_str::<Level>("PRIVMSG").unwrap_err();
    assert!(err.is_tags_component_not_found());
}
