use ircv3_parse::params_parse;

#[test]
fn only_channel() {
    let msg = " #<channel>";
    let (remain, result) = params_parse(msg).unwrap();

    assert_eq!(remain, "");
    assert_eq!(result.channel(), "#<channel>");
    assert_eq!(result.message(), "");
}

#[test]
fn only_channel_rn() {
    let msg = " #<channel>\r\n";
    let (remain, result) = params_parse(msg).unwrap();

    assert_eq!(remain, "");
    assert_eq!(result.channel(), "#<channel>");
    assert_eq!(result.message(), "");
}

#[test]
fn middle() {
    let msg = " bar = #twitchdev :bar";
    let (remain, result) = params_parse(msg).unwrap();

    assert_eq!(result.channel(), "bar = #twitchdev");
    assert_eq!(result.message(), "bar");
    assert_eq!(remain, "");
}

#[test]
fn channel_message() {
    let msg = " #barbar :This room is already in unique-chat mode.";
    let (remain, result) = params_parse(msg).unwrap();

    assert_eq!(result.channel(), "#barbar");
    assert_eq!(
        result.message(),
        "This room is already in unique-chat mode."
    );
    assert_eq!(remain, "");
}

#[test]
fn empty() {
    let msg = " ";
    let (remain, result) = params_parse(msg).unwrap();

    assert_eq!(result.channel(), "");
    assert_eq!(result.message(), "");
    assert_eq!(remain, "");
}

#[test]
fn empty_rn() {
    let msg = " \r\n";
    let (remain, result) = params_parse(msg).unwrap();

    assert_eq!(result.channel(), "");
    assert_eq!(result.message(), "");
    assert_eq!(remain, "");
}
