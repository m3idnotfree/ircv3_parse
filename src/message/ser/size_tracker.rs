use crate::message::ser::{MessageSerializer, SerializeParams, SerializeSource, SerializeTags};
use crate::{Commands, IRCError};
use crate::{COLON, SPACE};

pub struct SizeTracker {
    count: usize,
    tags: SizeTagsTracker,
    source: SizeSourceTracker,
    params: SizeParamsTracker,
    has_trailing: bool,
    finished: bool,
}

impl SizeTracker {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            count: 0,
            tags: SizeTagsTracker::new(),
            source: SizeSourceTracker::new(),
            params: SizeParamsTracker::new(),
            has_trailing: false,
            finished: false,
        }
    }

    pub fn total(&mut self) -> usize {
        self.flush_tags();
        self.flush_source();
        self.flush_params();

        if self.finished {
            self.put_slice(b"\r\n");
        }

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
        let size = self.tags.byte_len();
        if size > 0 {
            // tags + ' '
            self.count += size + 1;
        }
    }

    #[inline]
    fn flush_source(&mut self) {
        let size = self.source.byte_len();
        if size > 0 {
            // source + ' '
            self.count += size + 1;
        }
    }

    #[inline]
    fn flush_params(&mut self) {
        self.count += self.params.byte_len();
    }
}

impl MessageSerializer for SizeTracker {
    type Tags = SizeTagsTracker;
    type Source = SizeSourceTracker;
    type Params = SizeParamsTracker;

    fn tags(&mut self) -> &mut Self::Tags {
        &mut self.tags
    }

    fn source(&mut self) -> &mut Self::Source {
        &mut self.source
    }

    fn command(&mut self, command: Commands) {
        self.put_slice(command.as_bytes());
    }

    fn params(&mut self) -> &mut Self::Params {
        &mut self.params
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
        self.finished = true;
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
        let val_len = value.map_or(0, str::len);

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
}

impl SizeSourceTracker {
    pub fn new() -> Self {
        Self { count: 0 }
    }

    fn byte_len(&self) -> usize {
        self.count
    }
}

impl SerializeSource for SizeSourceTracker {
    fn name(&mut self, name: &str) -> Result<(), IRCError> {
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

pub struct SizeParamsTracker {
    count: usize,
}

impl SizeParamsTracker {
    pub fn new() -> Self {
        Self { count: 0 }
    }

    pub fn byte_len(&self) -> usize {
        self.count
    }
}

impl SerializeParams for SizeParamsTracker {
    fn push(&mut self, value: &str) -> Result<(), IRCError> {
        // ' ' + value
        self.count += 1 + value.len();
        Ok(())
    }

    fn extend<I, S>(&mut self, params: I) -> Result<(), IRCError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        params.into_iter().for_each(|param| {
            // ' ' + param
            self.count += 1 + param.as_ref().len();
        });
        Ok(())
    }

    fn end(&self) {}
}
