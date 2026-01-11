#[cfg(feature = "derive")]
#[test]
fn roundtip() {
    use ircv3_parse::message::ser::ToMessage;
    use ircv3_parse::{Commands, FromMessage, ToMessage};

    #[derive(FromMessage, ToMessage)]
    #[irc(crlf)]
    struct Message<'a> {
        #[irc(tag)]
        subscriper: String,
        #[irc(tag)]
        msgid: Option<&'a str>,
        #[irc(command = "PRIVMSG")]
        command: Commands<'a>,
        #[irc(trailing)]
        message: &'a str,
    }

    let msg = Message {
        subscriper: "1".to_string(),
        msgid: Some(""),
        command: Commands::PRIVMSG,
        message: "hi",
    };

    let size = msg.serialized_size();
    let msg_bytes = ircv3_parse::to_message(&msg).unwrap();

    assert_eq!("@subscriper=1;msgid= PRIVMSG :hi\r\n", msg_bytes);
    assert_eq!(34, size);

    let msg_str = String::from_utf8(msg_bytes.to_vec()).unwrap();
    let de: Message = ircv3_parse::from_str(&msg_str).unwrap();

    assert_eq!(msg.subscriper, de.subscriper);
    assert_eq!(msg.msgid, de.msgid);
    assert_eq!(msg.command, de.command);
    assert_eq!(msg.message, de.message);
}
