use ircv3_parse::ParamsParse;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{not_line_ending, space1},
    sequence::tuple,
    IResult,
};
use pretty_assertions::assert_eq;

// #[test]
// fn builder_test() {
//     let msg = ":foo!foo@foo.tmi.twitch.tv PRIVMSG guest w :bleedPurple";
//     let result = IRCv3Builder::new(WhoAmI::default()).parse(msg);
//
//     assert_eq!("guest".to_string(), result.params.stats);
//     assert_eq!("w".to_string(), result.params.user);
// }

// #[derive(Default)]
// struct WhoAmI {
//     pub stats: String,
//     pub user: String,
// }

// impl ParamsParse for WhoAmI {
//     fn parse(&self, _: &str, middle: ircv3_parse::IRCv3ParamsBase) -> Self
//     where
//         Self: Sized,
//     {
//         let join_middle = middle.middle.join(" ");
//         let (_, (who, user)) = whoami(join_middle.as_str()).unwrap();
//         WhoAmI {
//             stats: who.to_string(),
//             user: user.to_string(),
//         }
//     }
// }

// pub fn whoami(msg: &str) -> IResult<&str, (&str, &str)> {
//     let (remain, who) = alt((owner_user, guest_user))(msg)?;
//     Ok((remain, who))
// }

fn owner_user(msg: &str) -> IResult<&str, (&str, &str)> {
    let (remain, (gust, _, user)) = tuple((
        tag("owner"),
        space1,
        alt((take_until(" "), not_line_ending)),
    ))(msg)?;

    Ok((remain, (gust, user)))
}

fn guest_user(msg: &str) -> IResult<&str, (&str, &str)> {
    let (remain, (gust, _, user)) = tuple((
        tag("guest"),
        space1,
        alt((take_until(" "), not_line_ending)),
    ))(msg)?;

    Ok((remain, (gust, user)))
}
