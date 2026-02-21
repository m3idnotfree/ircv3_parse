use ircv3_parse_derive::FromMessage;

#[test]
fn value() {
    #[derive(FromMessage)]
    struct Tag {
        #[irc(tag = "msgid")]
        msg_id: String,
    }

    let input = "@msgid=123 PRIVMSG #channel :hello";
    let msg: Tag = ircv3_parse::from_str(input).unwrap();
    assert_eq!("123", msg.msg_id);
}

#[test]
fn value_optional_some() {
    #[derive(FromMessage)]
    struct Tag {
        #[irc(tag = "msgid")]
        msg_id: Option<String>,
    }

    let input = "@msgid=123 PRIVMSG #channel :hello";
    let msg: Tag = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Some("123".to_string()), msg.msg_id);
}

#[test]
fn value_optional_none() {
    #[derive(FromMessage)]
    struct Tag {
        #[irc(tag = "msgid")]
        msg_id: Option<String>,
    }

    let input = "@other=456 PRIVMSG #channel :hello";
    let msg: Tag = ircv3_parse::from_str(input).unwrap();
    assert_eq!(None, msg.msg_id);
}

#[test]
fn tag_flag() {
    #[derive(FromMessage)]
    struct Tag {
        #[irc(tag_flag = "subcriber")]
        subcriber: bool,
    }

    let input = "@subcriber PRIVMSG #channel :hello";
    let msg: Tag = ircv3_parse::from_str(input).unwrap();
    assert!(msg.subcriber);
}

#[test]
fn tag_flag_not_found() {
    #[derive(FromMessage)]
    struct Tag {
        #[irc(tag_flag = "m-1")]
        flag: bool,
    }

    let input = "@msgid=123 PRIVMSG #channel :hello";
    let msg: Tag = ircv3_parse::from_str(input).unwrap();
    assert!(!msg.flag);
}

#[test]
fn empty_attribute_uses_field_name() {
    #[derive(FromMessage)]
    struct Tags {
        #[irc(tag)]
        msgid: String,
        #[irc(tag_flag)]
        field: bool,
        #[irc(tag_flag)]
        field2: bool,
    }

    let input = "@msgid=1;field2 PRIVMSG #channel :hi";
    let msg: Tags = ircv3_parse::from_str(input).unwrap();

    assert_eq!("1", msg.msgid);
    assert!(!msg.field);
    assert!(msg.field2);
}

#[test]
fn unnamed_value() {
    #[derive(FromMessage)]
    struct Tag(#[irc(tag = "msgid")] String);

    let input = "@msgid=123 PRIVMSG #channel :hello";
    let msg: Tag = ircv3_parse::from_str(input).unwrap();
    assert_eq!("123", msg.0);
}

#[test]
fn unnamed_value_optional_some() {
    #[derive(FromMessage)]
    struct Tag(#[irc(tag = "msgid")] Option<String>);

    let input = "@msgid=123 PRIVMSG #channel :hello";
    let msg: Tag = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Some("123".to_string()), msg.0);
}

#[test]
fn unnamed_value_optional_none() {
    #[derive(FromMessage)]
    struct Tag(#[irc(tag = "msgid")] Option<String>);

    let input = "@other=456 PRIVMSG #channel :hello";
    let msg: Tag = ircv3_parse::from_str(input).unwrap();
    assert_eq!(None, msg.0);
}

#[test]
fn unnamed_tag_flag() {
    #[derive(FromMessage)]
    struct Tag(#[irc(tag_flag = "m-1")] bool);

    let input = "@m-1 PRIVMSG #channel :hello";
    let msg: Tag = ircv3_parse::from_str(input).unwrap();
    assert!(msg.0);
}

#[test]
fn unnamed_tag_flag_not_found() {
    #[derive(FromMessage)]
    struct Tag(#[irc(tag_flag = "m-1")] bool);

    let input = "@msgid=123 PRIVMSG #channel :hello";
    let msg: Tag = ircv3_parse::from_str(input).unwrap();
    assert!(!msg.0);
}

#[test]
fn unnamed_multiple_tags() {
    #[derive(FromMessage)]
    struct Tag(
        #[irc(tag = "msgid")] Option<String>,
        #[irc(tag_flag = "m-1")] bool,
        #[irc(tag = "color")] Option<String>,
    );

    let input = "@msgid=123;m-1;color=#FF0000 PRIVMSG #channel :hello";
    let msg: Tag = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Some("123".to_string()), msg.0);
    assert!(msg.1);
    assert_eq!(Some("#FF0000".to_string()), msg.2);
}

#[test]
fn nested_value() {
    #[derive(FromMessage, Debug, PartialEq)]
    struct MsgId(#[irc(tag = "msgid")] String);

    #[derive(FromMessage)]
    struct Message {
        msg_id: MsgId,
    }

    let input = "@msgid=123 PRIVMSG #channel :hello";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(MsgId("123".to_string()), msg.msg_id);
}

#[test]
fn nested_optional() {
    #[derive(FromMessage, Debug, PartialEq)]
    struct MsgId(#[irc(tag = "msgid")] String);

    #[derive(FromMessage)]
    struct Message {
        msg_id: Option<MsgId>,
    }

    let input = "@msgid=123 PRIVMSG #channel :hello";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Some(MsgId("123".to_string())), msg.msg_id);
}

#[test]
fn nested_optional_none() {
    #[derive(FromMessage, Debug, PartialEq)]
    struct MsgId(#[irc(tag = "msgid")] String);

    #[derive(FromMessage)]
    struct Message {
        msg_id: Option<MsgId>,
    }

    let input = "PRIVMSG #channel :hello";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(None, msg.msg_id);
}

#[test]
fn nested_outer_attribute_ignored() {
    #[derive(FromMessage, Debug, PartialEq)]
    struct MsgId(#[irc(tag = "msgid")] String);

    #[derive(FromMessage)]
    struct Message {
        #[irc(tag = "ignore")]
        msg_id: MsgId,
    }

    let input = "@msgid=123 PRIVMSG #channel :hello";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(MsgId("123".to_string()), msg.msg_id);
}

#[test]
fn unit_struct() {
    #[derive(FromMessage, Debug, PartialEq)]
    #[irc(tag = "msgid")]
    struct MsgId;

    let input = "@msgid=123 PRIVMSG #channel :hello";
    let msg: MsgId = ircv3_parse::from_str(input).unwrap();
    assert_eq!(MsgId, msg);
}

#[test]
fn default_trait_no_component() {
    #[derive(FromMessage)]
    struct Tag {
        #[irc(tag = "msgid", default)]
        msg_id: String,
    }

    let input = "PRIVMSG #channel :hello";
    let msg: Tag = ircv3_parse::from_str(input).unwrap();
    assert_eq!("", msg.msg_id);
}

#[test]
fn default_fn_no_component() {
    fn default_id() -> String {
        "0".to_string()
    }

    #[derive(FromMessage)]
    struct Tag {
        #[irc(tag = "msgid", default = "default_id")]
        msg_id: String,
    }

    let input = "PRIVMSG #channel :hello";
    let msg: Tag = ircv3_parse::from_str(input).unwrap();
    assert_eq!("0", msg.msg_id);
}

#[test]
fn tag_flag_default_trait_no_component() {
    #[derive(FromMessage)]
    struct Tag {
        #[irc(tag_flag = "msgid", default)]
        msg_id: bool,
    }

    let input = "PRIVMSG #channel :hello";
    let msg: Tag = ircv3_parse::from_str(input).unwrap();
    assert!(!msg.msg_id);
}

#[test]
fn default_trait_present() {
    #[derive(FromMessage)]
    struct Tag {
        #[irc(tag = "msgid", default)]
        msg_id: String,
    }

    let input = "@msgid=123 PRIVMSG #channel :hello";
    let msg: Tag = ircv3_parse::from_str(input).unwrap();
    assert_eq!("123", msg.msg_id);
}

#[test]
fn default_fn_present() {
    fn default_id() -> String {
        "0".to_string()
    }

    #[derive(FromMessage)]
    struct Tag {
        #[irc(tag = "msgid", default = "default_id")]
        msg_id: String,
    }

    let input = "@msgid=123 PRIVMSG #channel :hello";
    let msg: Tag = ircv3_parse::from_str(input).unwrap();
    assert_eq!("123", msg.msg_id);
}

#[test]
fn tag_flag_default_trait_present() {
    #[derive(FromMessage)]
    struct Tag {
        #[irc(tag_flag = "msgid", default)]
        msg_id: bool,
    }

    let input = "@msgid PRIVMSG #channel :hello";
    let msg: Tag = ircv3_parse::from_str(input).unwrap();
    assert!(msg.msg_id);
}

#[test]
fn optional_with_default() {
    #[derive(FromMessage)]
    struct Tag {
        #[irc(tag = "msgid", default)]
        msg_id: Option<String>,
    }

    let input = "@f=456 PRIVMSG #channel :hello";
    let msg: Tag = ircv3_parse::from_str(input).unwrap();
    assert_eq!(None, msg.msg_id);
}

#[test]
fn optional_with_default_present() {
    #[derive(FromMessage)]
    struct Tag {
        #[irc(tag = "msgid", default)]
        msg_id: Option<String>,
    }

    let input = "@msgid=123 PRIVMSG #channel :hello";
    let msg: Tag = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Some("123".to_string()), msg.msg_id);
}

#[test]
fn optional_with_default_no_component() {
    #[derive(FromMessage)]
    struct Tag {
        #[irc(tag = "msgid", default)]
        msg_id: Option<String>,
    }

    let input = "PRIVMSG #channel :hello";
    let msg: Tag = ircv3_parse::from_str(input).unwrap();
    assert_eq!(None, msg.msg_id);
}

#[test]
fn unnamed_default_trait() {
    #[derive(FromMessage)]
    struct Tag(#[irc(tag = "msgid", default)] String);

    let input = "PRIVMSG #channel :hello";
    let msg: Tag = ircv3_parse::from_str(input).unwrap();
    assert_eq!("", msg.0);
}

#[test]
fn unnamed_default_fn() {
    fn default_id() -> String {
        "0".to_string()
    }

    #[derive(FromMessage)]
    struct Tag(#[irc(tag = "msgid", default = "default_id")] String);

    let input = "PRIVMSG #channel :hello";
    let msg: Tag = ircv3_parse::from_str(input).unwrap();
    assert_eq!("0", msg.0);
}
