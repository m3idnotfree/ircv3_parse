use std::collections::HashMap;

use ircv3_tags::Ircv3TagsParse;
use nom::{
    bytes::complete::{tag, take_until, take_while},
    character::complete::{not_line_ending, space1},
    combinator::opt,
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[derive(Debug)]
pub struct Ircv3Parse<'a> {
    pub tags: Ircv3TagsParse<'a>,
    pub prefix: Ircv3Prefix<'a>,

    pub command: String,
    pub message: &'a str,
}

impl<'a> Ircv3Parse<'a> {
    pub fn new(msg: &str) -> Ircv3Parse {
        let tags = Ircv3TagsParse::new(msg);
        let prefix = Ircv3Prefix::new(tags.msg);
        let (message, command) = Ircv3Parse::command_parse(prefix.msg).unwrap();

        Ircv3Parse {
            tags,
            prefix,
            command: command.to_string(),
            message,
        }
    }

    pub fn command_parse(msg: &str) -> IResult<&str, &str> {
        take_until(" ")(msg)
    }
}

#[derive(Debug, PartialEq)]
pub struct Ircv3Prefix<'a> {
    prefix: Option<(&'a str, Option<&'a str>)>,
    pub msg: &'a str,
}

impl<'a> Ircv3Prefix<'a> {
    pub fn new(msg: &str) -> Ircv3Prefix {
        let (msg, prefix) = Ircv3Prefix::prefix_parse(msg).unwrap();
        Ircv3Prefix { prefix, msg }
    }

    pub fn to_string(self) -> Option<(String, Option<String>)> {
        match self.prefix {
            Some(value) => {
                let (server_nick, host) = value;
                Some((server_nick.to_string(), host.map(str::to_string)))
            }
            None => None,
        }
    }

    pub fn to_str(self) -> Option<(&'a str, Option<&'a str>)> {
        self.prefix
    }

    pub fn prefix_parse(msg: &str) -> IResult<&str, Option<(&str, Option<&str>)>> {
        opt(delimited(
            tag(":"),
            tuple((Ircv3Prefix::server_nick, Ircv3Prefix::opts_user)),
            space1,
        ))(msg)
    }

    pub fn server_nick(msg: &str) -> IResult<&str, &str> {
        take_while(|c: char| !c.is_whitespace() && c != '!')(msg)
    }

    pub fn opts_user(msg: &str) -> IResult<&str, Option<&str>> {
        println!("msd = {:#?}", msg);
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

pub fn channel_message(msg: &str) -> IResult<&str, HashMap<String, String>> {
    let (msg, channel) = delimited(space1, take_until(" "), space1)(msg)?;
    let (remain, (_, message)) = tuple((tag(":"), not_line_ending))(msg)?;

    let mut map = HashMap::new();
    map.insert("channel".to_string(), channel.to_string());
    map.insert("message".to_string(), message.to_string());

    Ok((remain, map))
}

pub fn middle_message(msg: &str) -> IResult<&str, HashMap<String, String>> {
    let (msg, middle) = delimited(space1, take_until(":"), tag(":"))(msg)?;
    let (_, message) = preceded(tag(":"), not_line_ending)(msg)?;
    let mut map = HashMap::new();
    map.insert("middle".to_string(), middle.to_string());
    map.insert("message".to_string(), message.to_string());

    Ok((message, map))
}
