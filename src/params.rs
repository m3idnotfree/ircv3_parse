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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::params::params_parse;

    #[test]
    fn base_params_only_channel() {
        let msg = " #<channel>";
        let (remain, mut params) = params_parse(msg).unwrap();

        assert_eq!(remain, "");
        assert_eq!(1, params.middle.len());
        assert_eq!(None, params.message);
        assert_eq!(Some("#<channel>".to_string()), params.middle.pop());
    }

    #[test]
    fn base_params_only_channel_rn() {
        let msg = " #<channel>\r\n";
        let (remain, mut params) = params_parse(msg).unwrap();

        assert_eq!(remain, "");
        assert_eq!(Some("#<channel>".to_string()), params.middle.pop());
    }

    #[test]
    fn base_params_middle() {
        let msg = " bar = #twitchdev :bar";
        let (remain, mut params) = params_parse(msg).unwrap();

        assert_eq!(Some("bar".to_string()), params.message);
        assert_eq!(3, params.middle.len());
        assert_eq!(Some("#twitchdev".to_string()), params.middle.pop());
        assert_eq!(Some("=".to_string()), params.middle.pop());
        assert_eq!(Some("bar".to_string()), params.middle.pop());
        assert_eq!(remain, "");
    }

    #[test]
    fn base_params_channel_message() {
        let msg = " #barbar :This room is already in unique-chat mode.";
        let (remain, mut params) = params_parse(msg).unwrap();

        assert_eq!(Some("#barbar".to_string()), params.middle.pop());
        assert_eq!(
            Some("This room is already in unique-chat mode.".to_string()),
            params.message
        );
        assert_eq!(remain, "");
    }

    #[test]
    fn base_paramsspace_empty() {
        let msg = " ";
        let (remain, params) = params_parse(msg).unwrap();

        assert_eq!(1, params.middle.len());
        assert_eq!(None, params.message);
        assert_eq!(remain, "");
    }

    #[test]
    fn base_params_space_empty_rn() {
        let msg = " \r\n";
        let (remain, params) = params_parse(msg).unwrap();

        assert_eq!(1, params.middle.len());
        assert_eq!(None, params.message);
        assert_eq!(remain, "");
    }

    #[test]
    fn base_paramse_empty() {
        let msg = "";
        let (remain, params) = params_parse(msg).unwrap();

        assert_eq!(0, params.middle.len());
        assert_eq!("", remain);
    }

    #[test]
    fn base_params_empty_rn() {
        let msg = "\r\n";
        let (remain, params) = params_parse(msg).unwrap();

        assert_eq!(0, params.middle.len());
        assert_eq!("", remain);
    }

    #[test]
    fn base_params_channel_message_base() {
        let msg = " #ronni :Kappa Keepo Kappa";
        let (remain, mut params) = params_parse(msg).unwrap();

        assert_eq!(Some("#ronni".to_string()), params.middle.pop(),);
        assert_eq!(Some("Kappa Keepo Kappa".to_string()), params.message,);
        assert_eq!(remain, "");
    }
    #[test]
    fn base_params_channel_message_rn() {
        let msg = " #ronni :Kappa Keepo Kappa\r\n";
        let (remain, mut params) = params_parse(msg).unwrap();

        assert_eq!(Some("#ronni".to_string()), params.middle.pop());
        assert_eq!(Some("Kappa Keepo Kappa".to_string()), params.message,);
        assert_eq!(remain, "");
    }
}
