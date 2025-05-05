use ircv3_tags::IRCv3Tags;

use crate::{params::IRCv3ParamsBase, IRCv3Source, ParamsParse};

#[derive(Debug)]
pub struct IRCv3Message<'a, T> {
    pub tags: Option<IRCv3Tags<'a>>,
    pub source: Option<IRCv3Source>,
    pub command: String,
    pub params: T,
}

#[derive(Debug)]
pub struct IRCv3MessageBase<'a> {
    pub tags: Option<IRCv3Tags<'a>>,
    pub source: Option<IRCv3Source>,
    pub command: String,
    pub params: IRCv3ParamsBase,
}

impl<'a> IRCv3MessageBase<'a> {
    pub fn params_middle_parse<F>(&self, f: &F) -> F
    where
        F: ParamsParse,
    {
        f.parse(self.command.as_str(), self.params.clone())
    }
}
