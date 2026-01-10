#![allow(unused)]
#[allow(unused_imports)]
use ircv3_parse::message::ser::ToMessage as _;
use ircv3_parse_derive::ToMessage;

#[test]
fn struct_level_command() {
    #[derive(ToMessage)]
    #[irc(command = "PRIVMSG")]
    struct PrivMsg {}

    let priv_msg = PrivMsg {};

    let size = priv_msg.serialized_size();
    let msg = priv_msg.to_bytes().unwrap();
    assert_eq!("PRIVMSG", msg);
    assert_eq!(7, size)
}

#[test]
fn struct_level_command_crlf() {
    #[derive(ToMessage)]
    #[irc(command = "PRIVMSG", crlf)]
    struct PrivMsg {};

    let priv_msg = PrivMsg {};

    let size = priv_msg.serialized_size();
    let msg = priv_msg.to_bytes().unwrap();
    assert_eq!("PRIVMSG\r\n", msg);
    assert_eq!(9, size)
}

#[test]
fn struct_level_command_with_field_command() {
    #[derive(ToMessage)]
    #[irc(command = "PRIVMSG")]
    struct PrivMsg {
        #[irc(command = "NOTICE")]
        command: String,
    }

    let priv_msg = PrivMsg {
        command: "NOTICE".to_string(),
    };

    let size = priv_msg.serialized_size();
    let msg = priv_msg.to_bytes().unwrap();
    assert_eq!("NOTICE", msg);
    assert_eq!(6, size);
}

#[test]
fn field_command() {
    #[derive(ToMessage)]
    struct PrivMsg {
        #[irc(command = "NOTICE")]
        command: String,
    }

    let priv_msg = PrivMsg {
        command: "NOTICE".to_string(),
    };

    let size = priv_msg.serialized_size();
    let msg = priv_msg.to_bytes().unwrap();
    assert_eq!("NOTICE", msg);
    assert_eq!(6, size)
}

#[test]
fn tag() {
    #[derive(ToMessage)]
    struct Tag {
        #[irc(tag)]
        field: String,
    }

    let priv_msg = Tag {
        field: "value".to_string(),
    };

    let size = priv_msg.serialized_size();
    let msg = priv_msg.to_bytes().unwrap();
    assert_eq!("@field=value", msg);
    assert_eq!(12, size)
}

#[test]
fn tag_str() {
    #[derive(ToMessage)]
    struct Tag<'a> {
        #[irc(tag = "key")]
        field: &'a str,
    }

    let priv_msg = Tag { field: "value" };

    let size = priv_msg.serialized_size();
    let msg = priv_msg.to_bytes().unwrap();
    assert_eq!("@key=value", msg);
    assert_eq!(10, size)
}

#[test]
fn tag_opt_none() {
    #[derive(ToMessage)]
    struct Tag<'a> {
        #[irc(tag)]
        field: Option<&'a str>,
    }

    let priv_msg = Tag { field: None };

    let size = priv_msg.serialized_size();
    let msg = priv_msg.to_bytes().unwrap();
    assert_eq!("@field=", msg);
    assert_eq!(7, size)
}

#[test]
fn tag_opt_some() {
    #[derive(ToMessage)]
    struct Tag {
        #[irc(tag)]
        field: Option<String>,
    }

    let priv_msg = Tag {
        field: Some("value".to_string()),
    };

    let size = priv_msg.serialized_size();
    let msg = priv_msg.to_bytes().unwrap();
    assert_eq!("@field=value", msg);
    assert_eq!(12, size)
}

#[test]
fn tag_opt_ser() {
    #[derive(ToMessage)]
    struct Field {
        #[irc(tag)]
        field: Option<String>,
    }

    #[derive(ToMessage)]
    struct Tag {
        #[irc(tag)]
        tag: Field,
    }

    let field = Field {
        field: Some("value".to_string()),
    };

    let tag = Tag { tag: field };

    let size = tag.serialized_size();
    let msg = tag.to_bytes().unwrap();
    assert_eq!("@field=value", msg);
    assert_eq!(12, size)
}

#[test]
fn tag_multi() {
    #[derive(ToMessage)]
    struct Tag {
        #[irc(tag)]
        field: String,
        #[irc(tag)]
        field2: String,
    }

    let priv_msg = Tag {
        field: "value".to_string(),
        field2: "value2".to_string(),
    };

    let size = priv_msg.serialized_size();
    let msg = priv_msg.to_bytes().unwrap();
    assert_eq!("@field=value;field2=value2", msg);
    assert_eq!(26, size)
}

#[test]
fn source() {
    #[derive(ToMessage)]
    struct Source {
        #[irc(source)]
        name: String,
    }

    let priv_msg = Source {
        name: "name".to_string(),
    };

    let size = priv_msg.serialized_size();
    let msg = priv_msg.to_bytes().unwrap();
    assert_eq!(":name", msg);
    assert_eq!(5, size)
}

#[test]
fn source_with_user() {
    #[derive(ToMessage)]
    struct Source {
        #[irc(source)]
        name: String,
        #[irc(source = "user")]
        user: String,
    }

    let priv_msg = Source {
        name: "name".to_string(),
        user: "user".to_string(),
    };

    let size = priv_msg.serialized_size();
    let msg = priv_msg.to_bytes().unwrap();
    assert_eq!(":name!user", msg);
    assert_eq!(10, size)
}

#[test]
fn source_with_host() {
    #[derive(ToMessage)]
    struct Source {
        #[irc(source)]
        name: String,
        #[irc(source = "host")]
        host: String,
    }

    let priv_msg = Source {
        name: "name".to_string(),
        host: "example.com".to_string(),
    };

    let size = priv_msg.serialized_size();
    let msg = priv_msg.to_bytes().unwrap();
    assert_eq!(":name@example.com", msg);
    assert_eq!(17, size)
}

#[test]
fn source_with_all() {
    #[derive(ToMessage)]
    struct Source {
        #[irc(source)]
        name: String,
        #[irc(source = "user")]
        user: String,
        #[irc(source = "host")]
        host: String,
    }

    let priv_msg = Source {
        name: "name".to_string(),
        user: "user".to_string(),
        host: "example.com".to_string(),
    };

    let size = priv_msg.serialized_size();
    let msg = priv_msg.to_bytes().unwrap();
    assert_eq!(":name!user@example.com", msg);
    assert_eq!(22, size)
}

#[test]
fn source_with_all2() {
    #[derive(ToMessage)]
    struct Source {
        #[irc(source = "name")]
        name: String,
        #[irc(source = "user")]
        user: String,
        #[irc(source = "host")]
        host: String,
    }

    let priv_msg = Source {
        name: "name".to_string(),
        user: "user".to_string(),
        host: "example.com".to_string(),
    };

    let size = priv_msg.serialized_size();
    let msg = priv_msg.to_bytes().unwrap();
    assert_eq!(":name!user@example.com", msg);
    assert_eq!(22, size)
}

#[test]
fn param() {
    #[derive(ToMessage)]
    struct Params {
        #[irc(param)]
        param: String,
    }

    let priv_msg = Params {
        param: "param".to_string(),
    };

    let size = priv_msg.serialized_size();
    let msg = priv_msg.to_bytes().unwrap();
    assert_eq!(" param", msg);
    assert_eq!(6, size)
}

#[test]
fn param_multi() {
    #[derive(ToMessage)]
    struct Params<'a> {
        #[irc(param = 1)]
        param: String,
        #[irc(param)]
        param2: &'a str,
    }

    let priv_msg = Params {
        param: "param".to_string(),
        param2: "param2",
    };

    let size = priv_msg.serialized_size();
    let msg = priv_msg.to_bytes().unwrap();
    assert_eq!(" param param2", msg);
    assert_eq!(13, size)
}

#[test]
fn trailing() {
    #[derive(ToMessage)]
    struct Trailing {
        #[irc(trailing)]
        message: String,
    }

    let priv_msg = Trailing {
        message: "hi".to_string(),
    };

    let size = priv_msg.serialized_size();
    let msg = priv_msg.to_bytes().unwrap();
    assert_eq!(" :hi", msg);
    assert_eq!(4, size)
}

#[test]
fn full() {
    #[derive(ToMessage)]
    #[irc(command = "PRIVMSG", crlf)]
    struct Full<'a> {
        #[irc(tag)]
        field: &'a str,
        #[irc(tag = "key")]
        field2: String,

        #[irc(tag_flag)]
        flag: bool,
        #[irc(tag_flag = "msgid")]
        flag2: bool,

        #[irc(source)]
        name: &'a str,
        #[irc(source = "user")]
        user: String,
        #[irc(source = "host")]
        host: String,

        #[irc(param)]
        param: String,
        #[irc(params)]
        param_vec: Vec<&'a str>,
        #[irc(params)]
        param_vec_string: Vec<String>,

        #[irc(trailing)]
        message: String,
    }

    let msg = Full {
        field: "value",
        field2: "value2".to_string(),
        flag: false,
        flag2: true,

        name: "nick",
        user: "user".to_string(),
        host: "example.com".to_string(),

        param: "param".to_string(),
        param_vec: vec!["param2", "param3"],
        param_vec_string: vec!["param4".to_string(), "param5".to_string()],

        message: "hi".to_string(),
    };

    let size = msg.serialized_size();
    let actual = msg.to_bytes().unwrap();

    assert_eq!(
        "@field=value;key=value2;msgid :nick!user@example.com PRIVMSG param param2 param3 param4 param5 :hi\r\n",
        actual
    );
    assert_eq!(100, size)
}
