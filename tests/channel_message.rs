use std::collections::HashMap;

use ircv3_parse::{self, channel_message};
use pretty_assertions::assert_eq;

#[test]
fn channel_message_base() {
    let msg = " #ronni :Kappa Keepo Kappa";
    let (remain, ha) = channel_message(msg).unwrap();
    let expect_channel = "#ronni".to_string();
    let expect_message = "Kappa Keepo Kappa".to_string();
    let mut map = HashMap::new();
    map.insert("channel".to_string(), expect_channel);
    map.insert("message".to_string(), expect_message);

    assert_eq!(ha, map);
    assert_eq!(remain, "");
}
#[test]
fn channel_message_rn() {
    let msg = " #ronni :Kappa Keepo Kappa\r\n";
    let (remain, ha) = channel_message(msg).unwrap();
    let expect_channel = "#ronni".to_string();
    let expect_message = "Kappa Keepo Kappa".to_string();
    let mut map = HashMap::new();
    map.insert("channel".to_string(), expect_channel);
    map.insert("message".to_string(), expect_message);

    assert_eq!(ha, map);
    assert_eq!(remain, "\r\n");
}
