use bytes::{BufMut, Bytes, BytesMut};

use crate::compat::{String, ToOwned, Vec};

use crate::error::{ParamError, SourceError, TagError};
use crate::{validators, Commands, IRCError};
use crate::{AT, BANG, COLON, EQ, SEMICOLON, SPACE};

/// Serialize the IRC message from custom data structure.
pub trait ToMessage {
    fn to_message<S: MessageSerializer>(&self, serialize: &mut S) -> Result<(), IRCError>;

    fn to_bytes(&self) -> Result<Bytes, IRCError> {
        let mut serializer = IRCSerializer::new();
        self.to_message(&mut serializer)?;
        Ok(serializer.into_bytes())
    }
}

pub trait MessageSerializer {
    fn tags(&mut self) -> &mut IRCTagsSerializer;
    fn source(&mut self) -> &mut IRCSourceSerializer;
    fn set_command(&mut self, command: Commands);
    fn params(&mut self) -> &mut IRCParamsSerializer;
    fn set_trailing(&mut self, value: &str) -> Result<(), IRCError>;
    fn end(&mut self) -> Result<(), IRCError>;
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
    fn tags(&mut self) -> &mut IRCTagsSerializer {
        &mut self.tags
    }

    fn source(&mut self) -> &mut IRCSourceSerializer {
        &mut self.source
    }

    fn set_command(&mut self, command: Commands) {
        self.command = Some(command.as_str().to_owned());
    }

    fn params(&mut self) -> &mut IRCParamsSerializer {
        &mut self.params
    }

    fn set_trailing(&mut self, value: &str) -> Result<(), IRCError> {
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

        self.source.validate()?;

        self.finished = true;
        Ok(())
    }
}

#[derive(Debug, Clone)]
enum TagTy {
    Value { key: String, value: Option<String> },
    Flag(String),
}

impl TagTy {
    fn key(&self) -> &str {
        match self {
            Self::Value { key, .. } | Self::Flag(key) => key,
        }
    }

    pub fn validate(&self) -> Result<(), TagError> {
        match self {
            Self::Value { key, value } => {
                validators::tag_key(key)?;
                if let Some(value) = value {
                    validators::tag_value(value)
                } else {
                    Ok(())
                }
            }
            Self::Flag(key) => validators::tag_key(key),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct IRCTagsSerializer {
    tags: Vec<TagTy>,
}

impl IRCTagsSerializer {
    pub fn insert_tag(&mut self, key: &str, value: Option<&str>) -> Result<(), TagError> {
        validators::tag_key(key)?;

        let value = value
            .map(|v| -> Result<String, TagError> {
                validators::tag_value(v)?;
                Ok(v.to_owned())
            })
            .transpose()?;

        let new_tag = TagTy::Value {
            key: key.to_owned(),
            value,
        };

        match self.tags.iter_mut().find(|tag| tag.key() == key) {
            Some(existing) => *existing = new_tag,
            None => self.push(new_tag),
        }

        Ok(())
    }

    pub fn insert_flag(&mut self, key: &str) -> Result<(), TagError> {
        validators::tag_key(key)?;

        let new_tag = TagTy::Flag(key.to_owned());

        match self.tags.iter_mut().find(|tag| tag.key() == key) {
            Some(existing) => *existing = new_tag,
            None => self.push(new_tag),
        }

        Ok(())
    }

    pub fn validate(&self) -> Result<(), TagError> {
        for tag in &self.tags {
            tag.validate()?;
        }

        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.tags.is_empty()
    }

    fn push(&mut self, ty: TagTy) {
        self.tags.push(ty);
    }

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

impl ToMessage for IRCTagsSerializer {
    fn to_message<S: MessageSerializer>(&self, serialize: &mut S) -> Result<(), IRCError> {
        if self.is_empty() {
            return Ok(());
        }

        let tags = serialize.tags();
        for tag in self.tags.iter() {
            match tag {
                TagTy::Value { key, value } => {
                    tags.insert_tag(key, value.as_deref())?;
                }
                TagTy::Flag(key) => {
                    tags.insert_flag(key)?;
                }
            };
        }

        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
pub struct IRCSourceSerializer {
    name: Option<String>,
    user: Option<String>,
    host: Option<String>,
}

impl IRCSourceSerializer {
    pub fn set_name(&mut self, name: &str) -> Result<(), SourceError> {
        if self.name.is_some() {
            return Err(SourceError::DublicateComponent { component: "name" });
        }

        if validators::host(name).is_err() {
            validators::nick(name)?;
        }

        self.name = Some(name.to_owned());
        Ok(())
    }

    pub fn set_user(&mut self, user: &str) -> Result<(), SourceError> {
        if self.user.is_some() {
            return Err(SourceError::DublicateComponent { component: "user" });
        }

        validators::user(user)?;

        self.user = Some(user.to_owned());
        Ok(())
    }

    pub fn set_host(&mut self, host: &str) -> Result<(), SourceError> {
        if self.host.is_some() {
            return Err(SourceError::DublicateComponent { component: "host" });
        }

        validators::host(host)?;

        self.host = Some(host.to_owned());
        Ok(())
    }

    pub fn validate(&self) -> Result<(), SourceError> {
        if (self.user.is_some() || self.host.is_some()) && self.name.is_none() {
            Err(SourceError::MissingNick)
        } else {
            Ok(())
        }
    }

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

impl ToMessage for IRCSourceSerializer {
    fn to_message<S: MessageSerializer>(&self, serialize: &mut S) -> Result<(), IRCError> {
        self.validate()?;

        let source = serialize.source();

        if let Some(name) = &self.name {
            source.set_name(name)?;
        } else {
            return Err(IRCError::Source(SourceError::MissingNick));
        }

        if let Some(user) = &self.user {
            source.set_user(user)?;
        }

        if let Some(host) = &self.host {
            source.set_host(host)?;
        }

        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
pub struct IRCParamsSerializer {
    params: Vec<String>,
}

impl IRCParamsSerializer {
    pub fn push(&mut self, param: &str) -> Result<(), ParamError> {
        validators::param(param)?;
        self.params.push(param.to_owned());
        Ok(())
    }

    pub fn extend<I, S>(&mut self, params: I) -> Result<(), ParamError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let validate: Result<Vec<String>, ParamError> = params
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

    fn write_to(&self, buffer: &mut BytesMut) {
        self.params.iter().for_each(|param| {
            buffer.put_u8(SPACE);
            buffer.put_slice(param.as_bytes());
        });
    }
}

impl ToMessage for IRCParamsSerializer {
    fn to_message<S: MessageSerializer>(&self, serialize: &mut S) -> Result<(), IRCError> {
        let params = serialize.params();
        params.extend(&self.params)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{ser::ToMessage, Commands};

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
                tags.insert_tag("field", Some(&self.field))?;

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

                serialize.set_command(Commands::PRIVMSG);

                let params = serialize.params();
                params.push(&self.channel)?;

                serialize.set_trailing(&self.message)?;

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

        let actual = crate::to_message(&msg).unwrap();
        assert_eq!("@field=value PRIVMSG #channel :Hi\r\n", actual);
    }

    #[test]
    fn tags_drop_guard() {
        struct Tags1;

        impl ToMessage for Tags1 {
            fn to_message<S: super::MessageSerializer>(
                &self,
                serialize: &mut S,
            ) -> Result<(), crate::IRCError> {
                let tags = serialize.tags();
                tags.insert_tag("field1", Some("value"))?;

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
                tags.insert_tag("field2", Some("value"))?;

                Ok(())
            }
        }

        struct Message {
            tag1: Tags1,
            tag2: Tags2,
        }

        impl ToMessage for Message {
            fn to_message<S: super::MessageSerializer>(
                &self,
                serialize: &mut S,
            ) -> Result<(), crate::IRCError> {
                self.tag1.to_message(serialize)?;
                self.tag2.to_message(serialize)?;

                serialize.set_command(Commands::PRIVMSG);
                Ok(())
            }
        }

        let msg = Message {
            tag1: Tags1,
            tag2: Tags2,
        };

        let actual = crate::to_message(&msg).unwrap();
        assert_eq!("@field1=value;field2=value PRIVMSG", actual);
    }

    #[test]
    fn tags_explicit_end_call() {
        struct Tags;

        impl ToMessage for Tags {
            fn to_message<S: super::MessageSerializer>(
                &self,
                serialize: &mut S,
            ) -> Result<(), crate::IRCError> {
                let _tags = serialize.tags();

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
                serialize.set_command(Commands::PRIVMSG);
                Ok(())
            }
        }

        let msg = Message { tags: Tags };

        let actual = crate::to_message(&msg).unwrap();
        assert_eq!("PRIVMSG", actual);
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
                source.set_name("nick")?;
                source.set_user("user").map_err(crate::IRCError::from)
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
                serialize.set_command(Commands::PRIVMSG);
                Ok(())
            }
        }

        let msg = Message { source: Source };
        let actual = crate::to_message(&msg).unwrap();
        assert_eq!(":nick!user PRIVMSG", actual);
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
                tags.insert_tag("field", Some("value1"))?;

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
                tags.insert_tag("field2", Some("value2"))?;

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

        let actual = crate::to_message(&msg).unwrap();
        assert_eq!("@field=value1;field2=value2 ", actual);
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

        let actual = crate::to_message(&msg).unwrap();
        assert_eq!(" param1 param2 param3 param4", actual);
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
                serialize.set_trailing(self.message.as_ref())
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

        let actual = crate::to_message(&msg).unwrap();
        assert_eq!(" :Hello world", actual);
    }

    #[test]
    fn tag_overwrite_value() {
        struct Tags;

        impl ToMessage for Tags {
            fn to_message<S: super::MessageSerializer>(
                &self,
                serialize: &mut S,
            ) -> Result<(), crate::IRCError> {
                let tags = serialize.tags();
                tags.insert_tag("key", Some("old"))?;
                tags.insert_tag("key", Some("new"))?;
                Ok(())
            }
        }

        let actual = crate::to_message(&Tags).unwrap();
        assert_eq!("@key=new ", actual);
    }

    #[test]
    fn tag_overwrite_flag() {
        struct Tags;

        impl ToMessage for Tags {
            fn to_message<S: super::MessageSerializer>(
                &self,
                serialize: &mut S,
            ) -> Result<(), crate::IRCError> {
                let tags = serialize.tags();
                tags.insert_flag("key")?;
                tags.insert_tag("key", Some("value"))?;
                Ok(())
            }
        }

        let actual = crate::to_message(&Tags).unwrap();
        assert_eq!("@key=value ", actual);
    }

    #[test]
    fn tag_overwrite_preserves_order() {
        struct Tags;

        impl ToMessage for Tags {
            fn to_message<S: super::MessageSerializer>(
                &self,
                serialize: &mut S,
            ) -> Result<(), crate::IRCError> {
                let tags = serialize.tags();
                tags.insert_tag("a", Some("1"))?;
                tags.insert_tag("b", Some("2"))?;
                tags.insert_tag("a", Some("new"))?;
                Ok(())
            }
        }

        let actual = crate::to_message(&Tags).unwrap();
        assert_eq!("@a=new;b=2 ", actual);
    }
}
