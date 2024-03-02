use ircv3_parse::Ircv3Parse;

#[test]
fn only_channel() {
    let msg = ":<user>!<user>@<user>.tmi.twitch.tv JOIN #<channel>";
    let result = Ircv3Parse::new(msg);
    let (r, result) = result.params.channel().unwrap();
    assert_eq!(r, "");
    assert_eq!(result, "#<channel>");
}
#[test]
fn only_channel_rn() {
    let msg = ":<user>!<user>@<user>.tmi.twitch.tv JOIN #<channel>\r\n";
    let result = Ircv3Parse::new(msg);
    let (r, result) = result.params.channel().unwrap();
    assert_eq!(r, "");
    assert_eq!(result, "#<channel>");
}

#[test]
fn middle() {
    let msd = ":bar.tmi.twitch.tv 353 bar = #twitchdev :bar";
    let (_, result) = Ircv3Parse::parse(msd).unwrap();
    let (remain, result) = result.params.middle_n_message().unwrap();

    assert_eq!(result.middle, "bar = #twitchdev");
    assert_eq!(result.message, "bar");
    assert_eq!(remain, "");
}
