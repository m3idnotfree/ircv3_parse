mod commands;
mod params;
mod source;
mod tags;

pub use commands::{CapSubCommands, Commands};
pub use params::{Middles, Params};
pub use source::Source;
pub use tags::{TagValue, Tags};

use crate::scanner::Scanner;

pub struct Message<'a> {
    input: &'a str,
    scanner: Scanner,
}

impl<'a> Message<'a> {
    #[inline]
    pub fn new(input: &'a str, scanner: Scanner) -> Self {
        Self { input, scanner }
    }

    #[inline]
    pub fn tags(&self) -> Option<Tags<'a>> {
        if self.scanner.has_tags() {
            Some(Tags::new(self.scanner.tags_span.extract(self.input)))
        } else {
            None
        }
    }

    #[inline]
    pub fn source(&self) -> Option<Source<'a>> {
        if self.scanner.has_source() {
            Some(Source::parse(self.scanner.source_span.extract(self.input)))
        } else {
            None
        }
    }

    #[inline]
    pub fn command(&self) -> Commands {
        Commands::from(self.scanner.command_span.extract(self.input))
    }

    #[inline]
    pub fn params(&self) -> Params<'a> {
        if self.scanner.has_trailing() {
            let start_pos = self.scanner.params_span.start as usize;
            let end_pos = self.scanner.trailing_span.end as usize;
            let input = &self.input[start_pos..end_pos];

            Params::new(
                input,
                self.scanner.params_span.extract(self.input),
                Some(self.scanner.trailing_span.extract(self.input)),
            )
        } else {
            let input = self.scanner.params_span.extract(self.input);
            Params::new(input, input, None)
        }
    }
}
