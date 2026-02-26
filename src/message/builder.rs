use bytes::Bytes;

use crate::compat::{Debug, Vec};

use crate::{
    error::{IRCError, ParamError, SourceError},
    validators, Commands,
};

use crate::message::ser::{
    IRCSerializer, IRCTagsSerializer, MessageSerializer, SerializeParams, SerializeSource,
    SizeTracker, ToMessage,
};

#[derive(Debug, Default, Clone, Copy)]
struct SourceParts<'a> {
    pub name: Option<&'a str>,
    pub user: Option<&'a str>,
    pub host: Option<&'a str>,
}

impl<'a> SourceParts<'a> {
    fn new() -> Self {
        Self {
            name: None,
            user: None,
            host: None,
        }
    }

    pub fn validate(&self) -> Result<(), SourceError> {
        if let Some(name) = &self.name {
            validators::nick(name)?;
        } else {
            return Err(SourceError::MissingNick);
        }

        if let Some(user) = self.user {
            validators::user(user)?;
        }

        if let Some(host) = self.host {
            validators::host(host)?;
        }

        Ok(())
    }
}

impl<'a> ToMessage for SourceParts<'a> {
    fn to_message<S: MessageSerializer>(&self, serialize: &mut S) -> Result<(), IRCError> {
        let source = serialize.source();

        if let Some(name) = self.name {
            source.name(name)?;
        } else {
            return Err(IRCError::Source(SourceError::MissingNick));
        }

        if let Some(user) = self.user {
            source.user(user)?;
        }

        if let Some(host) = self.host {
            source.host(host)?;
        }

        source.end();
        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
struct Params<'a>(Vec<&'a str>);

impl<'a> Params<'a> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, param: &'a str) {
        self.0.push(param);
    }

    pub fn extend<I>(&mut self, params: I)
    where
        I: IntoIterator<Item = &'a str>,
    {
        self.0.extend(params);
    }

    pub fn validate(&self) -> Result<(), ParamError> {
        for param in &self.0 {
            validators::param(param)?;
        }
        Ok(())
    }
}

impl<'a> ToMessage for Params<'a> {
    fn to_message<S: MessageSerializer>(&self, serialize: &mut S) -> Result<(), IRCError> {
        let params = serialize.params();
        for p in &self.0 {
            params.push(p)?;
        }

        params.end();
        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
struct Components<'a> {
    tags: IRCTagsSerializer,
    source: Option<SourceParts<'a>>,
    params: Params<'a>,
    trailing: Option<&'a str>,
}

impl<'a> Components<'a> {
    pub fn new() -> Self {
        Self {
            tags: IRCTagsSerializer::default(),
            source: None,
            params: Params::new(),
            trailing: None,
        }
    }

    pub fn to_message<T: MessageSerializer>(
        &self,
        serialize: &mut T,
        command: Commands,
    ) -> Result<(), IRCError> {
        if !self.tags.is_empty() {
            self.tags.to_message(serialize)?;
        }

        if let Some(source) = self.source {
            source.to_message(serialize)?;
        }

        serialize.command(command);

        self.params.to_message(serialize)?;

        if let Some(trailing) = self.trailing {
            serialize.trailing(trailing)?;
        }

        serialize.end()?;

        Ok(())
    }

    pub fn serialized_size(&self, command: Commands) -> usize {
        let mut tracker = SizeTracker::new();
        self.to_message(&mut tracker, command)
            .expect("size calculation shoulld not fail");

        tracker.total()
    }

    pub fn validate(&self) -> Result<(), IRCError> {
        self.tags.validate()?;
        if let Some(source) = &self.source {
            source.validate()?;
        }

        self.params.validate()?;

        if let Some(trailing) = self.trailing {
            validators::trailing(trailing)?;
        }

        Ok(())
    }
}

#[derive(Default)]
pub struct MessageBuilder<'a> {
    command: Option<Commands<'a>>,
    components: Components<'a>,
}

impl<'a> MessageBuilder<'a> {
    pub fn new() -> Self {
        Self {
            command: None,
            components: Components::new(),
        }
    }

    pub fn set_command(&mut self, command: Commands<'a>) -> Result<&mut Self, IRCError> {
        if self.command.is_some() {
            Err(IRCError::DuplicateCommand)
        } else {
            self.command = Some(command);
            Ok(self)
        }
    }

    pub fn add_tag(&mut self, key: &'a str, value: Option<&'a str>) -> Result<&mut Self, IRCError> {
        self.components.tags.insert_tag(key, value)?;

        Ok(self)
    }

    pub fn add_tags(&mut self, tags: &[(&'a str, Option<&'a str>)]) -> Result<&mut Self, IRCError> {
        for &(key, value) in tags {
            self.add_tag(key, value)?;
        }

        Ok(self)
    }

    pub fn add_tag_flag(&mut self, key: &'a str) -> Result<&mut Self, IRCError> {
        self.components.tags.insert_flag(key)?;
        Ok(self)
    }

    pub fn add_tag_flags(&mut self, keys: &[&'a str]) -> Result<&mut Self, IRCError> {
        for key in keys {
            self.add_tag_flag(key)?;
        }

        Ok(self)
    }

    pub fn set_source_name(&mut self, name: &'a str) -> Result<&mut Self, IRCError> {
        if self.components.source.is_some() {
            Err(IRCError::Source(SourceError::DublicateComponent {
                component: "name",
            }))
        } else {
            validators::nick(name)?;
            let mut source = SourceParts::new();
            source.name = Some(name);
            self.components.source = Some(source);
            Ok(self)
        }
    }

    pub fn set_source_user(&mut self, user: &'a str) -> Result<&mut Self, IRCError> {
        if let Some(ref mut source_parts) = self.components.source {
            validators::user(user)?;
            source_parts.user = Some(user);
            Ok(self)
        } else {
            Err(IRCError::SourceNotSet { component: "user" })
        }
    }

    pub fn set_source_host(&mut self, host: &'a str) -> Result<(), IRCError> {
        if let Some(ref mut source_parts) = self.components.source {
            validators::host(host)?;
            source_parts.host = Some(host);
            Ok(())
        } else {
            Err(IRCError::SourceNotSet { component: "host" })
        }
    }

    pub fn set_source(
        &mut self,
        name: &'a str,
        user: Option<&'a str>,
        host: Option<&'a str>,
    ) -> Result<&mut Self, IRCError> {
        let mut source = SourceParts::new();
        source.name = Some(name);
        source.user = user;
        source.host = host;

        source.validate()?;

        self.components.source = Some(source);
        Ok(self)
    }

    pub fn add_param(&mut self, param: &'a str) -> Result<&mut Self, IRCError> {
        validators::param(param)?;
        self.components.params.push(param);
        Ok(self)
    }

    pub fn add_params<I>(&mut self, params: I) -> Result<&mut Self, IRCError>
    where
        I: IntoIterator<Item = &'a str>,
    {
        self.components.params.extend(params);
        Ok(self)
    }

    pub fn set_trailing(&mut self, trailing: &'a str) -> Result<&mut Self, IRCError> {
        validators::trailing(trailing)?;
        self.components.trailing = Some(trailing);
        Ok(self)
    }

    pub fn build(self) -> Result<Bytes, IRCError> {
        if let Some(command) = self.command {
            let size = self.components.serialized_size(command);
            let mut buffer = IRCSerializer::with_capacity(size);

            self.components.to_message(&mut buffer, command)?;

            Ok(buffer.into_bytes())
        } else {
            Err(IRCError::MissingCommand)
        }
    }

    pub fn validator(&self) -> Result<(), IRCError> {
        self.components.validate()
    }
}

impl<'a> ToMessage for MessageBuilder<'a> {
    fn to_message<S: MessageSerializer>(&self, serialize: &mut S) -> Result<(), IRCError> {
        if let Some(command) = self.command {
            self.components.to_message(serialize, command)?;
            Ok(())
        } else {
            Err(IRCError::MissingCommand)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        components::Commands,
        message::ser::{
            self, IRCSerializer, IRCTagsSerializer, SerializeParams, SerializeSource,
            SerializeTags, ToMessage,
        },
        MessageBuilder,
    };

    use super::{Params, SourceParts};

    #[test]
    fn tags_single() {
        let mut tags = IRCTagsSerializer::default();
        tags.insert_tag("key", Some("value")).unwrap();

        let mut buffer = IRCSerializer::new();
        tags.to_message(&mut buffer).unwrap();

        assert_eq!("@key=value ", buffer.into_bytes());
        assert_eq!(11, tags.serialized_size());
    }

    #[test]
    fn tags_multiple() {
        let mut tags = IRCTagsSerializer::default();
        tags.insert_tag("key", Some("value")).unwrap();
        tags.insert_tag("key2", None).unwrap();
        tags.insert_flag("flag").unwrap();

        let mut buffer = IRCSerializer::new();
        tags.to_message(&mut buffer).unwrap();

        assert_eq!("@key=value;key2=;flag ", buffer.into_bytes());
        assert_eq!(22, tags.serialized_size());
    }

    #[test]
    fn source() {
        let mut source = SourceParts::new();
        source.name = Some("nick");
        source.user = Some("user");
        source.host = Some("example.com");

        let mut buffer = IRCSerializer::new();
        source.to_message(&mut buffer).unwrap();

        assert_eq!(":nick!user@example.com ", buffer.into_bytes());
        assert_eq!(23, source.serialized_size());
    }

    #[test]
    fn source_user() {
        let mut source = SourceParts::new();
        source.name = Some("nick");
        source.user = Some("user");

        let mut buffer = IRCSerializer::new();
        source.to_message(&mut buffer).unwrap();

        assert_eq!(":nick!user ", buffer.into_bytes());
        assert_eq!(11, source.serialized_size());
    }

    #[test]
    fn source_host() {
        let mut source = SourceParts::new();
        source.name = Some("nick");
        source.host = Some("example.com");

        let mut buffer = IRCSerializer::new();
        source.to_message(&mut buffer).unwrap();

        assert_eq!(":nick@example.com ", buffer.into_bytes());
        assert_eq!(18, source.serialized_size());
    }

    #[test]
    fn source_server() {
        let mut source = SourceParts::new();
        source.name = Some("irc.example.com");

        let mut buffer = IRCSerializer::new();
        source.to_message(&mut buffer).unwrap();

        assert_eq!(":irc.example.com ", buffer.into_bytes());
        assert_eq!(17, source.serialized_size());
    }

    #[test]
    fn params() {
        let mut params = Params::new();
        params.push("param1");
        params.extend(["param2", "param3"]);

        let mut buffer = IRCSerializer::new();
        params.to_message(&mut buffer).unwrap();

        assert_eq!(" param1 param2 param3", buffer.into_bytes());
        assert_eq!(21, params.serialized_size());
    }

    #[test]
    fn base() {
        let mut msg = MessageBuilder::new();
        msg.set_command(Commands::PRIVMSG).unwrap();
        msg.add_tag("tag1", Some("value1"))
            .unwrap()
            .add_tag("tag2", None)
            .unwrap()
            .add_tag_flag("flag")
            .unwrap();

        msg.set_source_name("nick").unwrap();
        msg.set_source_user("user").unwrap();
        msg.set_source_host("example.com").unwrap();

        msg.set_trailing("").unwrap();

        let size = msg.serialized_size();
        let actual = msg.build().unwrap();
        assert_eq!(
            "@tag1=value1;tag2=;flag :nick!user@example.com PRIVMSG :\r\n",
            actual
        );
        assert_eq!(58, size);
    }

    #[test]
    fn to_message() {
        struct PrivMsg {
            tag: Vec<(String, Option<String>)>,
            source: String,
            param: Vec<String>,
            message: String,
        }

        impl ToMessage for PrivMsg {
            fn to_message<S: ser::MessageSerializer>(
                &self,
                serialize: &mut S,
            ) -> Result<(), crate::IRCError> {
                let tags = serialize.tags();
                for (key, value) in &self.tag {
                    tags.tag(key, value.as_deref())?;
                }
                tags.end();

                let source = serialize.source();
                source.name(&self.source)?;
                source.end();

                serialize.command(Commands::PRIVMSG);

                let params = serialize.params();
                for p in &self.param {
                    params.push(p)?;
                }

                params.end();

                serialize.trailing(&self.message)?;

                serialize.end()?;
                Ok(())
            }
        }

        let priv_msg = PrivMsg {
            tag: vec![("key".to_string(), Some("value".to_string()))],
            source: "name".to_string(),
            param: vec!["param".to_string()],
            message: "hi".to_string(),
        };

        let size = priv_msg.serialized_size();
        let msg = crate::to_message(&priv_msg).unwrap();

        assert_eq!("@key=value :name PRIVMSG param :hi\r\n", msg,);
        assert_eq!(36, size);
    }
}
