use ircv3_parse::params_parse;
use pretty_assertions::assert_eq;

#[test]
fn channel_message_base() {
    let msg = " #ronni :Kappa Keepo Kappa";
    let (remain, params) = params_parse(msg).unwrap();

    assert_eq!(params.channel(), "#ronni");
    assert_eq!(params.message(), "Kappa Keepo Kappa");
    assert_eq!(remain, "");
}
#[test]
fn channel_message_rn() {
    let msg = " #ronni :Kappa Keepo Kappa\r\n";
    let (remain, params) = params_parse(msg).unwrap();

    assert_eq!(params.channel(), "#ronni");
    assert_eq!(params.message(), "Kappa Keepo Kappa");
    assert_eq!(remain, "");
}
