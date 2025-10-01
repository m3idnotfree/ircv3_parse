use ircv3_parse::parse;

#[test]
fn full_mesasge() {
    let input = "@aaa=bbb;ccc;example.com/ddd=eee;+fff=;ggg=hello\\sworld :nick!user@host.com PRIVMSG #channel :Hello World!";
    let message = parse(input).unwrap();

    let json = serde_json::to_string(&message).unwrap();

    let value: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(value["tags"]["aaa"], "bbb");
    assert_eq!(value["tags"]["ccc"], serde_json::Value::Null);
    assert_eq!(value["tags"]["example.com/ddd"], "eee");
    assert_eq!(value["tags"]["+fff"], "");
    assert_eq!(value["tags"]["ggg"], "hello\\sworld");
    assert_eq!(value["source"]["name"], "nick");
    assert_eq!(value["source"]["user"], "user");
    assert_eq!(value["source"]["host"], "host.com");
    assert_eq!(value["command"], "PRIVMSG");
    assert_eq!(value["params"]["middles"][0], "#channel");
    assert_eq!(value["params"]["trailing"], "Hello World!");
}

#[test]
fn minimal_message() {
    let input = "PING";
    let message = parse(input).unwrap();

    let json = serde_json::to_string(&message).unwrap();
    assert_eq!(json, r#"{"command":"PING"}"#);
}

#[test]
fn with_params_only() {
    let input = "PRIVMSG #channel :Hello";
    let message = parse(input).unwrap();

    let json = serde_json::to_string(&message).unwrap();

    let value: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert!(value["tags"].is_null());
    assert!(value["source"].is_null());
    assert_eq!(value["command"], "PRIVMSG");
    assert_eq!(value["params"]["middles"][0], "#channel");
    assert_eq!(value["params"]["trailing"], "Hello");
}

#[test]
fn with_source_only() {
    let input = ":server.example.com 001 client :Welcome";
    let message = parse(input).unwrap();

    let json = serde_json::to_string(&message).unwrap();

    let value: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert!(value["tags"].is_null());
    assert_eq!(value["source"]["name"], "server.example.com");
    assert!(value["source"]["user"].is_null());
    assert!(value["source"]["host"].is_null());
    assert_eq!(value["command"], "001");
    assert_eq!(value["params"]["middles"][0], "client");
    assert_eq!(value["params"]["trailing"], "Welcome");
}

#[test]
fn source_variations() {
    let msg1 = parse(":irc.example.com NOTICE * :Hello").unwrap();
    let json1 = serde_json::to_string(&msg1).unwrap();
    let val1: serde_json::Value = serde_json::from_str(&json1).unwrap();
    assert_eq!(val1["source"]["name"], "irc.example.com");
    assert!(val1["source"]["user"].is_null());
    assert!(val1["source"]["host"].is_null());

    let msg2 = parse(":nick!user@host.com PRIVMSG #test :Hi").unwrap();
    let json2 = serde_json::to_string(&msg2).unwrap();
    let val2: serde_json::Value = serde_json::from_str(&json2).unwrap();
    assert_eq!(val2["source"]["name"], "nick");
    assert_eq!(val2["source"]["user"], "user");
    assert_eq!(val2["source"]["host"], "host.com");

    let msg3 = parse(":nick!user PRIVMSG #test :Hi").unwrap();
    let json3 = serde_json::to_string(&msg3).unwrap();
    let val3: serde_json::Value = serde_json::from_str(&json3).unwrap();
    assert_eq!(val3["source"]["name"], "nick");
    assert_eq!(val3["source"]["user"], "user");
    assert!(val3["source"]["host"].is_null());
}

#[test]
fn params_variations() {
    let msg1 = parse("MODE #channel +o user").unwrap();
    let json1 = serde_json::to_string(&msg1).unwrap();
    let val1: serde_json::Value = serde_json::from_str(&json1).unwrap();
    assert_eq!(val1["params"]["middles"][0], "#channel");
    assert_eq!(val1["params"]["middles"][1], "+o");
    assert_eq!(val1["params"]["middles"][2], "user");
    assert!(val1["params"]["trailing"].is_null());

    let msg2 = parse("PRIVMSG #channel :Hello World").unwrap();
    let json2 = serde_json::to_string(&msg2).unwrap();
    let val2: serde_json::Value = serde_json::from_str(&json2).unwrap();
    assert_eq!(val2["params"]["middles"][0], "#channel");
    assert_eq!(val2["params"]["trailing"], "Hello World");

    let msg3 = parse("PRIVMSG #channel target :Message text").unwrap();
    let json3 = serde_json::to_string(&msg3).unwrap();
    let val3: serde_json::Value = serde_json::from_str(&json3).unwrap();
    assert_eq!(val3["params"]["middles"][0], "#channel");
    assert_eq!(val3["params"]["middles"][1], "target");
    assert_eq!(val3["params"]["trailing"], "Message text");
}

#[test]
fn numeric_command() {
    let input = ":server.com 001 nick :Welcome to IRC";
    let message = parse(input).unwrap();

    let json = serde_json::to_string(&message).unwrap();
    let value: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(value["command"], "001");
}
