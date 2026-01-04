mod size_tracker;

pub use size_tracker::SizeTracker;

use bytes::{BufMut, Bytes, BytesMut};

use crate::{validators, Commands, IRCError};
use crate::{AT, BANG, COLON, EQ, SEMICOLON, SPACE};

/// Serialize the IRC message from custom data structure.
pub trait ToMessage {
    fn to_message<S: MessageSerializer>(&self, serialize: &mut S) -> Result<(), IRCError>;

    fn serialized_size(&self) -> usize {
        let mut tracker = SizeTracker::new();
        self.to_message(&mut tracker)
            .expect("size calculation should not fail");

        tracker.total()
    }

    fn to_bytes(&self) -> Result<Bytes, IRCError> {
        let mut serializer = IRCSerializer::with_capacity(self.serialized_size());
        self.to_message(&mut serializer)?;
        Ok(serializer.into_bytes())
    }
}

pub trait MessageSerializer: Sized {
    type Tags<'a>: SerializeTags
    where
        Self: 'a;

    type Source<'a>: SerializeSource
    where
        Self: 'a;

    type Params<'a>: SerializeParams
    where
        Self: 'a;

    fn tags(&mut self) -> Self::Tags<'_>;
    fn source(&mut self, name: &str) -> Result<Self::Source<'_>, IRCError>;
    fn command(&mut self, command: Commands);
    fn params(&mut self) -> Self::Params<'_>;
    fn trailing(&mut self, value: &str) -> Result<(), IRCError>;
    fn end(&mut self);
}

pub trait SerializeTags {
    fn tag(&mut self, key: &str, value: Option<&str>) -> Result<(), IRCError>;
    fn flag(&mut self, key: &str) -> Result<(), IRCError>;
    fn end(self);
}

pub trait SerializeSource {
    fn user(&mut self, user: &str) -> Result<(), IRCError>;
    fn host(&mut self, host: &str) -> Result<(), IRCError>;
    fn end(self);
}

pub trait SerializeParams {
    fn add(&mut self, value: &str) -> Result<(), IRCError>;
    fn end(self);
}

pub struct IRCSerializer {
    pub buffer: BytesMut,
}

impl IRCSerializer {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            buffer: BytesMut::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: BytesMut::with_capacity(capacity),
        }
    }

    pub fn into_bytes(self) -> Bytes {
        self.buffer.freeze()
    }
}

impl MessageSerializer for IRCSerializer {
    type Tags<'a>
        = IRCTagsSerializer<'a>
    where
        Self: 'a;

    type Source<'a>
        = IRCSourceSerializer<'a>
    where
        Self: 'a;

    type Params<'a>
        = IRCParamsSerializer<'a>
    where
        Self: 'a;

    fn tags(&mut self) -> Self::Tags<'_> {
        self.buffer.put_u8(AT);
        IRCTagsSerializer {
            buffer: &mut self.buffer,
            first: true,
        }
    }

    fn source(&mut self, name: &str) -> Result<Self::Source<'_>, IRCError> {
        if validators::host(name).is_err() {
            validators::nick(name)?;
        }

        self.buffer.put_u8(COLON);
        self.buffer.put_slice(name.as_bytes());
        Ok(IRCSourceSerializer {
            buffer: &mut self.buffer,
        })
    }

    fn command(&mut self, command: Commands) {
        self.buffer.put_slice(command.as_bytes());
    }

    fn params(&mut self) -> Self::Params<'_> {
        IRCParamsSerializer {
            buffer: &mut self.buffer,
        }
    }

    fn trailing(&mut self, value: &str) -> Result<(), IRCError> {
        validators::trailing(value)?;
        self.buffer.put_u8(SPACE);
        self.buffer.put_u8(COLON);
        self.buffer.put_slice(value.as_bytes());

        Ok(())
    }

    fn end(&mut self) {
        self.buffer.put_slice(b"\r\n");
    }
}

pub struct IRCTagsSerializer<'a> {
    buffer: &'a mut BytesMut,
    first: bool,
}

impl<'a> SerializeTags for IRCTagsSerializer<'a> {
    fn tag(&mut self, key: &str, value: Option<&str>) -> Result<(), IRCError> {
        validators::tag_key(key)?;

        if !self.first {
            self.buffer.put_u8(SEMICOLON);
        }

        self.first = false;

        self.buffer.put_slice(key.as_bytes());
        self.buffer.put_u8(EQ);

        if let Some(val) = value {
            validators::tag_value(val)?;
            self.buffer.put_slice(val.as_bytes());
        }

        Ok(())
    }

    fn flag(&mut self, key: &str) -> Result<(), IRCError> {
        validators::tag_key(key)?;

        if !self.first {
            self.buffer.put_u8(SEMICOLON);
        }

        self.first = false;

        self.buffer.put_slice(key.as_bytes());
        Ok(())
    }

    fn end(self) {
        self.buffer.put_u8(SPACE);
    }
}

pub struct IRCSourceSerializer<'a> {
    buffer: &'a mut BytesMut,
}

impl<'a> SerializeSource for IRCSourceSerializer<'a> {
    fn user(&mut self, user: &str) -> Result<(), IRCError> {
        validators::user(user)?;
        self.buffer.put_u8(BANG);
        self.buffer.put_slice(user.as_bytes());
        Ok(())
    }

    fn host(&mut self, host: &str) -> Result<(), IRCError> {
        validators::host(host)?;
        self.buffer.put_u8(AT);
        self.buffer.put_slice(host.as_bytes());
        Ok(())
    }

    fn end(self) {
        self.buffer.put_u8(SPACE);
    }
}

pub struct IRCParamsSerializer<'a> {
    buffer: &'a mut BytesMut,
}

impl<'a> SerializeParams for IRCParamsSerializer<'a> {
    fn add(&mut self, value: &str) -> Result<(), IRCError> {
        validators::param(value)?;
        self.buffer.put_u8(SPACE);
        self.buffer.put_slice(value.as_bytes());
        Ok(())
    }

    fn end(self) {}
}

#[cfg(test)]
mod tests {
    use crate::{
        message::ser::{SerializeParams, SerializeTags, ToMessage},
        Commands,
    };

    #[test]
    fn from_message() {
        struct Tags {
            field: String,
        }

        impl ToMessage for Tags {
            fn to_message<S: super::MessageSerializer>(
                &self,
                serialize: &mut S,
            ) -> Result<(), crate::IRCError> {
                let mut tags = serialize.tags();
                tags.tag("field", Some(&self.field))?;
                tags.end();

                Ok(())
            }
        }

        struct PrivMsg {
            tags: Tags,
            channel: String,
            message: String,
        }

        impl ToMessage for PrivMsg {
            fn to_message<S: super::MessageSerializer>(
                &self,
                serialize: &mut S,
            ) -> Result<(), crate::IRCError> {
                self.tags.to_message(serialize)?;

                serialize.command(Commands::PRIVMSG);

                let mut params = serialize.params();
                params.add(&self.channel)?;
                params.end();

                serialize.trailing(&self.message)?;

                serialize.end();
                Ok(())
            }
        }

        let msg = PrivMsg {
            tags: Tags {
                field: "value".to_string(),
            },
            channel: "#channel".to_string(),
            message: "Hi".to_string(),
        };

        let size = msg.serialized_size();
        let actual = crate::from_message(&msg).unwrap();
        assert_eq!("@field=value PRIVMSG #channel :Hi\r\n", actual);
        assert_eq!(35, size);
    }
}
