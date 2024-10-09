use std::collections::VecDeque;

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
    pub middle: VecDeque<String>,
    pub message: Option<String>,
}

type ParamsReturn<'a> = IResult<&'a str, IRCv3Params>;

pub fn params_parse(msg: &str) -> ParamsReturn {
    let a = IRCv3Params::default();
    params_inner(msg, a)
}

pub fn params_inner(msg: &str, mut ircv3_params: IRCv3Params) -> ParamsReturn {
    if msg.is_empty() {
        Ok((msg, ircv3_params))
    } else {
        let (remain, empty_crlf) = opt(msg_crlf)(msg)?;
        if empty_crlf.is_some() {
            Ok((remain, ircv3_params))
        } else {
            let (remain, (message, _, middle)) = preceded(space1, alt((message, middle)))(remain)?;

            if message.is_some() {
                ircv3_params.message = message.map(String::from);
            }

            if let Some(middle) = middle {
                ircv3_params.middle.push_back(middle.to_string());
            }
            params_inner(remain, ircv3_params)
        }
    }
}

/// (remain, (message, channel ,middle))
type ParamsNomReturn<'a> = IResult<&'a str, (Option<&'a str>, Option<String>, Option<&'a str>)>;

fn msg_crlf(msg: &str) -> IResult<&str, &str> {
    crlf(msg)
}

/// (remain, (message, channel ,middle))
fn message(msg: &str) -> ParamsNomReturn {
    let (remain, message) = preceded(tag(":"), not_line_ending)(msg)?;

    Ok((remain, (Some(message), None, None)))
}

/// (remain, (message, channel, middle))
fn middle(msg: &str) -> ParamsNomReturn {
    let (remain, middle) = take_while(|c: char| c != ':' && !c.is_whitespace())(msg)?;

    Ok((remain, (None, None, Some(middle))))
}
