use std::fmt;

use crate::{error::HostnameError, HYPEN};

pub struct RFC1123 {
    max_len: usize,
    max_segment_len: usize,
    max_depth: usize,
}

impl Default for RFC1123 {
    fn default() -> Self {
        Self {
            max_len: 253,
            max_segment_len: 63,
            max_depth: 10,
        }
    }
}

impl RFC1123 {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn validate(&self, input: &str) -> Result<(), HostnameError> {
        if input.is_empty() {
            return Err(HostnameError::Empty);
        }

        if input.len() > self.max_len {
            return Err(HostnameError::LabelTooLong {
                max: self.max_len,
                actual: input.len(),
            });
        }

        for (i, segment) in input.split('.').enumerate() {
            if self.max_depth < i {
                return Err(HostnameError::TooManyLabels {
                    max: self.max_depth,
                    actual: i,
                });
            }
            self.validate_segment(segment)?;
        }

        Ok(())
    }

    fn validate_segment(&self, segment: &str) -> Result<(), HostnameError> {
        if segment.is_empty() {
            return Err(HostnameError::Empty);
        }

        if segment.len() > self.max_segment_len {
            return Err(HostnameError::LabelTooLong {
                max: self.max_segment_len,
                actual: segment.len(),
            });
        }

        let bytes = segment.as_bytes();

        if !self.start_with(bytes[0]) {
            return Err(HostnameError::InvalidFirstChar {
                char: bytes[0] as char,
            });
        }

        if bytes.len() > 1 && self.end_with(bytes[bytes.len() - 1]) {
            return Err(HostnameError::InvalidLastChar {
                char: bytes[bytes.len() - 1] as char,
            });
        }

        for &c in bytes {
            if !self.validate_char(c) {
                return Err(HostnameError::InvalidChar { char: c as char });
            }
        }

        Ok(())
    }

    #[inline]
    fn start_with(&self, c: u8) -> bool {
        c.is_ascii_alphanumeric()
    }

    #[inline]
    fn end_with(&self, c: u8) -> bool {
        c == HYPEN
    }

    #[inline]
    fn validate_char(&self, c: u8) -> bool {
        c.is_ascii_alphanumeric() || c == HYPEN
    }
}

impl fmt::Display for RFC1123 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RFC1123")
    }
}
