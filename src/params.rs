use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{crlf, not_line_ending, space1},
    combinator::{eof, not, opt},
    sequence::{preceded, terminated, tuple},
    IResult,
};

#[derive(Debug, Clone, PartialEq)]
pub struct IRCv3Params<'a>(Option<(&'a str, Option<&'a str>)>);

impl<'a> IRCv3Params<'a> {
    pub fn parse(msg: &str) -> IResult<&str, IRCv3Params> {
        let (remain, data) = opt(preceded(
            space1,
            terminated(
                alt((
                    IRCv3Params::channel_and_message,
                    IRCv3Params::middle_and_message,
                    IRCv3Params::only_channel,
                )),
                alt((crlf, eof)),
            ),
        ))(msg)?;

        Ok((remain, IRCv3Params(data)))
    }

    pub fn get(&self) -> Option<(&'a str, Option<&'a str>)> {
        self.0
    }

    pub fn channel(&self) -> Option<&str> {
        self.0.map(|value| value.0)
    }

    pub fn message(&self) -> Option<&str> {
        match self.0 {
            None => None,
            Some(value) => value.1,
        }
    }

    fn channel_and_message(msg: &str) -> IResult<&str, (&str, Option<&str>)> {
        let (msg, (channel, message)) = tuple((
            terminated(take_until(" "), space1),
            preceded(tag(":"), not_line_ending),
        ))(msg)?;

        Ok((msg, (channel, Some(message))))
    }

    fn middle_and_message(msg: &str) -> IResult<&str, (&str, Option<&str>)> {
        let (remain, (middle, message)) =
            tuple((terminated(take_until(":"), tag(":")), not_line_ending))(msg)?;

        Ok((remain, (middle.trim(), Some(message))))
    }

    fn only_channel(msg: &str) -> IResult<&str, (&str, Option<&str>)> {
        not(eof)(msg)?;
        not(crlf)(msg)?;

        let (msg, channel) = not_line_ending(msg)?;
        Ok((msg, (channel, None)))
    }
}

impl<'a> AsRef<Option<(&'a str, Option<&'a str>)>> for IRCv3Params<'a> {
    fn as_ref(&self) -> &Option<(&'a str, Option<&'a str>)> {
        &self.0
    }
}
