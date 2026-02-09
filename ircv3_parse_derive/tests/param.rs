use ircv3_parse_derive::FromMessage;

#[test]
fn single() {
    #[derive(FromMessage)]
    struct Param<'a> {
        #[irc(param = 0)]
        channel: &'a str,
    }

    let input = "PRIVMSG #channel :hello";
    let msg: Param = ircv3_parse::from_str(input).unwrap();
    assert_eq!("#channel", msg.channel);
}

#[test]
fn multiple() {
    #[derive(FromMessage)]
    struct Params<'a> {
        #[irc(param = 0)]
        first: &'a str,
        #[irc(param = 1)]
        second: &'a str,
        #[irc(param = 2)]
        third: &'a str,
    }

    let input = "PRIVMSG param1 param2 param3";
    let msg: Params = ircv3_parse::from_str(input).unwrap();
    assert_eq!("param1", msg.first);
    assert_eq!("param2", msg.second);
    assert_eq!("param3", msg.third);
}

#[test]
fn default() {
    #[derive(FromMessage)]
    struct Param {
        #[irc(param)]
        param: String,
    }

    let input = "PRIVMSG #channel param2 :hi";
    let msg: Param = ircv3_parse::from_str(input).unwrap();
    assert_eq!("#channel", msg.param);
}

#[test]
fn optional() {
    #[derive(FromMessage)]
    struct Param {
        #[irc(param = 1)]
        param: Option<String>,
    }

    let input = "PRIVMSG param1 param2";
    let msg: Param = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Some("param2".to_string()), msg.param);
}

#[test]
fn optional_missing() {
    #[derive(FromMessage)]
    struct Param {
        #[irc(param = 1)]
        param: Option<String>,
    }

    let input = "PRIVMSG param";
    let msg: Param = ircv3_parse::from_str(input).unwrap();
    assert_eq!(None, msg.param);
}

#[test]
fn with_function() {
    fn parse_num(s: Option<&str>) -> u32 {
        s.and_then(|x| x.parse().ok()).unwrap_or(0)
    }

    #[derive(FromMessage)]
    struct Param {
        #[irc(param = 0, with = "parse_num")]
        count: u32,
    }

    let input = "PRIVMSG 42";
    let msg: Param = ircv3_parse::from_str(input).unwrap();
    assert_eq!(42, msg.count);
}

#[test]
fn with_function_missing() {
    fn parse_num(s: Option<&str>) -> u32 {
        s.and_then(|x| x.parse().ok()).unwrap_or(0)
    }

    #[derive(FromMessage)]
    struct Param {
        #[irc(param = 0, with = "parse_num")]
        count: u32,
    }

    let input = "PRIVMSG";
    let msg: Param = ircv3_parse::from_str(input).unwrap();
    assert_eq!(0, msg.count);
}

#[test]
fn unnamed_single() {
    #[derive(FromMessage)]
    struct Param<'a>(#[irc(param = 0)] &'a str);

    let input = "PRIVMSG #channel :hello";
    let msg: Param = ircv3_parse::from_str(input).unwrap();
    assert_eq!("#channel", msg.0);
}

#[test]
fn unnamed_multiple() {
    #[derive(FromMessage)]
    struct Params<'a>(
        #[irc(param = 0)] &'a str,
        #[irc(param = 1)] &'a str,
        #[irc(param = 2)] &'a str,
    );

    let input = "PRIVMSG param1 param2 param3";
    let msg: Params = ircv3_parse::from_str(input).unwrap();
    assert_eq!("param1", msg.0);
    assert_eq!("param2", msg.1);
    assert_eq!("param3", msg.2);
}

#[test]
fn unnamed_default() {
    #[derive(FromMessage)]
    struct Param(#[irc(param)] String);

    let input = "PRIVMSG #channel param2 :hi";
    let msg: Param = ircv3_parse::from_str(input).unwrap();
    assert_eq!("#channel", msg.0);
}

#[test]
fn unnamed_optional() {
    #[derive(FromMessage)]
    struct Param(#[irc(param = 1)] Option<String>);

    let input = "PRIVMSG param1 param2";
    let msg: Param = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Some("param2".to_string()), msg.0);
}

#[test]
fn unnamed_optional_missing() {
    #[derive(FromMessage)]
    struct Param(#[irc(param = 1)] Option<String>);

    let input = "PRIVMSG param";
    let msg: Param = ircv3_parse::from_str(input).unwrap();
    assert_eq!(None, msg.0);
}

#[test]
fn unnamed_with_function() {
    fn parse_num(s: Option<&str>) -> u32 {
        s.and_then(|x| x.parse().ok()).unwrap_or(0)
    }

    #[derive(FromMessage)]
    struct Param(#[irc(param = 0, with = "parse_num")] u32);

    let input = "PRIVMSG 42";
    let msg: Param = ircv3_parse::from_str(input).unwrap();
    assert_eq!(42, msg.0);
}

#[test]
fn unnamed_with_function_missing() {
    fn parse_num(s: Option<&str>) -> u32 {
        s.and_then(|x| x.parse().ok()).unwrap_or(0)
    }

    #[derive(FromMessage)]
    struct Param(#[irc(param = 0, with = "parse_num")] u32);

    let input = "PRIVMSG";
    let msg: Param = ircv3_parse::from_str(input).unwrap();
    assert_eq!(0, msg.0);
}

#[test]
fn nested_param() {
    #[derive(FromMessage, Debug, PartialEq)]
    struct Channel<'a>(#[irc(param = 0)] &'a str);

    #[derive(FromMessage)]
    struct Message<'a> {
        ch: Channel<'a>,
    }

    let input = "PRIVMSG #channel :hello";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Channel("#channel"), msg.ch);
}

#[test]
fn nested_param_optional() {
    #[derive(FromMessage, Debug, PartialEq)]
    struct Channel<'a>(#[irc(param = 0)] &'a str);

    #[derive(FromMessage)]
    struct Message<'a> {
        ch: Option<Channel<'a>>,
    }

    let input = "PRIVMSG #channel :hello";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Some(Channel("#channel")), msg.ch);
}

#[test]
fn nested_param_with_function() {
    fn parse_num(s: Option<&str>) -> u32 {
        s.and_then(|x| x.parse().ok()).unwrap_or(0)
    }

    #[derive(FromMessage, Debug, PartialEq)]
    struct Count(#[irc(param = 0, with = "parse_num")] u32);

    #[derive(FromMessage)]
    struct Message {
        count: Count,
    }

    let input = "PRIVMSG 42";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Count(42), msg.count);
}

#[test]
fn nested_outer_attribute_ignored() {
    fn parse_num(s: Option<&str>) -> u32 {
        s.and_then(|x| x.parse().ok()).unwrap_or(0)
    }

    #[derive(FromMessage, Debug, PartialEq)]
    struct Count(#[irc(param = 0, with = "parse_num")] u32);

    #[derive(FromMessage)]
    struct Message {
        #[irc(param = 2)]
        count: Count,
    }

    let input = "PRIVMSG 42";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Count(42), msg.count);
}
