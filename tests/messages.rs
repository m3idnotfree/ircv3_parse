use ircv3_parse::IRCv3;
use pretty_assertions::assert_eq;

// #[test]
// fn messages() {
//     let msg = ":tmi.twitch.tv 001 <user> :Welcome, GLHF!\r\n:tmi.twitch.tv 002 <user> :Your host is tmi.twitch.tv\r\n:tmi.twitch.tv 003 <user> :This server is rather new\r\n:tmi.twitch.tv 004 <user> :-\r\n:tmi.twitch.tv 375 <user> :-\r\n:tmi.twitch.tv 372 <user> :You are in a maze of twisty passages, all alike.\r\n:tmi.twitch.tv 376 <user> :>\r\n@badge-info=;badges=;color=;display-name=<user>;emote-sets=0,300374282;user-id=12345678;user-type= :tmi.twitch.tv GLOBALUSERSTATE\r\n";
//     let mut ircv3 = IRCv3::parse_vecdeque(msg);
//
//     assert_eq!(8, ircv3.len());
//     let first_message = ircv3.pop_front();
//     assert!(first_message.is_some());
//     let first_message = first_message.unwrap();
//
//     assert!(first_message.tags.is_none());
//     assert!(first_message.source.is_some());
//     let first_message_prefix = first_message.source.unwrap();
//     assert_eq!("tmi.twitch.tv", first_message_prefix.servername_nick);
//     assert_eq!(None, first_message_prefix.user);
//     assert_eq!(None, first_message_prefix.host);
//     assert_eq!("001".to_string(), first_message.command);
//     assert_eq!("<user>".to_string(), first_message.params.unknwon);
//     assert_eq!(
//         Some("Welcome, GLHF!".to_string()),
//         first_message.params.message
//     );
//
//     let last_message = ircv3.pop_back();
//     assert!(last_message.is_some());
//
//     let last_message = last_message.unwrap();
//     assert!(last_message.tags.is_some());
//     assert!(last_message.source.is_some());
//
//     let last_message_tags = last_message.tags.unwrap();
//     assert_eq!(Some("".to_string()), last_message_tags.get("badge-info"));
//     assert_eq!(Some("".to_string()), last_message_tags.get("badges"));
//     assert_eq!(Some("".to_string()), last_message_tags.get("color"));
//     assert_eq!(
//         Some("<user>".to_string()),
//         last_message_tags.get("display-name")
//     );
//     assert_eq!(
//         Some("0,300374282".to_string()),
//         last_message_tags.get("emote-sets")
//     );
//     assert_eq!(
//         Some("12345678".to_string()),
//         last_message_tags.get("user-id")
//     );
//     assert_eq!(Some("".to_string()), last_message_tags.get("user-type"));
//     assert_eq!(None, last_message_tags.get("non"));
//
//     let last_message_prefix = last_message.source.unwrap();
//     assert_eq!("tmi.twitch.tv", last_message_prefix.servername_nick);
//     assert_eq!(None, last_message_prefix.user);
//     assert_eq!(None, last_message_prefix.host);
//     assert_eq!("GLOBALUSERSTATE".to_string(), last_message.command);
//     assert_eq!("", last_message.params.unknwon);
//     assert_eq!(None, last_message.params.message);
// }
