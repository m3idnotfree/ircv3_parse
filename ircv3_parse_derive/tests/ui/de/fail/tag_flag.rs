use ircv3_parse_derive::FromMessage;

#[derive(FromMessage)]
#[irc(tag_flag = "moderator")]
enum A {
    Present,
    Maybe,
    Absent,
}

#[derive(FromMessage)]
#[irc(tag_flag = "moderator")]
enum B {
    #[irc(present)]
    Present,
    Absent {
        reason: String,
    },
}

#[derive(FromMessage)]
#[irc(tag_flag = "moderator")]
enum C {
    #[irc(present)]
    Present,
    #[irc(value = "no")]
    Absent,
}

#[derive(FromMessage)]
#[irc(param)]
enum D {
    #[irc(present)]
    Channel,
    Server,
}

#[derive(FromMessage)]
#[irc(tag_flag = "moderator")]
enum E {
    #[irc(present = "yes")]
    Present,
    Absent,
}

fn main() {}
