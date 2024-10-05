use nom::{
    bytes::complete::{tag, take_while},
    character::complete::space1,
    combinator::opt,
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[derive(Debug, Clone, PartialEq)]
pub struct IRCv3Prefix((String, Option<String>));

impl IRCv3Prefix {
    pub fn parse(msg: &str) -> IResult<&str, Option<IRCv3Prefix>> {
        let (msg, data) = prefix_parse(msg)?;

        Ok((
            msg,
            data.map(|x| IRCv3Prefix((x.0.to_string(), x.1.map(|x| x.to_string())))),
        ))
    }

    pub fn server_nick(&self) -> String {
        self.0 .0.clone()
    }

    pub fn user(&self) -> Option<String> {
        self.0 .1.clone()
    }

    // fn opts_host(msg: &str) -> IResult<&str, Option<&str>> {
    //     opt(preceded(tag("@"), take_until(" ")))(msg)
    // }
}

fn prefix_parse(msg: &str) -> IResult<&str, Option<(&str, Option<&str>)>> {
    opt(delimited(tag(":"), tuple((server_nic, opts_user)), space1))(msg)
}

fn server_nic(msg: &str) -> IResult<&str, &str> {
    take_while(|c: char| !c.is_whitespace() && c != '!')(msg)
}

fn opts_user(msg: &str) -> IResult<&str, Option<&str>> {
    opt(preceded(
        tag("!"),
        // take_while(|c: char| !c.is_whitespace() && c != '@'),
        take_while(|c: char| !c.is_whitespace()),
    ))(msg)
}
