use proptest::prelude::{prop_compose, prop_oneof, Just, Strategy};

prop_compose! {
    pub fn command_strategy()(
        cmd in prop_oneof![
            Just("CAP".to_string()),
            Just("AUTHENTICATE".to_string()),
            Just("PASS".to_string()),
            Just("NICK".to_string()),
            Just("USER".to_string()),
            Just("PING".to_string()),
            Just("PONG".to_string()),
            Just("OPER".to_string()),
            Just("QUIT".to_string()),
            Just("ERROR".to_string()),
            Just("JOIN".to_string()),
            Just("PART".to_string()),
            Just("TOPIC".to_string()),
            Just("NAMES".to_string()),
            Just("LIST".to_string()),
            Just("INVITE".to_string()),
            Just("KICK".to_string()),
            Just("MOTD".to_string()),
            Just("VERSION".to_string()),
            Just("ADMIN".to_string()),
            Just("CONNECT".to_string()),
            Just("LUSERS".to_string()),
            Just("TIME".to_string()),
            Just("STATS".to_string()),
            Just("HELP".to_string()),
            Just("INFO".to_string()),
            Just("MODE".to_string()),
            Just("PRIVMSG".to_string()),
            Just("NOTICE".to_string()),
            Just("WHO".to_string()),
            Just("WHOIS".to_string()),
            Just("WHOWAS".to_string()),
            Just("KILL".to_string()),
            Just("REHASH".to_string()),
            Just("RESTART".to_string()),
            Just("SQUIT".to_string()),
            Just("AWAY".to_string()),
            Just("LINKS".to_string()),
            Just("USERHOST".to_string()),
            Just("WALLOPS".to_string()),
            (000u16..=999u16).prop_map(|n| format!("{:03}", n)),
            "[a-zA-Z]{1,10}",
    ]) -> String { cmd }
}

prop_compose! {
    pub fn invalid_command_strategy()(
        cmd in prop_oneof![
            Just("".to_string()),
            Just(" ".to_string()),
            (0u16..=99u16).prop_map(|n| n.to_string()),
            (1000u16..=9999u16).prop_map(|n| n.to_string()),
            (0u16..=999u16).prop_map(|n| format!("{}A", n)),
            (0u16..=999u16).prop_map(|n| format!("A{}", n)),
            "[a-zA-Z]{1,10}".prop_map(|c| format!("1{}", c)),
            "[a-zA-Z]{1,10}".prop_map(|c| format!("{}1", c)),
    ]) -> String { cmd }
}
