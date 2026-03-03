use ircv3_parse_derive::{FromMessage, ToMessage};

#[test]
fn basic() {
    #[derive(Debug, PartialEq, FromMessage, ToMessage)]
    #[irc(tag = "color")]
    enum Color {
        Red,
        Green,
        Blue,
        DarkGreen,
    }

    let msg: Color = ircv3_parse::from_str("@color=red PRIVMSG").unwrap();
    assert_eq!(Color::Red, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("@color=red ", output);

    let msg: Color = ircv3_parse::from_str("@color=green PRIVMSG").unwrap();
    assert_eq!(Color::Green, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("@color=green ", output);

    let msg: Color = ircv3_parse::from_str("@color=blue PRIVMSG").unwrap();
    assert_eq!(Color::Blue, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("@color=blue ", output);

    let msg: Color = ircv3_parse::from_str("@color=dark-green PRIVMSG").unwrap();
    assert_eq!(Color::DarkGreen, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("@color=dark-green ", output);

    let err = ircv3_parse::from_str::<Color>("@color=darkgreen PRIVMSG").unwrap_err();
    assert!(err.is_not_found_tag());

    let err = ircv3_parse::from_str::<Color>("@color=purple PRIVMSG").unwrap_err();
    assert!(err.is_not_found_tag());

    let err = ircv3_parse::from_str::<Color>("PRIVMSG").unwrap_err();
    assert!(err.is_tags_component_not_found());
}

#[test]
fn value() {
    #[derive(Debug, PartialEq, FromMessage, ToMessage)]
    #[irc(tag = "color")]
    enum Color {
        #[irc(value = "light-red")]
        LightRed,
        #[irc(value = "dark-green")]
        DarkGreen,
    }

    let msg: Color = ircv3_parse::from_str("@color=light-red PRIVMSG").unwrap();
    assert_eq!(Color::LightRed, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("@color=light-red ", output);

    let msg: Color = ircv3_parse::from_str("@color=dark-green PRIVMSG").unwrap();
    assert_eq!(Color::DarkGreen, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("@color=dark-green ", output);

    let err = ircv3_parse::from_str::<Color>("@color=red PRIVMSG").unwrap_err();
    assert!(err.is_not_found_tag());

    let err = ircv3_parse::from_str::<Color>("PRIVMSG").unwrap_err();
    assert!(err.is_tags_component_not_found());
}

#[test]
fn rename_kebab_case() {
    #[derive(Debug, PartialEq, FromMessage, ToMessage)]
    #[irc(tag = "color", rename = "kebab-case")]
    enum Color {
        LightRed,
        DarkGreen,
    }

    let msg: Color = ircv3_parse::from_str("@color=light-red PRIVMSG").unwrap();
    assert_eq!(Color::LightRed, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("@color=light-red ", output);

    let msg: Color = ircv3_parse::from_str("@color=dark-green PRIVMSG").unwrap();
    assert_eq!(Color::DarkGreen, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("@color=dark-green ", output);

    let err = ircv3_parse::from_str::<Color>("PRIVMSG").unwrap_err();
    assert!(err.is_tags_component_not_found());
}

#[test]
fn value_overrides_rename() {
    #[derive(Debug, PartialEq, FromMessage, ToMessage)]
    #[irc(tag = "color", rename = "kebab-case")]
    enum Color {
        #[irc(value = "crimson")]
        Red,
        DarkGreen,
    }

    let msg: Color = ircv3_parse::from_str("@color=crimson PRIVMSG").unwrap();
    assert_eq!(Color::Red, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("@color=crimson ", output);

    let msg: Color = ircv3_parse::from_str("@color=dark-green PRIVMSG").unwrap();
    assert_eq!(Color::DarkGreen, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("@color=dark-green ", output);
}

#[test]
fn default() {
    #[derive(Debug, PartialEq, FromMessage, ToMessage)]
    #[irc(tag = "color", default = "Black")]
    enum Color {
        Red,
        Green,
        Black,
    }

    let msg: Color = ircv3_parse::from_str("@color=red PRIVMSG").unwrap();
    assert_eq!(Color::Red, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("@color=red ", output);

    let msg: Color = ircv3_parse::from_str("@color=purple PRIVMSG").unwrap();
    assert_eq!(Color::Black, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("@color=black ", output);

    let msg: Color = ircv3_parse::from_str("PRIVMSG").unwrap();
    assert_eq!(Color::Black, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("@color=black ", output);
}

#[test]
fn named_fields() {
    #[derive(Debug, PartialEq, FromMessage, ToMessage)]
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
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("@action=join  #channel", output);

    let msg: Action = ircv3_parse::from_str("@action=quit PRIVMSG #channel :hi").unwrap();
    assert_eq!(Action::Quit, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("@action=quit ", output);

    let err = ircv3_parse::from_str::<Action>("@action=kick PRIVMSG").unwrap_err();
    assert!(err.is_not_found_tag());

    let err = ircv3_parse::from_str::<Action>("PRIVMSG").unwrap_err();
    assert!(err.is_tags_component_not_found());
}

#[test]
fn multiple_values() {
    #[derive(Debug, PartialEq, FromMessage, ToMessage)]
    #[irc(tag = "color")]
    enum Color {
        #[irc(value = "red", pick)]
        #[irc(value = "crimson")]
        Red,
        #[irc(value = "green", pick)]
        #[irc(value = "lime")]
        Green,
    }

    let msg: Color = ircv3_parse::from_str("@color=red PRIVMSG").unwrap();
    assert_eq!(Color::Red, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("@color=red ", output);

    let msg: Color = ircv3_parse::from_str("@color=crimson PRIVMSG").unwrap();
    assert_eq!(Color::Red, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("@color=red ", output);

    let msg: Color = ircv3_parse::from_str("@color=green PRIVMSG").unwrap();
    assert_eq!(Color::Green, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("@color=green ", output);

    let msg: Color = ircv3_parse::from_str("@color=lime PRIVMSG").unwrap();
    assert_eq!(Color::Green, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("@color=green ", output);

    let err = ircv3_parse::from_str::<Color>("@color=blue PRIVMSG").unwrap_err();
    assert!(err.is_not_found_tag());
}

#[test]
fn single_unnamed_field() {
    #[derive(Debug, PartialEq, FromMessage, ToMessage)]
    #[irc(tag = "level")]
    enum Level {
        High(String),
        Low(String),
    }

    let msg: Level = ircv3_parse::from_str("@level=high PRIVMSG").unwrap();
    assert_eq!(Level::High("high".to_string()), msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("@level=high ", output);

    let msg: Level = ircv3_parse::from_str("@level=low PRIVMSG").unwrap();
    assert_eq!(Level::Low("low".to_string()), msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("@level=low ", output);

    let err = ircv3_parse::from_str::<Level>("@level=medium PRIVMSG").unwrap_err();
    assert!(err.is_not_found_tag());

    let err = ircv3_parse::from_str::<Level>("PRIVMSG").unwrap_err();
    assert!(err.is_tags_component_not_found());
}
