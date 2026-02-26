use crate::message::ser::{MessageSerializer, SerializeParams, SerializeSource, SerializeTags};
use crate::{validators, Commands, IRCError};
use crate::{AT, BANG, COLON, SPACE};

pub struct SizeTracker {
    count: usize,
    tags: SizeTagsTracker,
    tags_flushed: bool,
    has_command: bool,
    has_trailing: bool,
    needs_space: bool,
}

impl SizeTracker {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            count: 0,
            tags: SizeTagsTracker::new(),
            tags_flushed: false,
            has_command: false,
            has_trailing: false,
            needs_space: false,
        }
    }

    pub fn total(&mut self) -> usize {
        self.flush_tags();
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
    fn flush_tags(&mut self) {
        if !self.tags_flushed {
            let size = self.tags.byte_len();
            if size > 0 {
                self.count += size;
                self.needs_space = true;
            }
            self.tags_flushed = true;
        }
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
    type Tags = SizeTagsTracker;

    type Source<'a>
        = SizeSourceTracker<'a>
    where
        Self: 'a;

    type Params<'a>
        = SizeParamsTracker<'a>
    where
        Self: 'a;

    fn tags(&mut self) -> &mut Self::Tags {
        &mut self.tags
    }

    fn source(&mut self) -> Self::Source<'_> {
        self.flush_tags();
        self.add_space_if_needed();
        SizeSourceTracker {
            tracker: self,
            has_name: false,
            ended: false,
        }
    }

    fn command(&mut self, command: Commands) {
        self.flush_tags();
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
        self.flush_tags();
        self.put_slice(b"\r\n");
        Ok(())
    }
}

pub struct SizeTagsTracker {
    count: usize,
    has_tags: bool,
}

impl SizeTagsTracker {
    fn new() -> Self {
        Self {
            count: 0,
            has_tags: false,
        }
    }

    fn byte_len(&self) -> usize {
        self.count
    }
}

impl SerializeTags for SizeTagsTracker {
    fn tag(&mut self, key: &str, value: Option<&str>) -> Result<(), IRCError> {
        validators::tag_key(key)?;

        let val_len = value
            .map(|v| -> Result<usize, IRCError> {
                validators::tag_value(v)?;
                Ok(v.len())
            })
            .transpose()?
            .unwrap_or(0);

        if !self.has_tags {
            // @
            self.count += 1;
            self.has_tags = true;
        } else {
            // ;
            self.count += 1;
        }
        // key + = + value
        self.count += key.len() + 1 + val_len;

        Ok(())
    }

    fn flag(&mut self, key: &str) -> Result<(), IRCError> {
        validators::tag_key(key)?;

        if !self.has_tags {
            // @
            self.count += 1;
            self.has_tags = true;
        } else {
            // ;
            self.count += 1;
        }
        self.count += key.len();

        Ok(())
    }

    fn end(&self) {}
}

pub struct SizeSourceTracker<'a> {
    tracker: &'a mut SizeTracker,
    has_name: bool,
    ended: bool,
}

impl SerializeSource for SizeSourceTracker<'_> {
    fn name(&mut self, name: &str) -> Result<(), IRCError> {
        self.tracker.put_u8(COLON);
        self.tracker.put_slice(name.as_bytes());

        self.has_name = true;

        Ok(())
    }

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
        if self.has_name && !self.ended {
            self.tracker.needs_space = true;
            self.ended = true;
        }
    }
}

impl Drop for SizeSourceTracker<'_> {
    fn drop(&mut self) {
        if self.has_name && !self.ended {
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
