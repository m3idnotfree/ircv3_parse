use bytes::{BufMut, Bytes, BytesMut};

use crate::{
    components::Commands,
    error::{ParamError, TagError},
    validators, AT, COLON, SEMICOLON, SPACE,
};

#[derive(Debug)]
pub struct MessageBuilder {
    buffer: BytesMut,
}

impl Default for MessageBuilder {
    fn default() -> Self {
        Self {
            buffer: BytesMut::with_capacity(512),
        }
    }
}

impl MessageBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: BytesMut::with_capacity(capacity),
        }
    }

    pub fn tags<F>(mut self, f: F) -> Result<Self, BuilderError>
    where
        F: FnOnce(TagBuilder<'_>) -> Result<TagBuilder<'_>, TagError>,
    {
        let tag_builder = f(TagBuilder::with_buffer(&mut self.buffer))?;
        tag_builder.finish();

        Ok(self)
    }

    pub fn source<F>(mut self, name: &str, f: F) -> Self
    where
        F: FnOnce(SourceBuilder<'_>) -> SourceBuilder<'_>,
    {
        self.buffer.put_u8(COLON);

        let source_builder = f(SourceBuilder::with_buffer(&mut self.buffer).name(name));
        source_builder.finish();

        self.buffer.put_u8(SPACE);
        self
    }

    pub fn command(mut self, cmd: Commands) -> Self {
        self.buffer.put_slice(cmd.as_str().as_bytes());
        self
    }

    pub fn params<'a, I>(mut self, i: I) -> Result<Self, BuilderError>
    where
        I: IntoIterator<Item = &'a str>,
    {
        self.buffer.put_u8(SPACE);

        for item in i.into_iter() {
            validators::params(item)?;
            self.buffer.extend_from_slice(item.as_bytes());
            self.buffer.put_u8(SPACE);
        }

        Ok(self)
    }

    pub fn trailing(mut self, trailing: &str) -> Self {
        if !trailing.is_empty() {
            self.buffer.put_slice(&[SPACE, COLON]);
            self.buffer.put_slice(trailing.as_bytes());
        }

        self
    }

    pub fn to_bytes(mut self) -> Bytes {
        self.buffer.put_slice(b"\r\n");
        self.buffer.freeze()
    }

    pub fn reset(&mut self) {
        self.buffer.clear();
    }
}

#[derive(Debug)]
pub struct SourceBuilder<'a> {
    buffer: &'a mut BytesMut,
}

impl<'a> SourceBuilder<'a> {
    pub fn with_buffer(buffer: &'a mut BytesMut) -> Self {
        Self { buffer }
    }

    fn name(self, name: &str) -> Self {
        self.buffer.put_slice(name.as_bytes());
        self
    }

    pub fn user(self, user: &str) -> Self {
        self.buffer.put_u8(b'!');
        self.buffer.put_slice(user.as_bytes());
        self
    }

    pub fn host(self, host: &str) -> Self {
        self.buffer.put_u8(AT);
        self.buffer.put_slice(host.as_bytes());
        self
    }

    pub fn finish(self) {}
}
#[derive(Debug)]
pub struct TagBuilder<'a> {
    buffer: &'a mut BytesMut,
    has_tags: bool,
}

impl<'a> TagBuilder<'a> {
    pub fn with_buffer(buffer: &'a mut BytesMut) -> Self {
        TagBuilder {
            buffer,
            has_tags: false,
        }
    }

    pub fn tag(mut self, key: &str, value: Option<&str>) -> Result<Self, TagError> {
        if self.has_tags {
            self.buffer.put_u8(SEMICOLON);
        } else {
            self.buffer.put_u8(AT);
        }

        validators::tag_value(key)?;
        self.buffer.put_slice(key.as_bytes());

        if let Some(val) = value {
            validators::tag_value(val)?;
            self.buffer.put_u8(b'=');
            self.buffer.put_slice(val.as_bytes());
        }

        self.has_tags = true;
        Ok(self)
    }

    pub fn flag(self, key: &str) -> Result<Self, TagError> {
        self.tag(key, None)
    }

    pub fn extend<I>(mut self, tags: I) -> Result<Self, TagError>
    where
        I: IntoIterator<Item = (&'a str, Option<&'a str>)>,
    {
        for (key, value) in tags {
            if self.has_tags {
                self.buffer.put_u8(SEMICOLON);
            } else {
                self.buffer.put_u8(AT);
                self.has_tags = true;
            }

            validators::tag_key(key)?;
            self.buffer.put_slice(key.as_bytes());

            if let Some(val) = value {
                validators::tag_value(val)?;
                self.buffer.put_u8(b'=');
                self.buffer.put_slice(val.as_bytes());
            }

            self.has_tags = true;
        }

        Ok(self)
    }

    pub fn finish(self) {
        if self.has_tags {
            self.buffer.put_u8(SPACE);
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BuilderError {
    #[error(transparent)]
    Tag(#[from] TagError),
    #[error(transparent)]
    Param(#[from] ParamError),
}

#[cfg(test)]
mod tests {
    use crate::components::Commands;

    use super::MessageBuilder;

    #[test]
    fn base() {
        let message = MessageBuilder::new()
            .tags(|tags| {
                tags.tag("tag1", Some("value1"))?
                    .tag("tag2", Some(""))?
                    .flag("flag")
            })
            .unwrap()
            .source("name", |source| source.host("example.com"))
            .command(Commands::PRIVMSG)
            .trailing("");

        let actual = message.to_bytes();

        assert_eq!(
            "@tag1=value1;tag2=;flag :name@example.com PRIVMSG\r\n",
            actual
        );
    }
}
