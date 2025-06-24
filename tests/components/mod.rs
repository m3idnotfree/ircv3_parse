use std::fmt;

use commands::command_strategy;
use params::{params_strategy, ParamsType};
use proptest::{
    option,
    prelude::{Arbitrary, BoxedStrategy, Just, Strategy},
    prop_compose, prop_oneof,
};
use source::{source_strategy, SourceType};
use tags::{tags_strategy, TagsType};

pub mod commands;
pub mod escape;
pub mod host;
pub mod params;
pub mod source;
pub mod tags;

#[derive(Debug)]
pub struct TestMessage {
    pub tags: Option<TagsType>,
    pub source: Option<SourceType>,
    pub command: String,
    pub params: ParamsType,
    pub line_ending: String,
}

impl fmt::Display for TestMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(tag) = &self.tags {
            write!(f, "@{} ", tag)?;
        }

        if let Some(source) = &self.source {
            write!(f, ":{} ", source)?;
        }

        write!(f, "{}", self.command)?;

        let params_str = self.params.to_string();
        write!(f, "{}", params_str)?;

        write!(f, "{}", self.line_ending)
    }
}

impl Arbitrary for TestMessage {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        (
            option::of(tags_strategy()),
            option::of(source_strategy()),
            command_strategy(),
            params_strategy(),
            line_ending_strategy(),
        )
            .prop_map(|(tags, source, command, params, line_ending)| TestMessage {
                tags,
                source,
                command,
                params,
                line_ending,
            })
            .boxed()
    }
}

prop_compose! {
    pub fn line_ending_strategy()(
        line in prop_oneof![
            Just(""),
            Just("\r"),
            Just("\n"),
            Just("\r\n"),
    ]) -> String { line.to_string() }
}
