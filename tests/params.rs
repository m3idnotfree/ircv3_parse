use ircv3_parse::IRCv3Params;
use nom::IResult;
// use pretty_assertions::assert_eq;

#[test]
fn only_channel() {
    let msg = " #<channel>";
    let (remain, result) = parse(msg).unwrap();

    assert_eq!(remain, "");
    assert_eq!(result.channel(), Some("#<channel>"));
    assert_eq!(result.message(), None);
}

#[test]
fn only_channel_rn() {
    let msg = " #<channel>\r\n";
    let (remain, result) = parse(msg).unwrap();

    assert_eq!(remain, "");
    assert_eq!(result.channel(), Some("#<channel>"));
    assert_eq!(result.message(), None);
}

#[test]
fn middle() {
    let msg = " bar = #twitchdev :bar";
    let (remain, result) = parse(msg).unwrap();

    assert_eq!(result.channel(), Some("bar = #twitchdev"));
    assert_eq!(result.message(), Some("bar"));
    assert_eq!(remain, "");
}
#[test]
fn channel_message() {
    let msg = " #barbar :This room is already in unique-chat mode.";
    let (remain, result) = parse(msg).unwrap();

    assert_eq!(result.channel(), Some("#barbar"));
    assert_eq!(
        result.message(),
        Some("This room is already in unique-chat mode.")
    );
    assert_eq!(remain, "");
}

#[test]
fn space_empty() {
    let msg = " ";
    let (remain, result) = parse(msg).unwrap();

    assert_eq!(result.get(), None);
    assert_eq!(result.channel(), None);
    assert_eq!(result.message(), None);
    assert_eq!(remain, " ");
}

#[test]
fn space_empty_rn() {
    let msg = " \r\n";
    let (remain, result) = parse(msg).unwrap();

    assert_eq!(result.get(), None);
    assert_eq!(result.channel(), None);
    assert_eq!(result.message(), None);
    assert_eq!(remain, " \r\n");
}

#[test]
fn empty() {
    let msg = "";
    let (remain, result) = parse(msg).unwrap();

    assert_eq!(result.get(), None);
    assert_eq!(result.channel(), None);
    assert_eq!(result.message(), None);
    assert_eq!(remain, "");
}

#[test]
fn empty_rn() {
    let msg = "\r\n";
    let (remain, result) = parse(msg).unwrap();

    assert_eq!(result.get(), None);
    assert_eq!(result.channel(), None);
    assert_eq!(result.message(), None);
    assert_eq!(remain, "\r\n");
}

fn parse(msg: &str) -> IResult<&str, IRCv3Params> {
    IRCv3Params::parse(msg)
}
