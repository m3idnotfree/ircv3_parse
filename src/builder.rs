use bytes::Bytes;

use crate::compat::{Debug, String, ToOwned};

use crate::ser::{
    IRCParamsSerializer, IRCSerializer, IRCSourceSerializer, IRCTagsSerializer, MessageSerializer,
    ToMessage,
};
use crate::{validators, Commands, SerError};

#[derive(Debug, Default, Clone)]
pub struct MessageBuilder {
    tags: IRCTagsSerializer,
    source: IRCSourceSerializer,
    command: Option<String>,
    params: IRCParamsSerializer,
    trailing: Option<String>,
}

impl MessageBuilder {
    pub fn new() -> Self {
        Self {
            tags: IRCTagsSerializer::default(),
            source: IRCSourceSerializer::default(),
            command: None,
            params: IRCParamsSerializer::default(),
            trailing: None,
        }
    }

    pub fn set_command(&mut self, command: Commands<'_>) -> Result<&mut Self, SerError> {
        if self.command.is_some() {
            Err(SerError::DuplicateCommand)
        } else {
            self.command = Some(command.as_str().to_owned());
            Ok(self)
        }
    }

    pub fn add_tag(&mut self, key: &str, value: Option<&str>) -> Result<&mut Self, SerError> {
        self.tags.insert_tag(key, value)?;

        Ok(self)
    }

    pub fn add_tags<'a, I>(&mut self, tags: I) -> Result<&mut Self, SerError>
    where
        I: IntoIterator<Item = (&'a str, Option<&'a str>)>,
    {
        for (key, value) in tags.into_iter() {
            self.add_tag(key, value)?;
        }

        Ok(self)
    }

    pub fn add_tag_flag(&mut self, key: &str) -> Result<&mut Self, SerError> {
        self.tags.insert_flag(key)?;
        Ok(self)
    }

    pub fn add_tag_flags<I, S>(&mut self, keys: I) -> Result<&mut Self, SerError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for key in keys.into_iter() {
            self.add_tag_flag(key.as_ref())?;
        }

        Ok(self)
    }

    pub fn set_source_name(&mut self, name: &str) -> Result<&mut Self, SerError> {
        self.source.set_name(name).map_err(SerError::from)?;
        Ok(self)
    }

    pub fn set_source_user(&mut self, user: &str) -> Result<&mut Self, SerError> {
        self.source.set_user(user).map_err(SerError::from)?;
        Ok(self)
    }

    pub fn set_source_host(&mut self, host: &str) -> Result<&mut Self, SerError> {
        self.source.set_host(host).map_err(SerError::from)?;
        Ok(self)
    }

    pub fn set_source(
        &mut self,
        name: &str,
        user: Option<&str>,
        host: Option<&str>,
    ) -> Result<&mut Self, SerError> {
        self.set_source_name(name)?;
        if let Some(user) = user {
            self.set_source_user(user)?;
        }

        if let Some(host) = host {
            self.set_source_host(host)?;
        }

        Ok(self)
    }

    pub fn add_param(&mut self, param: &str) -> Result<&mut Self, SerError> {
        self.params.push(param)?;
        Ok(self)
    }

    pub fn add_params<I, S>(&mut self, params: I) -> Result<&mut Self, SerError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.params.extend(params)?;
        Ok(self)
    }

    pub fn set_trailing(&mut self, trailing: &str) -> Result<&mut Self, SerError> {
        validators::trailing(trailing)?;
        self.trailing = Some(trailing.to_owned());
        Ok(self)
    }

    pub fn build(self) -> Result<Bytes, SerError> {
        let mut buffer = IRCSerializer::new();

        self.to_message(&mut buffer)?;

        Ok(buffer.into_bytes())
    }

    pub fn validate(&self) -> Result<(), SerError> {
        if self.command.is_none() {
            return Err(SerError::MissingCommand);
        }

        if let Some(trailing) = &self.trailing {
            validators::trailing(trailing)?;
        }

        Ok(())
    }
}

impl ToMessage for MessageBuilder {
    fn to_message<S: MessageSerializer>(&self, serialize: &mut S) -> Result<(), SerError> {
        self.tags.to_message(serialize)?;
        self.source.to_message(serialize)?;

        if let Some(command) = &self.command {
            let command = Commands::from(command.as_str());
            serialize.set_command(command);
        }

        self.params.to_message(serialize)?;

        if let Some(trailing) = &self.trailing {
            serialize.set_trailing(trailing)?;
        }

        serialize.end()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        components::Commands,
        ser::{
            self, IRCParamsSerializer, IRCSerializer, IRCSourceSerializer, IRCTagsSerializer,
            ToMessage,
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
    }

    #[test]
    fn source_user() {
        let mut source = IRCSourceSerializer::default();
        source.set_name("nick").unwrap();
        source.set_user("user").unwrap();

        let mut buffer = IRCSerializer::new();
        source.to_message(&mut buffer).unwrap();

        assert_eq!(":nick!user ", buffer.into_bytes());
    }

    #[test]
    fn source_host() {
        let mut source = IRCSourceSerializer::default();
        source.set_name("nick").unwrap();
        source.set_host("example.com").unwrap();

        let mut buffer = IRCSerializer::new();
        source.to_message(&mut buffer).unwrap();

        assert_eq!(":nick@example.com ", buffer.into_bytes());
    }

    #[test]
    fn source_server() {
        let mut source = IRCSourceSerializer::default();
        source.set_name("irc.example.com").unwrap();

        let mut buffer = IRCSerializer::new();
        source.to_message(&mut buffer).unwrap();

        assert_eq!(":irc.example.com ", buffer.into_bytes());
    }

    #[test]
    fn params() {
        let mut params = IRCParamsSerializer::default();
        params.push("param1").unwrap();
        params.extend(["param2", "param3"]).unwrap();

        let mut buffer = IRCSerializer::new();
        params.to_message(&mut buffer).unwrap();

        assert_eq!(" param1 param2 param3", buffer.into_bytes());
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

        let actual = msg.build().unwrap();
        assert_eq!(
            "@tag1=value1;tag2=;flag :nick!user@example.com PRIVMSG :\r\n",
            actual
        );
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
            ) -> Result<(), crate::SerError> {
                let tags = serialize.tags();
                for (key, value) in &self.tag {
                    tags.insert_tag(key, value.as_deref())?;
                }

                let source = serialize.source();
                source.set_name(&self.source)?;

                serialize.set_command(Commands::PRIVMSG);

                let params = serialize.params();
                for p in &self.param {
                    params.push(p)?;
                }

                serialize.set_trailing(&self.message)?;

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

        let msg = crate::to_message(&priv_msg).unwrap();

        assert_eq!("@key=value :name PRIVMSG param :hi\r\n", msg,);
    }
}
