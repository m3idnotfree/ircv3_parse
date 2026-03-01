#![allow(unused)]
#[allow(unused_imports)]
use ircv3_parse::de::FromMessage as _;
use ircv3_parse_derive::FromMessage;

#[test]
fn kebab_case() {
    #[derive(FromMessage, Debug, PartialEq)]
    #[irc(tag = "batch-type")]
    struct BatchStart;

    let input = "@batch-type=batch-start PRIVMSG #channel :hello";
    let msg: BatchStart = ircv3_parse::from_str(input).unwrap();
    assert_eq!(BatchStart, msg);
}
