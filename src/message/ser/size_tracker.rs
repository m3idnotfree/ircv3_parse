use crate::error::SourceError;
use crate::message::ser::{MessageSerializer, SerializeParams, SerializeSource, SerializeTags};
use crate::{validators, Commands, IRCError};
use crate::{COLON, SPACE};

pub struct SizeTracker {
    count: usize,
    tags: SizeTagsTracker,
    tags_flushed: bool,
    source: SizeSourceTracker,
    source_flushed: bool,
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
            source: SizeSourceTracker::new(),
            source_flushed: false,
            has_command: false,
            has_trailing: false,
            needs_space: false,
        }
    }

    pub fn total(&mut self) -> usize {
        self.flush_tags();
        self.flush_source();
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
    fn flush_source(&mut self) {
        if !self.source_flushed {
            let size = self.source.byte_len();
            if size > 0 {
                self.count += size;
                self.needs_space = true;
            }
            self.source_flushed = true;
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
    type Source = SizeSourceTracker;

    type Params<'a>
        = SizeParamsTracker<'a>
    where
        Self: 'a;

    fn tags(&mut self) -> &mut Self::Tags {
        &mut self.tags
    }

    fn source(&mut self) -> &mut Self::Source {
        self.flush_tags();
        self.add_space_if_needed();
        &mut self.source
    }

    fn command(&mut self, command: Commands) {
        self.flush_tags();
        self.add_space_if_needed();
        self.flush_source();
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
        self.add_space_if_needed();
        self.flush_source();
        self.add_space_if_needed();
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

pub struct SizeSourceTracker {
    count: usize,
    has_name: bool,
}

impl SizeSourceTracker {
    pub fn new() -> Self {
        Self {
            count: 0,
            has_name: false,
        }
    }

    fn byte_len(&self) -> usize {
        self.count
    }
}

impl SerializeSource for SizeSourceTracker {
    fn name(&mut self, name: &str) -> Result<(), IRCError> {
        if self.has_name {
            return Err(IRCError::Source(SourceError::DublicateComponent {
                component: "name",
            }));
        }

        // : + name
        self.count += 1 + name.len();

        Ok(())
    }

    fn user(&mut self, user: &str) -> Result<(), IRCError> {
        // ! + user
        self.count += 1 + user.len();
        Ok(())
    }

    fn host(&mut self, host: &str) -> Result<(), IRCError> {
        // @ + host
        self.count += 1 + host.len();
        Ok(())
    }

    fn end(&self) {}
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
