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

mod private {
    pub trait Sealed {}

    impl Sealed for super::IRCSerializer {}
    impl<'a> Sealed for super::IRCTagsSerializer<'a> {}
    impl<'a> Sealed for super::IRCSourceSerializer<'a> {}
    impl<'a> Sealed for super::IRCParamsSerializer<'a> {}

    impl Sealed for super::SizeTracker {}
    impl<'a> Sealed for super::size_tracker::SizeTagsTracker<'a> {}
    impl<'a> Sealed for super::size_tracker::SizeSourceTracker<'a> {}
    impl<'a> Sealed for super::size_tracker::SizeParamsTracker<'a> {}
}

pub trait MessageSerializer: private::Sealed + Sized {
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
    fn end(&mut self) -> Result<(), IRCError>;
}

pub trait SerializeTags: private::Sealed {
    fn tag(&mut self, key: &str, value: Option<&str>) -> Result<(), IRCError>;
    fn flag(&mut self, key: &str) -> Result<(), IRCError>;
    fn end(self);
}

pub trait SerializeSource: private::Sealed {
    fn user(&mut self, user: &str) -> Result<(), IRCError>;
    fn host(&mut self, host: &str) -> Result<(), IRCError>;
    fn end(self);
}

pub trait SerializeParams: private::Sealed {
    fn push(&mut self, value: &str) -> Result<(), IRCError>;
    fn extend<I, S>(&mut self, params: I) -> Result<(), IRCError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>;
    fn end(self);
}

pub struct IRCSerializer {
    has_tags: bool,
    has_command: bool,
    has_trailing: bool,
    needs_space: bool,
    buffer: BytesMut,
}

impl IRCSerializer {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            has_tags: false,
            has_command: false,
            has_trailing: false,
            needs_space: false,
            buffer: BytesMut::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            has_tags: false,
            has_command: false,
            has_trailing: false,
            needs_space: false,
            buffer: BytesMut::with_capacity(capacity),
        }
    }

    fn add_space_if_needed(&mut self) {
        if self.needs_space {
            self.buffer.put_u8(SPACE);
            self.needs_space = false;
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
        IRCTagsSerializer {
            buffer: &mut self.buffer,
            has_tags: &mut self.has_tags,
            needs_space: &mut self.needs_space,
        }
    }

    fn source(&mut self, name: &str) -> Result<Self::Source<'_>, IRCError> {
        if validators::host(name).is_err() {
            validators::nick(name)?;
        }

        self.add_space_if_needed();

        self.buffer.put_u8(COLON);
        self.buffer.put_slice(name.as_bytes());
        Ok(IRCSourceSerializer {
            buffer: &mut self.buffer,
            needs_space: &mut self.needs_space,
        })
    }

    fn command(&mut self, command: Commands) {
        self.add_space_if_needed();
        self.has_command = true;
        self.buffer.put_slice(command.as_bytes());
    }

    fn params(&mut self) -> Self::Params<'_> {
        IRCParamsSerializer {
            buffer: &mut self.buffer,
        }
    }

    fn trailing(&mut self, value: &str) -> Result<(), IRCError> {
        validators::trailing(value)?;
        if !self.has_trailing {
            self.buffer.put_u8(SPACE);
            self.buffer.put_u8(COLON);
            self.has_trailing = true;
        }

        self.buffer.put_slice(value.as_bytes());
        Ok(())
    }

    fn end(&mut self) -> Result<(), IRCError> {
        if !self.has_command {
            return Err(IRCError::MissingCommand);
        }

        self.buffer.put_slice(b"\r\n");
        Ok(())
    }
}

pub struct IRCTagsSerializer<'a> {
    buffer: &'a mut BytesMut,
    has_tags: &'a mut bool,
    needs_space: &'a mut bool,
}

impl<'a> SerializeTags for IRCTagsSerializer<'a> {
    fn tag(&mut self, key: &str, value: Option<&str>) -> Result<(), IRCError> {
        validators::tag_key(key)?;

        if !*self.has_tags {
            self.buffer.put_u8(AT);
            *self.has_tags = true;
        } else {
            self.buffer.put_u8(SEMICOLON);
        }

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

        if !*self.has_tags {
            self.buffer.put_u8(AT);
            *self.has_tags = true;
        } else {
            self.buffer.put_u8(SEMICOLON);
        }

        self.buffer.put_slice(key.as_bytes());
        Ok(())
    }

    fn end(self) {
        if !self.buffer.is_empty() {
            *self.needs_space = true;
        }
    }
}

impl Drop for IRCTagsSerializer<'_> {
    fn drop(&mut self) {
        if !self.buffer.is_empty() {
            *self.needs_space = true;
        }
    }
}

pub struct IRCSourceSerializer<'a> {
    buffer: &'a mut BytesMut,
    needs_space: &'a mut bool,
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
        *self.needs_space = true;
    }
}

impl Drop for IRCSourceSerializer<'_> {
    fn drop(&mut self) {
        *self.needs_space = true;
    }
}

pub struct IRCParamsSerializer<'a> {
    buffer: &'a mut BytesMut,
}

impl<'a> SerializeParams for IRCParamsSerializer<'a> {
    fn push(&mut self, value: &str) -> Result<(), IRCError> {
        validators::param(value)?;
        self.buffer.put_u8(SPACE);
        self.buffer.put_slice(value.as_bytes());
        Ok(())
    }

    fn extend<I, S>(&mut self, params: I) -> Result<(), IRCError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for param in params {
            validators::param(param.as_ref())?;
            self.buffer.put_u8(SPACE);
            self.buffer.put_slice(param.as_ref().as_bytes());
        }
        Ok(())
    }

    fn end(self) {}
}

#[cfg(test)]
mod tests {
    use crate::{
        message::ser::{SerializeParams, SerializeSource, SerializeTags, ToMessage},
        Commands,
    };

    #[test]
    fn complete_privmsg() {
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
                params.push(&self.channel)?;
                params.end();

                serialize.trailing(&self.message)?;

                serialize.end()?;
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
        let actual = crate::to_message(&msg).unwrap();
        assert_eq!("@field=value PRIVMSG #channel :Hi\r\n", actual);
        assert_eq!(35, size);
    }

    #[test]
    fn tags_drop_guard() {
        struct Tags;

        impl ToMessage for Tags {
            fn to_message<S: super::MessageSerializer>(
                &self,
                serialize: &mut S,
            ) -> Result<(), crate::IRCError> {
                let mut tags = serialize.tags();
                tags.tag("field", Some("value"))?;

                Ok(())
            }
        }

        struct Message {
            tag1: Tags,
            tag2: Tags,
        }

        impl ToMessage for Message {
            fn to_message<S: super::MessageSerializer>(
                &self,
                serialize: &mut S,
            ) -> Result<(), crate::IRCError> {
                self.tag1.to_message(serialize)?;
                self.tag2.to_message(serialize)?;

                serialize.command(Commands::PRIVMSG);
                Ok(())
            }
        }

        let msg = Message {
            tag1: Tags,
            tag2: Tags,
        };

        let size = msg.serialized_size();
        let actual = crate::to_message(&msg).unwrap();
        assert_eq!("@field=value;field=value PRIVMSG", actual);
        assert_eq!(32, size);
    }

    #[test]
    fn tags_explicit_end_call() {
        struct Tags;

        impl ToMessage for Tags {
            fn to_message<S: super::MessageSerializer>(
                &self,
                serialize: &mut S,
            ) -> Result<(), crate::IRCError> {
                let tags = serialize.tags();
                tags.end();

                Ok(())
            }
        }

        struct Message {
            tags: Tags,
        }

        impl ToMessage for Message {
            fn to_message<S: super::MessageSerializer>(
                &self,
                serialize: &mut S,
            ) -> Result<(), crate::IRCError> {
                self.tags.to_message(serialize)?;
                serialize.command(Commands::PRIVMSG);
                Ok(())
            }
        }

        let msg = Message { tags: Tags };

        let size = msg.serialized_size();
        let actual = crate::to_message(&msg).unwrap();
        assert_eq!("PRIVMSG", actual);
        assert_eq!(7, size);
    }

    #[test]
    fn source_drop_guard() {
        struct Source;

        impl ToMessage for Source {
            fn to_message<S: super::MessageSerializer>(
                &self,
                serialize: &mut S,
            ) -> Result<(), crate::IRCError> {
                let mut source = serialize.source("nick")?;
                source.user("user")
            }
        }

        struct Message {
            source: Source,
        }

        impl ToMessage for Message {
            fn to_message<S: super::MessageSerializer>(
                &self,
                serialize: &mut S,
            ) -> Result<(), crate::IRCError> {
                self.source.to_message(serialize)?;
                serialize.command(Commands::PRIVMSG);
                Ok(())
            }
        }

        let msg = Message { source: Source };
        let size = msg.serialized_size();
        let actual = crate::to_message(&msg).unwrap();
        assert_eq!(":nick!user PRIVMSG", actual);
        assert_eq!(18, size);
    }

    #[test]
    fn multiple_tags() {
        struct Tags1;

        impl ToMessage for Tags1 {
            fn to_message<S: super::MessageSerializer>(
                &self,
                serialize: &mut S,
            ) -> Result<(), crate::IRCError> {
                let mut tags = serialize.tags();
                tags.tag("field", Some("value1"))?;
                tags.end();

                Ok(())
            }
        }

        struct Tags2;

        impl ToMessage for Tags2 {
            fn to_message<S: super::MessageSerializer>(
                &self,
                serialize: &mut S,
            ) -> Result<(), crate::IRCError> {
                let mut tags = serialize.tags();
                tags.tag("field2", Some("value2"))?;
                tags.end();

                Ok(())
            }
        }

        struct Message {
            tags1: Tags1,
            tags2: Tags2,
        }

        impl ToMessage for Message {
            fn to_message<S: super::MessageSerializer>(
                &self,
                serialize: &mut S,
            ) -> Result<(), crate::IRCError> {
                self.tags1.to_message(serialize)?;
                self.tags2.to_message(serialize)?;
                Ok(())
            }
        }

        let msg = Message {
            tags1: Tags1,
            tags2: Tags2,
        };

        let size = msg.serialized_size();
        let actual = crate::to_message(&msg).unwrap();
        assert_eq!("@field=value1;field2=value2", actual);
        assert_eq!(27, size);
    }

    #[test]
    fn multiple_params() {
        struct Param1 {
            param: String,
        }

        impl ToMessage for Param1 {
            fn to_message<S: super::MessageSerializer>(
                &self,
                serialize: &mut S,
            ) -> Result<(), crate::IRCError> {
                let mut params = serialize.params();

                params.push(self.param.as_ref())?;
                params.end();
                Ok(())
            }
        }

        struct Param2 {
            param: String,
        }

        impl ToMessage for Param2 {
            fn to_message<S: super::MessageSerializer>(
                &self,
                serialize: &mut S,
            ) -> Result<(), crate::IRCError> {
                let mut params = serialize.params();

                params.push(self.param.as_ref())?;
                params.end();
                Ok(())
            }
        }

        struct Param3 {
            param: Vec<String>,
        }

        impl ToMessage for Param3 {
            fn to_message<S: super::MessageSerializer>(
                &self,
                serialize: &mut S,
            ) -> Result<(), crate::IRCError> {
                let mut params = serialize.params();

                let param_refs: Vec<&str> = self.param.iter().map(|p| p.as_ref()).collect();

                params.extend(&param_refs)?;
                params.end();
                Ok(())
            }
        }

        struct Message {
            param1: Param1,
            param2: Param2,
            param3: Param3,
        }

        impl ToMessage for Message {
            fn to_message<S: super::MessageSerializer>(
                &self,
                serialize: &mut S,
            ) -> Result<(), crate::IRCError> {
                self.param1.to_message(serialize)?;
                self.param2.to_message(serialize)?;
                self.param3.to_message(serialize)?;
                Ok(())
            }
        }

        let msg = Message {
            param1: Param1 {
                param: "param1".to_string(),
            },
            param2: Param2 {
                param: "param2".to_string(),
            },
            param3: Param3 {
                param: vec!["param3".to_string(), "param4".to_string()],
            },
        };

        let size = msg.serialized_size();
        let actual = crate::to_message(&msg).unwrap();
        assert_eq!(" param1 param2 param3 param4", actual);
        assert_eq!(28, size);
    }

    #[test]
    fn multiple_trailing() {
        struct Trailing {
            message: String,
        }

        impl ToMessage for Trailing {
            fn to_message<S: super::MessageSerializer>(
                &self,
                serialize: &mut S,
            ) -> Result<(), crate::IRCError> {
                serialize.trailing(self.message.as_ref())
            }
        }

        struct Message {
            message1: Trailing,
            message2: Trailing,
        }

        impl ToMessage for Message {
            fn to_message<S: super::MessageSerializer>(
                &self,
                serialize: &mut S,
            ) -> Result<(), crate::IRCError> {
                self.message1.to_message(serialize)?;
                self.message2.to_message(serialize)
            }
        }

        let msg = Message {
            message1: Trailing {
                message: "Hello".to_string(),
            },
            message2: Trailing {
                message: " world".to_string(),
            },
        };

        let size = msg.serialized_size();
        let actual = crate::to_message(&msg).unwrap();
        assert_eq!(" :Hello world", actual);
        assert_eq!(13, size);
    }
}
