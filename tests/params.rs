use ircv3_parse::Ircv3Parse;

#[test]
fn only_channel() {
    let msg = ":<user>!<user>@<user>.tmi.twitch.tv JOIN #<channel>";
    let result = Ircv3Parse::new(msg);
    // let result =result.params.channel().unwrap();
    let (r, result) = result.params.channel().unwrap();
    assert_eq!(r, "");
    assert_eq!(result, "#<channel>");
}
#[test]
fn only_channel_rn() {
    let msg = ":<user>!<user>@<user>.tmi.twitch.tv JOIN #<channel>\r\n";
    let result = Ircv3Parse::new(msg);
    let (r, result) = result.params.channel().unwrap();
    assert_eq!(r, "\r\n");
    assert_eq!(result, "#<channel>");
}
