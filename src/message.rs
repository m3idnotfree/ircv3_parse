use ircv3_tags::IRCv3Tags;

use crate::{params::IRCv3ParamsBase, IRCv3Source, ParamsParse};

#[derive(Debug)]
pub struct IRCv3Message<T> {
    pub tags: Option<IRCv3Tags>,
    pub source: Option<IRCv3Source>,
    pub command: String,
    pub params: T,
}

#[derive(Debug)]
pub struct IRCv3MessageBase {
    pub tags: Option<IRCv3Tags>,
    pub source: Option<IRCv3Source>,
    pub command: String,
    pub params: IRCv3ParamsBase,
}

impl IRCv3MessageBase {
    pub fn params_middle_parse<F>(&self, f: &F) -> F
    where
        F: ParamsParse,
    {
        f.parse(self.command.as_str(), self.params.clone())
    }
}
