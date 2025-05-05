use std::collections::VecDeque;

use nom::{combinator::opt, sequence::tuple, Parser};

use crate::{
    command::command_parse, params::params_parse, source::source_parse, IRCv3Message,
    IRCv3MessageBase, IRCv3Params, ParamsParse,
};

pub fn parse<T>(input: &str, params: T) -> IRCv3Message<T>
where
    T: ParamsParse,
{
    let (_, base) = parse_base(input);

    let parse_middle = base.params_middle_parse(&params);

    IRCv3Message {
        tags: base.tags,
        source: base.source,
        command: base.command,
        params: parse_middle,
    }
}

fn parse_base(msg: &str) -> (&str, IRCv3MessageBase) {
    let (msg, (tags, source, command, params)) = (
        opt(ircv3_tags::try_parse),
        source_parse,
        command_parse,
        params_parse,
    )
        .parse(msg)
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

// fn parse_inner<'a, T>(
//     params: T,
//     input: &str,
//     mut result: VecDeque<IRCv3Message<'a, T>>,
// ) -> VecDeque<IRCv3Message<'a, T>>
// where
//     T: ParamsParse,
// {
//     if input.is_empty() {
//         result
//     } else {
//         let (msg, base) = parse_base(input);
//
//         let parse_middle = base.params_middle_parse(&params);
//
//         result.push_back(IRCv3Message {
//             tags: base.tags,
//             source: base.source,
//             command: base.command,
//             params: parse_middle,
//         });
//
//         parse_inner(params, msg, result)
//     }
// }

// #[derive(Debug)]
// pub struct IRCv3Builder<T>
// where
//     T: ParamsParse,
// {
//     pub params_parse: T,
// }

// impl<T> IRCv3Builder<T>
// where
//     T: ParamsParse,
// {
//     pub fn new(params_parse: T) -> Self {
//         Self { params_parse }
//     }

// pub fn parse(&self, msg: &str) -> IRCv3Message<T> {
//     let (_, base) = Self::parse_base(msg);
//
//     let parse_middle = base.params_middle_parse(&self.params_parse);
//
//     IRCv3Message {
//         tags: base.tags,
//         source: base.source,
//         command: base.command,
//         params: parse_middle,
//     }
// }

// pub fn parse_vecdeque(&self, msg: &str) -> VecDeque<IRCv3Message<T>> {
//     Self::parse_inner(self, msg, VecDeque::new())
// }
//
// fn parse_inner(
//     &self,
//     msg: &str,
//     mut result: VecDeque<IRCv3Message<T>>,
// ) -> VecDeque<IRCv3Message<T>> {
//     if msg.is_empty() {
//         result
//     } else {
//         let (msg, base) = Self::parse_base(msg);
//
//         let parse_middle = base.params_middle_parse(&self.params_parse);
//
//         result.push_back(IRCv3Message {
//             tags: base.tags,
//             source: base.source,
//             command: base.command,
//             params: parse_middle,
//         });
//
//         Self::parse_inner(self, msg, result)
//     }
// }

// fn parse_base(msg: &str) -> (&str, IRCv3MessageBase) {
//     let (msg, (tags, source, command, params)) = (
//         opt(ircv3_tags::try_parse),
//         source_parse,
//         command_parse,
//         params_parse,
//     )
//         .parse(msg)
//         .unwrap();
//
//     (
//         msg,
//         IRCv3MessageBase {
//             tags,
//             source,
//             command: command.to_string(),
//             params,
//         },
//     )
// }
// }

// impl Default for IRCv3Builder<IRCv3Params> {
//     fn default() -> Self {
//         Self {
//             params_parse: IRCv3Params::default(),
//         }
//     }
// }
