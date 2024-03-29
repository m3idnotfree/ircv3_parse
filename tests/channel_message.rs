use ircv3_parse::IRCv3Params;
use pretty_assertions::assert_eq;

#[test]
fn channel_message_base() {
    let msg = " #ronni :Kappa Keepo Kappa";
    let (remain, params) = IRCv3Params::parse(msg).unwrap();

    assert_eq!(params.channel(), Some("#ronni"));
    assert_eq!(params.message(), Some("Kappa Keepo Kappa"));
    assert_eq!(remain, "");
}
#[test]
fn channel_message_rn() {
    let msg = " #ronni :Kappa Keepo Kappa\r\n";
    let (remain, params) = IRCv3Params::parse(msg).unwrap();

    assert_eq!(params.channel(), Some("#ronni"));
    assert_eq!(params.message(), Some("Kappa Keepo Kappa"));
    assert_eq!(remain, "");
}
