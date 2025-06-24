use crate::{
    error::{CommandError, IRCError, ParamError},
    AT, COLON, CR, LF, SPACE,
};

const MEMCHR_THRESHOLD: usize = 12;

#[derive(Debug)]
pub struct Scanner {
    has_tags: bool,
    has_source: bool,
    has_params: bool,
    has_trailing: bool,
    pub tags_span: ByteSpan,
    pub source_span: ByteSpan,
    pub command_span: ByteSpan,
    pub params_span: ByteSpan,
    pub trailing_span: ByteSpan,
}

impl Scanner {
    pub fn new(input: &str) -> Result<Self, IRCError> {
        let bytes = input.as_bytes();

        if bytes.is_empty() {
            return Err(IRCError::EmptyInput);
        }

        let mut scanner = Self {
            has_tags: false,
            has_source: false,
            has_params: false,
            has_trailing: false,
            tags_span: ByteSpan::empty(),
            source_span: ByteSpan::empty(),
            command_span: ByteSpan::empty(),
            params_span: ByteSpan::empty(),
            trailing_span: ByteSpan::empty(),
        };

        let mut pos = 0;

        scanner.scan_tags(bytes, &mut pos)?;
        scanner.scan_source(bytes, &mut pos)?;
        scanner.scan_command(bytes, &mut pos)?;

        let line_end = Self::is_message_end(bytes, pos);
        if line_end {
            return Ok(scanner);
        }

        scanner.scan_parameters(bytes, pos)?;

        Ok(scanner)
    }

    #[inline]
    fn scan_tags(&mut self, bytes: &[u8], pos: &mut usize) -> Result<(), IRCError> {
        if bytes[*pos] != AT {
            return Ok(());
        }

        *pos += 1; // skip '@'
        let start = *pos;

        *pos +=
            find_byte(SPACE, &bytes[*pos..]).ok_or(IRCError::MissingSpace { component: "TAG" })?;

        self.tags_span = ByteSpan::new(start, *pos);
        self.has_tags = true;
        *pos += 1; // skip space
        Ok(())
    }

    #[inline]
    fn scan_source(&mut self, bytes: &[u8], pos: &mut usize) -> Result<(), IRCError> {
        if *pos >= bytes.len() || bytes[*pos] != COLON {
            return Ok(());
        }

        *pos += 1; // skip ':'
        let start = *pos;

        *pos += find_byte(SPACE, &bytes[*pos..]).ok_or(IRCError::MissingSpace {
            component: "SOURCE",
        })?;

        self.source_span = ByteSpan::new(start, *pos);
        self.has_source = true;
        *pos += 1; // skip space

        Ok(())
    }

    #[inline]
    fn scan_command(&mut self, bytes: &[u8], pos: &mut usize) -> Result<(), IRCError> {
        if *pos >= bytes.len() {
            return Err(IRCError::Command(CommandError::Empty));
        }

        let start = *pos;
        let first_byte = bytes[*pos];

        if !first_byte.is_ascii_alphanumeric() {
            return Err(IRCError::Command(CommandError::InvalidFirstChar {
                char: first_byte as char,
            }));
        }

        if first_byte.is_ascii_digit() {
            *pos += 1;
            let mut digit_count = 1;

            while *pos < bytes.len() && bytes[*pos].is_ascii_digit() {
                digit_count += 1;
                *pos += 1;
            }
            if digit_count != 3 {
                return Err(IRCError::Command(CommandError::WrongDigitCount {
                    actual: digit_count,
                }));
            }
        } else {
            while *pos < bytes.len() && bytes[*pos].is_ascii_alphabetic() {
                *pos += 1;
            }
        }

        self.command_span = ByteSpan::new(start, *pos);

        Ok(())
    }

    #[inline]
    fn scan_parameters(&mut self, bytes: &[u8], mut pos: usize) -> Result<(), IRCError> {
        if bytes[pos] != SPACE {
            return Err(IRCError::MissingSpace { component: "PARAM" });
        }

        pos += 1; // skip space

        if pos < bytes.len() && bytes[pos] == COLON {
            return self.scan_trailing(bytes, pos + 1);
        }

        if let Some(space_colon_pos) = find_space_colon_pattern(&bytes[pos..]) {
            let start = pos + space_colon_pos;

            if start == pos {
                return Err(IRCError::Param(ParamError::EmptyMiddle));
            }

            self.params_span = ByteSpan::new(pos, start);
            self.has_params = true;

            let trailing_start = start + 2; // skip " :"
            self.scan_trailing(bytes, trailing_start)?;
        } else {
            let end_pos = pos + find_line_ending(&bytes[pos..]).unwrap_or(bytes[pos..].len());

            if end_pos == pos {
                return Err(IRCError::Param(ParamError::EmptyMiddle));
            }

            self.params_span = ByteSpan::new(pos, end_pos);
            self.has_params = true;
        }
        Ok(())
    }

    #[inline]
    fn scan_trailing(&mut self, bytes: &[u8], pos: usize) -> Result<(), IRCError> {
        let end = pos + find_line_ending(&bytes[pos..]).unwrap_or(bytes[pos..].len());

        self.trailing_span = ByteSpan::new(pos, end);
        self.has_trailing = true;
        Ok(())
    }

    #[inline]
    fn is_message_end(bytes: &[u8], pos: usize) -> bool {
        pos >= bytes.len() || matches!(bytes[pos], CR | LF)
    }

    #[inline]
    pub fn has_tags(&self) -> bool {
        self.has_tags
    }
    #[inline]
    pub fn has_source(&self) -> bool {
        self.has_source
    }
    #[inline]
    pub fn has_params(&self) -> bool {
        self.has_params
    }
    #[inline]
    pub fn has_trailing(&self) -> bool {
        self.has_trailing
    }
}

#[inline]
fn find_byte(needle: u8, haystack: &[u8]) -> Option<usize> {
    if haystack.len() < MEMCHR_THRESHOLD {
        haystack.iter().position(|&b| b == needle)
    } else {
        memchr::memchr(needle, haystack)
    }
}

#[inline]
fn find_line_ending(bytes: &[u8]) -> Option<usize> {
    if bytes.len() < MEMCHR_THRESHOLD {
        bytes.iter().position(|&b| matches!(b, CR | LF))
    } else {
        memchr::memchr2(CR, LF, bytes)
    }
}

#[inline]
fn find_space_colon_pattern(bytes: &[u8]) -> Option<usize> {
    if bytes.len() < 2 {
        return None;
    }

    if bytes.len() < MEMCHR_THRESHOLD {
        for i in 0..bytes.len().saturating_sub(1) {
            if bytes[i] == SPACE && bytes[i + 1] == COLON {
                return Some(i);
            }
        }
        None
    } else {
        memchr::memchr(COLON, bytes).and_then(|colon_pos| {
            if colon_pos > 0 && bytes[colon_pos - 1] == SPACE {
                Some(colon_pos - 1)
            } else {
                None
            }
        })
    }
}

#[derive(Debug)]
pub struct ByteSpan {
    pub start: u32,
    pub end: u32,
}

impl Default for ByteSpan {
    fn default() -> Self {
        Self::empty()
    }
}

impl ByteSpan {
    #[inline]
    pub(crate) fn new(start: usize, end: usize) -> Self {
        Self {
            start: start as u32,
            end: end as u32,
        }
    }

    #[inline]
    pub(crate) fn empty() -> Self {
        Self { start: 0, end: 0 }
    }

    #[inline]
    pub(crate) fn extract<'a>(&self, input: &'a str) -> &'a str {
        if self.start == self.end {
            ""
        } else {
            &input[self.start as usize..self.end as usize]
        }
    }

    #[inline]
    pub(crate) fn extract_bytes<'a>(&self, input: &'a [u8]) -> &'a [u8] {
        if self.start == self.end {
            &[]
        } else {
            &input[self.start as usize..self.end as usize]
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    #[inline]
    pub fn len(&self) -> usize {
        (self.end - self.start) as usize
    }
}
