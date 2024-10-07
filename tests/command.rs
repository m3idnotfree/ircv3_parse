use ircv3_parse::command_parse;
use pretty_assertions::assert_eq;

#[test]
fn command_privmsg() {
    let msg = "PRIVMSG #bar :bleedPurple";
    let (remain, command) = command_parse(msg).unwrap();

    assert_eq!("PRIVMSG", command);
    assert_eq!(" #bar :bleedPurple", remain);
}

#[test]
fn command_digit() {
    let msg = "857 #bar :bleedPurple";
    let (remain, command) = command_parse(msg).unwrap();

    assert_eq!("857", command);
    assert_eq!(" #bar :bleedPurple", remain);
}
