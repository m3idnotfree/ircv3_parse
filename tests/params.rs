use ircv3_parse::params_parse;
use pretty_assertions::assert_eq;

#[test]
fn only_channel() {
    let msg = " #<channel>";
    let (remain, result) = params_parse(msg).unwrap();

    assert_eq!(remain, "");
    assert_eq!(0, result.middle.len());
    assert_eq!(None, result.message);
    assert_eq!(Some("<channel>".to_string()), result.channel);
}

#[test]
fn only_channel_rn() {
    let msg = " #<channel>\r\n";
    let (remain, result) = params_parse(msg).unwrap();

    assert_eq!(remain, "");
    assert_eq!(result.channel, Some("<channel>".to_string()));
}

#[test]
fn middle() {
    let msg = " bar = #twitchdev :bar";
    let (remain, result) = params_parse(msg).unwrap();

    assert_eq!(result.message, Some("bar".to_string()));
    assert_eq!(0, result.middle.len());
    assert_eq!(Some("bar = twitchdev".to_string()), result.channel);
    assert_eq!(remain, "");
}

#[test]
fn channel_message() {
    let msg = " #barbar :This room is already in unique-chat mode.";
    let (remain, result) = params_parse(msg).unwrap();

    assert_eq!(result.channel, Some("barbar".to_string()));
    assert_eq!(
        result.message,
        Some("This room is already in unique-chat mode.".to_string())
    );
    assert_eq!(remain, "");
}

#[test]
fn space_empty() {
    let msg = " ";
    let (remain, result) = params_parse(msg).unwrap();

    assert_eq!(None, result.channel);
    assert_eq!(1, result.middle.len());
    assert_eq!(None, result.message);
    assert_eq!(remain, "");
}

#[test]
fn space_empty_rn() {
    let msg = " \r\n";
    let (remain, result) = params_parse(msg).unwrap();

    assert_eq!(None, result.channel);
    assert_eq!(1, result.middle.len());
    assert_eq!(None, result.message);
    assert_eq!(remain, "");
}

#[test]
fn empty() {
    let msg = "";
    let (remain, result) = params_parse(msg).unwrap();

    assert_eq!(None, result.channel);
    assert_eq!(0, result.middle.len());
    assert_eq!("", remain);
}

#[test]
fn empty_rn() {
    let msg = "\r\n";
    let (remain, result) = params_parse(msg).unwrap();

    // assert_eq!(result.get(), None);
    assert_eq!(None, result.channel);
    assert_eq!(0, result.middle.len());
    assert_eq!("", remain);
}

#[test]
fn channel_message_base() {
    let msg = " #ronni :Kappa Keepo Kappa";
    let (remain, result) = params_parse(msg).unwrap();

    assert_eq!(result.channel, Some("ronni".to_string()));
    assert_eq!(result.message, Some("Kappa Keepo Kappa".to_string()));
    assert_eq!(remain, "");
}
#[test]
fn channel_message_rn() {
    let msg = " #ronni :Kappa Keepo Kappa\r\n";
    let (remain, result) = params_parse(msg).unwrap();

    assert_eq!(result.channel, Some("ronni".to_string()));
    assert_eq!(result.message, Some("Kappa Keepo Kappa".to_string()));
    assert_eq!(remain, "");
}
