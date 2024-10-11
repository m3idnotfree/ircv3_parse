use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while},
    character::complete::{crlf, not_line_ending, space1},
    combinator::opt,
    sequence::{preceded, tuple},
    IResult,
};

pub trait ParamsParse {
    fn parse(&self, command: &str, params: IRCv3ParamsBase) -> Self
    where
        Self: Sized;
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct IRCv3Params {
    pub channel: Option<Channel>,
    pub message: Option<String>,
    pub unknwon: String,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Channel {
    pub name: String,
    pub alt: Option<String>,
}

impl ParamsParse for IRCv3Params {
    fn parse(&self, _: &str, middles: IRCv3ParamsBase) -> Self {
        let join_middles = middles.middle.join(" ");
        let (unknown, channel) = channel(&join_middles).unwrap();

        IRCv3Params {
            channel: channel.map(|(name, alt)| Channel { name, alt }),
            message: middles.message,
            unknwon: unknown.trim().to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct IRCv3ParamsBase {
    pub middle: Vec<String>,
    pub message: Option<String>,
}

type ParamsReturn<'a> = IResult<&'a str, IRCv3ParamsBase>;

pub fn params_parse(msg: &str) -> ParamsReturn {
    params_inner(msg, IRCv3ParamsBase::default())
}

pub fn params_inner(msg: &str, mut ircv3_params: IRCv3ParamsBase) -> ParamsReturn {
    if msg.is_empty() {
        Ok((msg, ircv3_params))
    } else {
        let (remain, empty_crlf) = opt(msg_crlf)(msg)?;
        if empty_crlf.is_some() {
            Ok((remain, ircv3_params))
        } else {
            let (remain, (message, middle)) = preceded(space1, alt((message, middle)))(remain)?;

            if message.is_some() {
                ircv3_params.message = message.map(String::from);
            }

            if let Some(middle) = middle {
                ircv3_params.middle.push(middle.to_string());
            }

            params_inner(remain, ircv3_params)
        }
    }
}

/// (remain, (message, middle))
type ParamsNomReturn<'a> = IResult<&'a str, (Option<&'a str>, Option<&'a str>)>;

fn msg_crlf(msg: &str) -> IResult<&str, &str> {
    crlf(msg)
}

/// (remain, (message, middle))
fn message(msg: &str) -> ParamsNomReturn {
    let (remain, message) = preceded(tag(":"), not_line_ending)(msg)?;

    Ok((remain, (Some(message), None)))
}

/// (remain, (message, middle))
fn middle(msg: &str) -> ParamsNomReturn {
    let (remain, middle) = take_while(|c: char| c != ':' && !c.is_whitespace())(msg)?;

    Ok((remain, (None, Some(middle))))
}

/// (remain, channel)
pub fn channel(msg: &str) -> IResult<&str, Option<(String, Option<String>)>> {
    let (remain, channel) = opt(alt((only_channel, alt_eq_channel, alt_spc_channel)))(msg)?;
    Ok((remain, channel))
}

/// (remain, (channel, alt))
fn only_channel(msg: &str) -> IResult<&str, (String, Option<String>)> {
    let (remain, channel) = preceded(tag("#"), alt((take_until(" "), not_line_ending)))(msg)?;
    Ok((remain, (format!("#{channel}"), None)))
}

/// (remain, (channel, alt))
fn alt_eq_channel(msg: &str) -> IResult<&str, (String, Option<String>)> {
    let (remain, (as_channel, _, _, _, channel)) = tuple((
        take_until("="),
        tag("="),
        space1,
        tag("#"),
        alt((take_until(" "), not_line_ending)),
    ))(msg)?;
    Ok((
        remain,
        (
            format!("#{}", channel.trim()),
            Some(as_channel.trim().to_string()),
        ),
    ))
}

/// (remain, (channel, alt))
fn alt_spc_channel(msg: &str) -> IResult<&str, (String, Option<String>)> {
    let (remain, (as_channel, _, _, channel)) = tuple((
        take_until(" "),
        space1,
        tag("#"),
        alt((take_until(" "), not_line_ending)),
    ))(msg)?;
    Ok((
        remain,
        (format!("#{channel}"), Some(as_channel.trim().to_string())),
    ))
}
