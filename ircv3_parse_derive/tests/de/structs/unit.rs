#![allow(unused)]
#[allow(unused_imports)]
use ircv3_parse::de::FromMessage as _;
use ircv3_parse_derive::{FromMessage, ToMessage};

#[test]
fn kebab_case() {
    #[derive(Debug, PartialEq, FromMessage, ToMessage)]
    #[irc(tag = "batch-type")]
    struct BatchStart;

    let input = "@batch-type=batch-start PRIVMSG #channel :hello";
    let msg: BatchStart = ircv3_parse::from_str(input).unwrap();
    assert_eq!(BatchStart, msg);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("@batch-type=batch-start ", output);
}
