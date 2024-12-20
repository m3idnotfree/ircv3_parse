use std::collections::VecDeque;

use nom::sequence::tuple;

use crate::{
    command::command_parse, params::params_parse, source::source_parse, IRCv3Message,
    IRCv3MessageBase, IRCv3Params, ParamsParse,
};

#[derive(Debug)]
pub struct IRCv3Builder<T>
where
    T: ParamsParse,
{
    pub params_parse: T,
}

impl<T> IRCv3Builder<T>
where
    T: ParamsParse,
{
    pub fn new(params_parse: T) -> Self {
        Self { params_parse }
    }

    pub fn parse(&self, msg: &str) -> IRCv3Message<T> {
        let (_, base) = Self::parse_base(msg);

        let parse_middle = base.params_middle_parse(&self.params_parse);

        IRCv3Message {
            tags: base.tags,
            source: base.source,
            command: base.command,
            params: parse_middle,
        }
    }

    pub fn parse_vecdeque(&self, msg: &str) -> VecDeque<IRCv3Message<T>> {
        Self::parse_inner(self, msg, VecDeque::new())
    }

    fn parse_inner(
        &self,
        msg: &str,
        mut result: VecDeque<IRCv3Message<T>>,
    ) -> VecDeque<IRCv3Message<T>> {
        if msg.is_empty() {
            result
        } else {
            let (msg, base) = Self::parse_base(msg);

            let parse_middle = base.params_middle_parse(&self.params_parse);

            result.push_back(IRCv3Message {
                tags: base.tags,
                source: base.source,
                command: base.command,
                params: parse_middle,
            });

            Self::parse_inner(self, msg, result)
        }
    }

    fn parse_base(msg: &str) -> (&str, IRCv3MessageBase) {
        let (msg, (tags, source, command, params)) = tuple((
            ircv3_tags::parse_nom,
            source_parse,
            command_parse,
            params_parse,
        ))(msg)
        .unwrap();

        (
            msg,
            IRCv3MessageBase {
                tags,
                source,
                command: command.to_string(),
                params,
            },
        )
    }
}

impl Default for IRCv3Builder<IRCv3Params> {
    fn default() -> Self {
        Self {
            params_parse: IRCv3Params::default(),
        }
    }
}
