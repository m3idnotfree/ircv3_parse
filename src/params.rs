use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{crlf, not_line_ending, space1},
    combinator::eof,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

pub fn params_parse(msg: &str) -> IResult<&str, IRCv3Params> {
    Ircv3Params::parse(msg)
}

#[derive(Debug, Clone, PartialEq)]
pub struct IRCv3Params<'a>((&'a str, &'a str));

impl<'a> IRCv3Params<'a> {
    pub fn new(params: (&'a str, &'a str)) -> Self {
        Self(params)
    }

    pub fn channel(&self) -> &str {
        self.0 .0
    }

    pub fn message(&self) -> &str {
        self.0 .1
    }
}

struct Ircv3Params;

impl Ircv3Params {
    pub fn parse(msg: &str) -> IResult<&str, IRCv3Params> {
        let (remain, data) = preceded(space1, Ircv3Params::pars)(msg)?;

        Ok((remain, IRCv3Params(data)))
    }

    pub fn pars(msg: &str) -> IResult<&str, (&str, &str)> {
        alt((
            Ircv3Params::channel_n_message,
            Ircv3Params::middle_n_message,
            Ircv3Params::empty,
            Ircv3Params::only_channel,
        ))(msg)
    }

    fn channel_n_message(msg: &str) -> IResult<&str, (&str, &str)> {
        tuple((
            terminated(take_until(" "), space1),
            delimited(tag(":"), not_line_ending, alt((crlf, eof))),
        ))(msg)
    }

    fn middle_n_message(msg: &str) -> IResult<&str, (&str, &str)> {
        let (remain, (middle, message)) = tuple((
            terminated(take_until(":"), tag(":")),
            terminated(not_line_ending, alt((crlf, eof))),
        ))(msg)?;

        Ok((remain, (middle.trim(), message)))
    }

    fn empty(msg: &str) -> IResult<&str, (&str, &str)> {
        let (_, _) = alt((crlf, eof))(msg)?;
        Ok(("", ("", "")))
    }

    fn only_channel(msg: &str) -> IResult<&str, (&str, &str)> {
        let (msg, channel) = terminated(not_line_ending, alt((crlf, eof)))(msg)?;
        Ok((msg, (channel, "")))
    }
}
