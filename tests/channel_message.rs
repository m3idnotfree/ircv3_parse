use std::collections::HashMap;

use ircv3_parse::{self, channel_message};
use pretty_assertions::assert_eq;

#[test]
fn channel_message_base() {
    let msg = " #ronni :Kappa Keepo Kappa";
    let (remain, c_m) = channel_message(msg).unwrap();
    let expect_channel = "#ronni";
    let expect_message = "Kappa Keepo Kappa";
    let mut map = HashMap::new();
    map.insert("channel", expect_channel);
    map.insert("message", expect_message);

    assert_eq!(c_m, map);
    assert_eq!(remain, "");
}
#[test]
fn channel_message_rn() {
    let msg = " #ronni :Kappa Keepo Kappa\r\n";
    let (remain, c_m) = channel_message(msg).unwrap();
    let expect_channel = "#ronni";
    let expect_message = "Kappa Keepo Kappa";
    let mut map = HashMap::new();
    map.insert("channel", expect_channel);
    map.insert("message", expect_message);

    assert_eq!(c_m, map);
    assert_eq!(remain, "\r\n");
}
