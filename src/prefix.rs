use nom::{
    bytes::complete::{tag, take_while},
    character::complete::space1,
    combinator::opt,
    sequence::{delimited, preceded, tuple},
    IResult,
};

pub fn prefix_parse(msg: &str) -> IResult<&str, IRCv3Prefix> {
    Ircv3Prefix::parse(msg)
}

#[derive(Debug, Clone, PartialEq)]
pub struct IRCv3Prefix<'a>(Option<(&'a str, Option<&'a str>)>);

impl<'a> IRCv3Prefix<'a> {
    pub fn new(prefix: Option<(&'a str, Option<&'a str>)>) -> Self {
        Self(prefix)
    }

    pub fn server_nick(&self) -> Option<&str> {
        self.0.as_ref().map(|value| value.0)
    }

    pub fn user(&self) -> Option<&str> {
        match self.0.as_ref() {
            None => None,
            Some(value) => value.1,
        }
    }

    pub fn get(&self) -> Option<(&'a str, Option<&'a str>)> {
        self.0.as_ref().copied()
    }
}

impl<'a> AsRef<Option<(&'a str, Option<&'a str>)>> for IRCv3Prefix<'a> {
    fn as_ref(&self) -> &Option<(&'a str, Option<&'a str>)> {
        &self.0
    }
}

struct Ircv3Prefix;

impl Ircv3Prefix {
    pub fn parse(msg: &str) -> IResult<&str, IRCv3Prefix> {
        let (msg, data) = Ircv3Prefix::prefix_parse(msg)?;

        Ok((msg, IRCv3Prefix(data)))
    }

    fn prefix_parse(msg: &str) -> IResult<&str, Option<(&str, Option<&str>)>> {
        opt(delimited(
            tag(":"),
            tuple((Ircv3Prefix::server_nick, Ircv3Prefix::opts_user)),
            space1,
        ))(msg)
    }

    fn server_nick(msg: &str) -> IResult<&str, &str> {
        take_while(|c: char| !c.is_whitespace() && c != '!')(msg)
    }

    fn opts_user(msg: &str) -> IResult<&str, Option<&str>> {
        opt(preceded(
            tag("!"),
            // take_while(|c: char| !c.is_whitespace() && c != '@'),
            take_while(|c: char| !c.is_whitespace()),
        ))(msg)
    }

    // fn opts_host(msg: &str) -> IResult<&str, Option<&str>> {
    //     opt(preceded(tag("@"), take_until(" ")))(msg)
    // }
}
