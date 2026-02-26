use bytes::Bytes;

use crate::compat::Debug;

use crate::{error::IRCError, validators, Commands};

use crate::message::ser::{
    IRCParamsSerializer, IRCSerializer, IRCSourceSerializer, IRCTagsSerializer, MessageSerializer,
    SizeTracker, ToMessage,
};

#[derive(Debug, Default, Clone)]
struct Components<'a> {
    tags: IRCTagsSerializer,
    source: IRCSourceSerializer,
    params: IRCParamsSerializer,
    trailing: Option<&'a str>,
}

impl<'a> Components<'a> {
    pub fn new() -> Self {
        Self {
            tags: IRCTagsSerializer::default(),
            source: IRCSourceSerializer::default(),
            params: IRCParamsSerializer::default(),
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

        self.source.to_message(serialize)?;

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
        self.source.validate()?;

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
        self.components
            .source
            .set_name(name)
            .map_err(IRCError::from)?;
        Ok(self)
    }

    pub fn set_source_user(&mut self, user: &'a str) -> Result<&mut Self, IRCError> {
        self.components
            .source
            .set_user(user)
            .map_err(IRCError::from)?;
        Ok(self)
    }

    pub fn set_source_host(&mut self, host: &'a str) -> Result<&mut Self, IRCError> {
        self.components
            .source
            .set_host(host)
            .map_err(IRCError::from)?;
        Ok(self)
    }

    pub fn set_source(
        &mut self,
        name: &'a str,
        user: Option<&'a str>,
        host: Option<&'a str>,
    ) -> Result<&mut Self, IRCError> {
        self.set_source_name(name)?;
        if let Some(user) = user {
            self.set_source_user(user)?;
        }

        if let Some(host) = host {
            self.set_source_host(host)?;
        }

        Ok(self)
    }

    pub fn add_param(&mut self, param: &'a str) -> Result<&mut Self, IRCError> {
        self.components.params.push(param)?;
        Ok(self)
    }

    pub fn add_params<I>(&mut self, params: I) -> Result<&mut Self, IRCError>
    where
        I: IntoIterator<Item = &'a str>,
    {
        self.components.params.extend(params)?;
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
            self, IRCParamsSerializer, IRCSerializer, IRCSourceSerializer, IRCTagsSerializer,
            SerializeParams, SerializeSource, SerializeTags, ToMessage,
        },
        MessageBuilder,
    };

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
        let mut source = IRCSourceSerializer::default();
        source.set_name("nick").unwrap();
        source.set_user("user").unwrap();
        source.set_host("example.com").unwrap();

        let mut buffer = IRCSerializer::new();
        source.to_message(&mut buffer).unwrap();

        assert_eq!(":nick!user@example.com ", buffer.into_bytes());
        assert_eq!(23, source.serialized_size());
    }

    #[test]
    fn source_user() {
        let mut source = IRCSourceSerializer::default();
        source.set_name("nick").unwrap();
        source.set_user("user").unwrap();

        let mut buffer = IRCSerializer::new();
        source.to_message(&mut buffer).unwrap();

        assert_eq!(":nick!user ", buffer.into_bytes());
        assert_eq!(11, source.serialized_size());
    }

    #[test]
    fn source_host() {
        let mut source = IRCSourceSerializer::default();
        source.set_name("nick").unwrap();
        source.set_host("example.com").unwrap();

        let mut buffer = IRCSerializer::new();
        source.to_message(&mut buffer).unwrap();

        assert_eq!(":nick@example.com ", buffer.into_bytes());
        assert_eq!(18, source.serialized_size());
    }

    #[test]
    fn source_server() {
        let mut source = IRCSourceSerializer::default();
        source.set_name("irc.example.com").unwrap();

        let mut buffer = IRCSerializer::new();
        source.to_message(&mut buffer).unwrap();

        assert_eq!(":irc.example.com ", buffer.into_bytes());
        assert_eq!(17, source.serialized_size());
    }

    #[test]
    fn params() {
        let mut params = IRCParamsSerializer::default();
        params.push("param1").unwrap();
        params.extend(["param2", "param3"]).unwrap();

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
