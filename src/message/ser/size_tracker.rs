use crate::message::ser::{MessageSerializer, SerializeParams, SerializeSource, SerializeTags};
use crate::{Commands, IRCError};
use crate::{AT, BANG, COLON, EQ, SEMICOLON, SPACE};

pub struct SizeTracker {
    count: usize,
}

impl SizeTracker {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { count: 0 }
    }

    pub fn total(&self) -> usize {
        self.count
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
    pub fn put_str(&mut self, s: &str) {
        self.count += s.len();
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
        self.put_u8(AT);
        SizeTagsTracker {
            tracker: self,
            first: true,
        }
    }

    fn source(&mut self, name: &str) -> Result<Self::Source<'_>, IRCError> {
        self.put_u8(COLON);
        self.put_slice(name.as_bytes());
        Ok(SizeSourceTracker { tracker: self })
    }

    fn command(&mut self, command: Commands) {
        self.put_slice(command.as_bytes());
    }

    fn params(&mut self) -> Self::Params<'_> {
        SizeParamsTracker { tracker: self }
    }

    fn trailing(&mut self, value: &str) -> Result<(), IRCError> {
        self.put_u8(SPACE);
        self.put_u8(COLON);
        self.put_slice(value.as_bytes());

        Ok(())
    }

    fn end(&mut self) {
        self.put_slice(b"\r\n");
    }
}

pub struct SizeTagsTracker<'a> {
    tracker: &'a mut SizeTracker,
    first: bool,
}

impl SerializeTags for SizeTagsTracker<'_> {
    fn tag(&mut self, key: &str, value: Option<&str>) -> Result<(), IRCError> {
        if !self.first {
            self.tracker.put_u8(SEMICOLON);
        }
        self.first = false;

        self.tracker.put_str(key);
        self.tracker.put_u8(EQ);

        if let Some(val) = value {
            self.tracker.put_str(val);
        }

        Ok(())
    }

    fn flag(&mut self, key: &str) -> Result<(), IRCError> {
        if !self.first {
            self.tracker.put_u8(SEMICOLON);
        }

        self.first = false;

        self.tracker.put_slice(key.as_bytes());
        Ok(())
    }

    fn end(self) {
        self.tracker.put_u8(SPACE);
    }
}

pub struct SizeSourceTracker<'a> {
    tracker: &'a mut SizeTracker,
}

impl SerializeSource for SizeSourceTracker<'_> {
    fn user(&mut self, user: &str) -> Result<(), IRCError> {
        self.tracker.put_u8(BANG);
        self.tracker.put_str(user);
        Ok(())
    }

    fn host(&mut self, host: &str) -> Result<(), IRCError> {
        self.tracker.put_u8(AT);
        self.tracker.put_str(host);
        Ok(())
    }

    fn end(self) {
        self.tracker.put_u8(SPACE);
    }
}

pub struct SizeParamsTracker<'a> {
    tracker: &'a mut SizeTracker,
}

impl SerializeParams for SizeParamsTracker<'_> {
    fn add(&mut self, value: &str) -> Result<(), IRCError> {
        self.tracker.put_u8(SPACE);
        self.tracker.put_str(value);
        Ok(())
    }

    fn end(self) {}
}
