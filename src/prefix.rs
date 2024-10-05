use nom::{
    bytes::complete::{tag, take_while},
    character::complete::space1,
    combinator::opt,
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[derive(Debug, Clone, PartialEq)]
pub struct IRCv3Prefix {
    pub servername_nick: String,
    pub user: Option<String>,
    pub host: Option<String>,
}

impl IRCv3Prefix {
    pub fn parse(msg: &str) -> IResult<&str, Option<IRCv3Prefix>> {
        let (msg, data) = prefix_parse(msg)?;

        Ok((
            msg,
            data.map(|x| IRCv3Prefix {
                servername_nick: x.0.to_string(),
                user: x.1.map(String::from),
                host: x.2.map(String::from),
            }),
        ))
    }
}

fn prefix_parse(msg: &str) -> IResult<&str, Option<(&str, Option<&str>, Option<&str>)>> {
    opt(delimited(
        tag(":"),
        tuple((server_nick, opts_user, opts_host)),
        space1,
    ))(msg)
}

fn server_nick(msg: &str) -> IResult<&str, &str> {
    take_while(|c: char| !c.is_whitespace() && c != '!')(msg)
}

fn opts_user(msg: &str) -> IResult<&str, Option<&str>> {
    opt(preceded(
        tag("!"),
        take_while(|c: char| !c.is_whitespace() && c != '@'),
        // take_while(|c: char| !c.is_whitespace()),
    ))(msg)
}
fn opts_host(msg: &str) -> IResult<&str, Option<&str>> {
    opt(preceded(
        tag("@"),
        // take_while(|c: char| !c.is_whitespace() && c != '@'),
        take_while(|c: char| !c.is_whitespace()),
    ))(msg)
}
