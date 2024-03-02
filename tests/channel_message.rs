use ircv3_parse::{self, ChannelnMsg, Ircv3Params};
use pretty_assertions::assert_eq;

#[test]
fn channel_message_base() {
    let msg = " #ronni :Kappa Keepo Kappa";
    let (remain, binding) = Ircv3Params::parse(msg).unwrap();
    let (_, c_m) = binding.channel_n_message().unwrap();

    let expected_c_m = ChannelnMsg::new("#ronni", "Kappa Keepo Kappa");

    assert_eq!(c_m, expected_c_m);
    assert_eq!(remain, "");
}
#[test]
fn channel_message_rn() {
    let msg = " #ronni :Kappa Keepo Kappa\r\n";
    let (_, binding) = Ircv3Params::parse(msg).unwrap();
    let (remain, c_m) = binding.channel_n_message().unwrap();
    let expected_c_m = ChannelnMsg::new("#ronni", "Kappa Keepo Kappa");

    assert_eq!(c_m, expected_c_m);
    assert_eq!(remain, "");
}
