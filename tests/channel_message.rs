use ircv3_parse::{self, ChannelNMsg, Ircv3Params};
use pretty_assertions::assert_eq;

#[test]
fn channel_message_base() {
    let msg = " #ronni :Kappa Keepo Kappa";
    let binding = Ircv3Params::new(msg);
    let (remain, c_m) = binding.channel_n_message().unwrap();

    let expected_c_m = ChannelNMsg::new("#ronni", "Kappa Keepo Kappa");

    assert_eq!(c_m, expected_c_m);
    assert_eq!(remain, "");
}
#[test]
fn channel_message_rn() {
    let msg = " #ronni :Kappa Keepo Kappa\r\n";
    let binding = Ircv3Params::new(msg);
    let (remain, c_m) = binding.channel_n_message().unwrap();
    let expected_c_m = ChannelNMsg::new("#ronni", "Kappa Keepo Kappa");

    assert_eq!(c_m, expected_c_m);
    assert_eq!(remain, "\r\n");
}
