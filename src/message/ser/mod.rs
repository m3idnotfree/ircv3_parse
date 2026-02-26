mod size_tracker;

pub use size_tracker::SizeTracker;

use bytes::{BufMut, Bytes, BytesMut};

use crate::compat::{String, ToOwned, Vec};

use crate::error::SourceError;
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
    impl Sealed for super::IRCTagsSerializer {}
    impl Sealed for super::IRCSourceSerializer {}
    impl Sealed for super::IRCParamsSerializer {}

    impl Sealed for super::SizeTracker {}
    impl Sealed for super::size_tracker::SizeTagsTracker {}
    impl Sealed for super::size_tracker::SizeSourceTracker {}
    impl Sealed for super::size_tracker::SizeParamsTracker {}
}

pub trait MessageSerializer: private::Sealed + Sized {
    type Tags: SerializeTags;
    type Source: SerializeSource;
    type Params: SerializeParams;

    fn tags(&mut self) -> &mut Self::Tags;
    fn source(&mut self) -> &mut Self::Source;
    fn command(&mut self, command: Commands);
    fn params(&mut self) -> &mut Self::Params;
    fn trailing(&mut self, value: &str) -> Result<(), IRCError>;
    fn end(&mut self) -> Result<(), IRCError>;
}

pub trait SerializeTags: private::Sealed {
    fn tag(&mut self, key: &str, value: Option<&str>) -> Result<(), IRCError>;
    fn flag(&mut self, key: &str) -> Result<(), IRCError>;
    fn end(&self);
}

pub trait SerializeSource: private::Sealed {
    fn name(&mut self, name: &str) -> Result<(), IRCError>;
    fn user(&mut self, user: &str) -> Result<(), IRCError>;
    fn host(&mut self, host: &str) -> Result<(), IRCError>;
    fn end(&self);
}

pub trait SerializeParams: private::Sealed {
    fn push(&mut self, value: &str) -> Result<(), IRCError>;
    fn extend<I, S>(&mut self, params: I) -> Result<(), IRCError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>;
    fn end(&self);
}

pub struct IRCSerializer {
    tags: IRCTagsSerializer,
    source: IRCSourceSerializer,
    command: Option<String>,
    params: IRCParamsSerializer,
    trailing: Option<String>,
    finished: bool,
    buffer: BytesMut,
}

impl IRCSerializer {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            tags: IRCTagsSerializer::default(),
            source: IRCSourceSerializer::default(),
            command: None,
            params: IRCParamsSerializer::default(),
            trailing: None,
            finished: false,
            buffer: BytesMut::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            tags: IRCTagsSerializer::default(),
            source: IRCSourceSerializer::default(),
            command: None,
            params: IRCParamsSerializer::default(),
            trailing: None,
            finished: false,
            buffer: BytesMut::with_capacity(capacity),
        }
    }

    fn flush_tags(&mut self) {
        if self.tags.write_to(&mut self.buffer) {
            self.buffer.put_u8(SPACE);
        }
    }

    fn flush_source(&mut self) {
        if self.source.write_to(&mut self.buffer) {
            self.buffer.put_u8(SPACE);
        }
    }

    fn flush_command(&mut self) {
        if let Some(command) = &self.command {
            self.buffer.put_slice(command.as_bytes());
        }
    }

    fn flush_params(&mut self) {
        self.params.write_to(&mut self.buffer);
    }

    fn flush_trailing(&mut self) {
        if let Some(trailing) = &self.trailing {
            self.buffer.put_u8(SPACE);
            self.buffer.put_u8(COLON);
            self.buffer.put_slice(trailing.as_bytes());
        }
    }

    pub fn into_bytes(mut self) -> Bytes {
        self.flush_tags();
        self.flush_source();
        self.flush_command();
        self.flush_params();
        self.flush_trailing();

        if self.finished {
            self.buffer.put_slice(b"\r\n");
        }

        self.buffer.freeze()
    }
}

impl MessageSerializer for IRCSerializer {
    type Tags = IRCTagsSerializer;
    type Source = IRCSourceSerializer;
    type Params = IRCParamsSerializer;

    fn tags(&mut self) -> &mut Self::Tags {
        &mut self.tags
    }

    fn source(&mut self) -> &mut Self::Source {
        &mut self.source
    }

    fn command(&mut self, command: Commands) {
        self.command = Some(command.as_str().to_owned());
    }

    fn params(&mut self) -> &mut Self::Params {
        &mut self.params
    }

    fn trailing(&mut self, value: &str) -> Result<(), IRCError> {
        validators::trailing(value)?;
        match &mut self.trailing {
            Some(t) => t.push_str(value),
            None => self.trailing = Some(value.to_owned()),
        }
        Ok(())
    }

    fn end(&mut self) -> Result<(), IRCError> {
        if self.command.is_none() {
            return Err(IRCError::MissingCommand);
        }

        self.finished = true;
        Ok(())
    }
}

#[derive(Debug, Clone)]
enum TagTy {
    Value { key: String, value: Option<String> },
    Flag(String),
}

#[derive(Debug, Default, Clone)]
pub struct IRCTagsSerializer {
    tags: Vec<TagTy>,
}

impl IRCTagsSerializer {
    fn write_to(&self, buffer: &mut BytesMut) -> bool {
        if self.tags.is_empty() {
            return false;
        }

        buffer.put_u8(AT);
        let mut first = true;
        for tag in &self.tags {
            if !first {
                buffer.put_u8(SEMICOLON);
            }
            first = false;

            match tag {
                TagTy::Value { key, value } => {
                    buffer.put_slice(key.as_bytes());
                    buffer.put_u8(EQ);
                    if let Some(val) = value {
                        buffer.put_slice(val.as_bytes());
                    }
                }
                TagTy::Flag(key) => {
                    buffer.put_slice(key.as_bytes());
                }
            }
        }

        true
    }
}

impl SerializeTags for IRCTagsSerializer {
    fn tag(&mut self, key: &str, value: Option<&str>) -> Result<(), IRCError> {
        validators::tag_key(key)?;

        let value = value
            .map(|v| -> Result<String, IRCError> {
                validators::tag_value(v)?;
                Ok(v.to_owned())
            })
            .transpose()?;

        self.tags.push(TagTy::Value {
            key: key.to_owned(),
            value,
        });
        Ok(())
    }

    fn flag(&mut self, key: &str) -> Result<(), IRCError> {
        validators::tag_key(key)?;
        self.tags.push(TagTy::Flag(key.to_owned()));
        Ok(())
    }

    fn end(&self) {}
}

#[derive(Debug, Default)]
pub struct IRCSourceSerializer {
    name: Option<String>,
    user: Option<String>,
    host: Option<String>,
}

impl IRCSourceSerializer {
    fn write_to(&self, buffer: &mut BytesMut) -> bool {
        if let Some(name) = &self.name {
            buffer.put_u8(COLON);
            buffer.put_slice(name.as_bytes());
        } else {
            return false;
        }

        if let Some(user) = &self.user {
            buffer.put_u8(BANG);
            buffer.put_slice(user.as_bytes());
        }

        if let Some(host) = &self.host {
            buffer.put_u8(AT);
            buffer.put_slice(host.as_bytes());
        }

        true
    }
}

impl SerializeSource for IRCSourceSerializer {
    fn name(&mut self, name: &str) -> Result<(), IRCError> {
        if self.name.is_some() {
            return Err(IRCError::Source(SourceError::DublicateComponent {
                component: "name",
            }));
        }

        if validators::host(name).is_err() {
            validators::nick(name)?;
        }

        self.name = Some(name.to_owned());
        Ok(())
    }

    fn user(&mut self, user: &str) -> Result<(), IRCError> {
        if self.user.is_some() {
            return Err(IRCError::Source(SourceError::DublicateComponent {
                component: "user",
            }));
        }

        validators::user(user)?;

        self.user = Some(user.to_owned());
        Ok(())
    }

    fn host(&mut self, host: &str) -> Result<(), IRCError> {
        if self.host.is_some() {
            return Err(IRCError::Source(SourceError::DublicateComponent {
                component: "host",
            }));
        }

        validators::host(host)?;

        self.host = Some(host.to_owned());
        Ok(())
    }

    fn end(&self) {}
}

#[derive(Debug, Default)]
pub struct IRCParamsSerializer {
    params: Vec<String>,
}

impl IRCParamsSerializer {
    fn write_to(&self, buffer: &mut BytesMut) {
        self.params.iter().for_each(|param| {
            buffer.put_u8(SPACE);
            buffer.put_slice(param.as_bytes());
        });
    }
}

impl SerializeParams for IRCParamsSerializer {
    fn push(&mut self, value: &str) -> Result<(), IRCError> {
        validators::param(value)?;
        self.params.push(value.to_owned());
        Ok(())
    }

    fn extend<I, S>(&mut self, params: I) -> Result<(), IRCError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let validate: Result<Vec<String>, IRCError> = params
            .into_iter()
            .map(|param| {
                let p = param.as_ref();
                validators::param(p)?;
                Ok(p.to_owned())
            })
            .collect();

        self.params.extend(validate?);
        Ok(())
    }

    fn end(&self) {}
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
                let tags = serialize.tags();
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

                let params = serialize.params();
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
                let tags = serialize.tags();
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
                let source = serialize.source();
                source.name("nick")?;
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
                let tags = serialize.tags();
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
                let tags = serialize.tags();
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
        assert_eq!("@field=value1;field2=value2 ", actual);
        assert_eq!(28, size);
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
                let params = serialize.params();

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
                let params = serialize.params();

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
                let params = serialize.params();

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
