use crate::message::ser::{MessageSerializer, SerializeParams, SerializeSource, SerializeTags};
use crate::{Commands, IRCError};
use crate::{AT, BANG, COLON, EQ, SEMICOLON, SPACE};

pub struct SizeTracker {
    count: usize,
    has_tags: bool,
    has_command: bool,
    has_trailing: bool,
    needs_space: bool,
}

impl SizeTracker {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            count: 0,
            has_tags: false,
            has_command: false,
            has_trailing: false,
            needs_space: false,
        }
    }

    pub fn total(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    #[inline]
    pub fn put_u8(&mut self, _bytes: u8) {
        self.count += 1;
    }

    #[inline]
    pub fn put_slice(&mut self, bytes: &[u8]) {
        self.count += bytes.len();
    }

    #[inline]
    fn add_space_if_needed(&mut self) {
        if self.needs_space {
            self.put_u8(SPACE);
            self.needs_space = false;
        }
    }
}

impl MessageSerializer for SizeTracker {
    type Tags<'a>
        = SizeTagsTracker<'a>
    where
        Self: 'a;

    type Source<'a>
        = SizeSourceTracker<'a>
    where
        Self: 'a;

    type Params<'a>
        = SizeParamsTracker<'a>
    where
        Self: 'a;

    fn tags(&mut self) -> Self::Tags<'_> {
        SizeTagsTracker { tracker: self }
    }

    fn source(&mut self, name: &str) -> Result<Self::Source<'_>, IRCError> {
        self.add_space_if_needed();
        self.put_u8(COLON);
        self.put_slice(name.as_bytes());
        Ok(SizeSourceTracker {
            tracker: self,
            ended: false,
        })
    }

    fn command(&mut self, command: Commands) {
        self.add_space_if_needed();
        self.has_command = true;
        self.put_slice(command.as_bytes());
    }

    fn params(&mut self) -> Self::Params<'_> {
        SizeParamsTracker { tracker: self }
    }

    fn trailing(&mut self, value: &str) -> Result<(), IRCError> {
        if !self.has_trailing {
            self.put_u8(SPACE);
            self.put_u8(COLON);
            self.has_trailing = true;
        }

        self.put_slice(value.as_bytes());
        Ok(())
    }

    fn end(&mut self) -> Result<(), IRCError> {
        self.put_slice(b"\r\n");
        Ok(())
    }
}

pub struct SizeTagsTracker<'a> {
    tracker: &'a mut SizeTracker,
}

impl SerializeTags for SizeTagsTracker<'_> {
    fn tag(&mut self, key: &str, value: Option<&str>) -> Result<(), IRCError> {
        if !self.tracker.has_tags {
            self.tracker.put_u8(AT);
            self.tracker.has_tags = true;
        } else {
            self.tracker.put_u8(SEMICOLON);
        }

        self.tracker.put_slice(key.as_bytes());
        self.tracker.put_u8(EQ);

        if let Some(val) = value {
            self.tracker.put_slice(val.as_bytes());
        }

        Ok(())
    }

    fn flag(&mut self, key: &str) -> Result<(), IRCError> {
        if !self.tracker.has_tags {
            self.tracker.put_u8(AT);
            self.tracker.has_tags = true;
        } else {
            self.tracker.put_u8(SEMICOLON);
        }

        self.tracker.put_slice(key.as_bytes());
        Ok(())
    }

    fn end(self) {
        if !self.tracker.is_empty() {
            self.tracker.needs_space = true;
        }
    }
}

impl Drop for SizeTagsTracker<'_> {
    fn drop(&mut self) {
        if !self.tracker.is_empty() {
            self.tracker.needs_space = true;
        }
    }
}

pub struct SizeSourceTracker<'a> {
    tracker: &'a mut SizeTracker,
    ended: bool,
}

impl SerializeSource for SizeSourceTracker<'_> {
    fn user(&mut self, user: &str) -> Result<(), IRCError> {
        self.tracker.put_u8(BANG);
        self.tracker.put_slice(user.as_bytes());
        Ok(())
    }

    fn host(&mut self, host: &str) -> Result<(), IRCError> {
        self.tracker.put_u8(AT);
        self.tracker.put_slice(host.as_bytes());
        Ok(())
    }

    fn end(mut self) {
        if !self.ended {
            self.tracker.needs_space = true;
            self.ended = true;
        }
    }
}

impl Drop for SizeSourceTracker<'_> {
    fn drop(&mut self) {
        if !self.ended {
            self.tracker.needs_space = true;
        }
    }
}

pub struct SizeParamsTracker<'a> {
    tracker: &'a mut SizeTracker,
}

impl SerializeParams for SizeParamsTracker<'_> {
    fn push(&mut self, value: &str) -> Result<(), IRCError> {
        self.tracker.put_u8(SPACE);
        self.tracker.put_slice(value.as_bytes());
        Ok(())
    }

    fn extend<I, S>(&mut self, params: I) -> Result<(), IRCError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for param in params {
            self.tracker.put_u8(SPACE);
            self.tracker.put_slice(param.as_ref().as_bytes());
        }
        Ok(())
    }

    fn end(self) {}
}
