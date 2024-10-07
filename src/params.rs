use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while},
    character::complete::{crlf, not_line_ending, space1},
    combinator::opt,
    sequence::preceded,
    IResult,
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct IRCv3Params {
    pub channel: Option<String>,
    pub middle: Vec<String>,
    pub message: Option<String>,
}

pub fn params_parse(msg: &str) -> IResult<&str, IRCv3Params> {
    let a = IRCv3Params::default();
    params_inner(msg, a)
}

pub fn params_inner(msg: &str, mut ircv3_params: IRCv3Params) -> IResult<&str, IRCv3Params> {
    if msg.is_empty() {
        Ok((msg, ircv3_params))
    } else {
        let (remain, empty_crlf) = opt(msg_crlf)(msg)?;
        if let Some(empty_crlf) = empty_crlf {
            // Ok((empty_crlf, ircv3_params))
            Ok((remain, ircv3_params))
        } else {
            let (remain, (message, channel, middle)) =
                preceded(space1, alt((trailing, channel, middle)))(remain)?;
            if message.is_some() {
                ircv3_params.message = message.map(String::from);
            }

            if channel.is_some() && ircv3_params.channel.is_none() {
                let is_eq = ircv3_params.middle.pop();

                let c = if is_eq.is_some() && is_eq.unwrap() == "=" {
                    let rename = ircv3_params.middle.pop().unwrap();
                    Some(format!("{rename} = {}", channel.unwrap()))
                } else {
                    channel.map(String::from)
                };
                ircv3_params.channel = c;
            }

            if let Some(middle) = middle {
                ircv3_params.middle.push(middle.to_string());
            }
            params_inner(remain, ircv3_params)
        }
    }
}

type ParamsReturn<'a> = IResult<&'a str, (Option<&'a str>, Option<&'a str>, Option<&'a str>)>;
/// (remain, (empty_crlf, message, channel ,middle))
fn msg_empty(msg: &str) -> IResult<&str, &str> {
    tag("")(msg)
}

/// (remain, (empty_crlf, message, channel ,middle))
fn msg_crlf(msg: &str) -> IResult<&str, &str> {
    crlf(msg)
}

/// (remain, (message, channel ,middle))
fn trailing(msg: &str) -> ParamsReturn {
    let (remain, message) = preceded(tag(":"), not_line_ending)(msg)?;

    Ok((remain, (Some(message), None, None)))
}
/// (remain, (message, channel ,middle))
fn channel(msg: &str) -> ParamsReturn {
    let (remain, channel) = preceded(tag("#"), alt((take_until(" "), not_line_ending)))(msg)?;

    Ok((remain, (None, Some(channel), None)))
}
/// (remain, (message, channel, middle))
fn middle(msg: &str) -> ParamsReturn {
    let (remain, middle) = take_while(|c: char| c != ':' && !c.is_whitespace())(msg)?;

    Ok((remain, (None, None, Some(middle))))
}
