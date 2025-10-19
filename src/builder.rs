use crate::compat::{format, Debug, Display, FmtResult, Formatter, String, ToString};

use crate::{components::Commands, validators, AT, COLON, SEMICOLON, SPACE};

use bytes::{BufMut, Bytes, BytesMut};

#[derive(Debug, PartialEq)]
enum BuildState {
    Start,
    Tags,
    Source,
    Params,
    Trailing,
    Complete,
}

#[derive(Debug)]
pub struct MessageBuilder<'a> {
    command: Commands<'a>,
    buffer: BytesMut,
    state: BuildState,
    #[cfg(debug_assertions)]
    validation_enabled: bool,
}

impl<'a> MessageBuilder<'a> {
    pub fn new(command: Commands<'a>) -> Self {
        Self {
            command,
            buffer: BytesMut::with_capacity(512),
            state: BuildState::Start,
            #[cfg(debug_assertions)]
            validation_enabled: true,
        }
    }

    pub fn with_capacity(command: Commands<'a>, capacity: usize) -> Self {
        Self {
            command,
            buffer: BytesMut::with_capacity(capacity),
            state: BuildState::Start,
            #[cfg(debug_assertions)]
            validation_enabled: true,
        }
    }

    pub fn with_tags<F>(mut self, f: F) -> Result<Self, BuilderError>
    where
        F: FnOnce(TagBuilder<'_>) -> Result<TagBuilder<'_>, BuilderError>,
    {
        if self.state != BuildState::Start {
            return Err(BuilderError::invalid_order(
                "Tags must come first",
                &self.state,
            ));
        }

        let tag_builder = f(TagBuilder::with_buffer(
            &mut self.buffer,
            #[cfg(debug_assertions)]
            self.validation_enabled,
        ))?;
        tag_builder.finish();

        self.state = BuildState::Tags;
        Ok(self)
    }

    pub fn with_source<F>(mut self, name: &str, f: F) -> Result<Self, BuilderError>
    where
        F: FnOnce(SourceBuilder<'_>) -> Result<SourceBuilder<'_>, BuilderError>,
    {
        if !matches!(self.state, BuildState::Start | BuildState::Tags) {
            return Err(BuilderError::invalid_order(
                "Source must come before command",
                &self.state,
            ));
        }

        self.buffer.put_u8(COLON);
        let source_builder = f(SourceBuilder::with_buffer(
            &mut self.buffer,
            name,
            #[cfg(debug_assertions)]
            self.validation_enabled,
        ))?;
        source_builder.finish();
        self.buffer.put_u8(SPACE);

        self.state = BuildState::Source;
        Ok(self)
    }

    fn write_command(&mut self) {
        if matches!(
            self.state,
            BuildState::Start | BuildState::Tags | BuildState::Source
        ) {
            self.buffer.put_slice(self.command.as_bytes());
        }
    }

    pub fn with_params<F>(mut self, f: F) -> Result<Self, BuilderError>
    where
        F: FnOnce(ParamBuilder<'_>) -> Result<ParamBuilder<'_>, BuilderError>,
    {
        if matches!(self.state, BuildState::Trailing | BuildState::Complete) {
            return Err(BuilderError::invalid_order(
                "Parameters must come before trailing",
                &self.state,
            ));
        }

        self.write_command();

        let param_builder = f(ParamBuilder::with_buffer(
            &mut self.buffer,
            #[cfg(debug_assertions)]
            self.validation_enabled,
        ))?;
        param_builder.finish();

        self.state = BuildState::Params;
        Ok(self)
    }

    pub fn with_trailing(mut self, content: &str) -> Result<Self, BuilderError> {
        if self.state == BuildState::Complete {
            return Err(BuilderError::invalid_order(
                "Cannot add trailing after completion",
                &self.state,
            ));
        }

        self.write_command();

        if !content.is_empty() {
            self.buffer.put_slice(&[SPACE, COLON]);
            self.buffer.put_slice(content.as_bytes());
        }

        self.state = BuildState::Trailing;
        Ok(self)
    }

    pub fn finish(mut self) -> Self {
        self.write_command();

        self.buffer.put_slice(b"\r\n");

        self.state = BuildState::Complete;
        self
    }

    pub fn to_bytes(mut self) -> Bytes {
        if self.state != BuildState::Complete {
            self.write_command();
            self.buffer.put_slice(b"\r\n");
        }

        self.buffer.freeze()
    }

    pub fn reset(&mut self) {
        self.buffer.clear();
        self.state = BuildState::Start;
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.buffer
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.buffer
    }

    pub fn build(mut self) -> Result<Bytes, BuilderError> {
        if self.state != BuildState::Complete {
            self.write_command();
            self.buffer.put_slice(b"\r\n");
        }

        Ok(self.buffer.freeze())
    }
}

#[derive(Debug)]
pub struct TagBuilder<'a> {
    buffer: &'a mut BytesMut,
    has_tags: bool,
    #[cfg(debug_assertions)]
    validation_enabled: bool,
}

impl<'a> TagBuilder<'a> {
    pub fn with_buffer(
        buffer: &'a mut BytesMut,
        #[cfg(debug_assertions)] validation_enabled: bool,
    ) -> Self {
        TagBuilder {
            buffer,
            has_tags: false,
            #[cfg(debug_assertions)]
            validation_enabled,
        }
    }

    pub fn add(mut self, key: &str, value: Option<&str>) -> Result<Self, BuilderError> {
        self.write_separator();

        #[cfg(debug_assertions)]
        if self.validation_enabled {
            validators::tag_key(key)
                .map_err(|e| BuilderError::tag_validation("tag key", key, e))?;
        }

        self.buffer.put_slice(key.as_bytes());

        if let Some(val) = value {
            #[cfg(debug_assertions)]
            if self.validation_enabled {
                validators::tag_value(val)
                    .map_err(|e| BuilderError::tag_validation("tag value", val, e))?;
            }

            self.buffer.put_u8(b'=');
            self.buffer.put_slice(val.as_bytes());
        }

        self.has_tags = true;
        Ok(self)
    }

    pub fn add_flag(self, key: &str) -> Result<Self, BuilderError> {
        self.add(key, None)
    }

    pub fn add_many<I>(mut self, tags: I) -> Result<Self, BuilderError>
    where
        I: IntoIterator<Item = (&'a str, Option<&'a str>)>,
    {
        for (key, value) in tags {
            self.write_separator();

            #[cfg(debug_assertions)]
            if self.validation_enabled {
                validators::tag_key(key)
                    .map_err(|e| BuilderError::tag_validation("tag key", key, e))?;
            }

            self.buffer.put_slice(key.as_bytes());

            if let Some(val) = value {
                #[cfg(debug_assertions)]
                if self.validation_enabled {
                    validators::tag_value(val)
                        .map_err(|e| BuilderError::tag_validation("tag value", val, e))?;
                }

                self.buffer.put_u8(b'=');
                self.buffer.put_slice(val.as_bytes());
            }

            self.has_tags = true;
        }

        Ok(self)
    }

    #[inline]
    fn write_separator(&mut self) {
        if self.has_tags {
            self.buffer.put_u8(SEMICOLON);
        } else {
            self.buffer.put_u8(AT);
        }
    }

    pub fn finish(self) {
        if self.has_tags {
            self.buffer.put_u8(SPACE);
        }
    }
}

#[derive(Debug)]
pub struct SourceBuilder<'a> {
    buffer: &'a mut BytesMut,
    #[cfg(debug_assertions)]
    validation_enabled: bool,
}

impl<'a> SourceBuilder<'a> {
    pub fn with_buffer(
        buffer: &'a mut BytesMut,
        name: &str,
        #[cfg(debug_assertions)] validation_enabled: bool,
    ) -> Self {
        buffer.put_slice(name.as_bytes());
        Self {
            buffer,
            validation_enabled,
        }
    }

    pub fn with_user(self, user: &str) -> Result<Self, BuilderError> {
        if user.is_empty() {
            return Err(BuilderError::empty_parameter("username"));
        }

        #[cfg(debug_assertions)]
        if self.validation_enabled {
            validators::user(user).map_err(|e| BuilderError::param_validation(user, e))?;
        }

        self.buffer.put_u8(b'!');
        self.buffer.put_slice(user.as_bytes());
        Ok(self)
    }

    pub fn with_host(self, host: &str) -> Result<Self, BuilderError> {
        if host.is_empty() {
            return Err(BuilderError::empty_parameter("hostname"));
        }

        #[cfg(debug_assertions)]
        if self.validation_enabled {
            use crate::rfc1123::RFC1123;
            RFC1123::new()
                .validate(host)
                .map_err(|e| BuilderError::host_validation(host, e))?;
        }

        self.buffer.put_u8(AT);
        self.buffer.put_slice(host.as_bytes());

        Ok(self)
    }

    pub fn finish(self) {}
}

#[derive(Debug)]
pub struct ParamBuilder<'a> {
    buffer: &'a mut BytesMut,
    has_params: bool,
    #[cfg(debug_assertions)]
    validation_enabled: bool,
}

impl<'a> ParamBuilder<'a> {
    pub fn with_buffer(
        buffer: &'a mut BytesMut,
        #[cfg(debug_assertions)] validation_enabled: bool,
    ) -> Self {
        Self {
            buffer,
            has_params: false,
            #[cfg(debug_assertions)]
            validation_enabled,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, param: &str) -> Result<Self, BuilderError> {
        #[cfg(debug_assertions)]
        if self.validation_enabled {
            validators::params(param)
                .map_err(|error| BuilderError::param_validation(param, error))?;
        }

        self.write_space_if_needed();
        self.buffer.put_slice(param.as_bytes());

        Ok(self)
    }

    pub fn add_many<I>(mut self, params: I) -> Result<Self, BuilderError>
    where
        I: IntoIterator<Item = &'a str>,
    {
        for param in params.into_iter() {
            #[cfg(debug_assertions)]
            if self.validation_enabled {
                validators::params(param)
                    .map_err(|error| BuilderError::param_validation(param, error))?;
            }

            self.write_space_if_needed();
            self.buffer.put_slice(param.as_bytes());
        }

        Ok(self)
    }

    pub fn add_comma_list<I>(mut self, items: I) -> Result<Self, BuilderError>
    where
        I: IntoIterator<Item = &'a str>,
    {
        self.write_space_if_needed();

        let mut first = true;
        for param in items.into_iter() {
            #[cfg(debug_assertions)]
            if self.validation_enabled {
                validators::params(param)
                    .map_err(|error| BuilderError::param_validation(param, error))?;
            }

            if !first {
                self.buffer.put_u8(b',');
            }
            self.buffer.put_slice(param.as_bytes());
            first = false;
        }

        Ok(self)
    }

    #[inline]
    fn write_space_if_needed(&mut self) {
        if !self.has_params {
            self.buffer.put_u8(SPACE);
            self.has_params = true;
        } else {
            self.buffer.put_u8(SPACE);
        }
    }

    pub fn finish(self) {}
}

#[derive(thiserror::Error)]
pub enum BuilderError {
    #[error("Invalid build order: {message} (current state: {state:?})")]
    InvalidOrder {
        message: &'static str,
        state: String,
    },
    #[error("Empty parameter: {field} cannot be empty")]
    EmptyParameter { field: &'static str },
    #[error("Tag validation failed for {field} '{input}': {reason}")]
    TagValidation {
        field: &'static str,
        input: String,
        reason: String,
    },
    #[error("Parameter validation failed for '{input}': {reason}")]
    ParamValidation { input: String, reason: String },
    #[error("Hostname validation failed for '{input}': {reason}")]
    HostValidation { input: String, reason: String },
}
impl Debug for BuilderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "IRC-BUILDER[{}]: {}", self.code(), self)
    }
}

impl BuilderError {
    fn invalid_order(message: &'static str, state: &BuildState) -> Self {
        Self::InvalidOrder {
            message,
            state: format!("{:?}", state),
        }
    }

    pub fn tag_validation(field: &'static str, input: &str, error: impl Display) -> Self {
        Self::TagValidation {
            field,
            input: input.to_string(),
            reason: error.to_string(),
        }
    }

    pub fn param_validation(input: &str, error: impl Display) -> Self {
        Self::ParamValidation {
            input: input.to_string(),
            reason: error.to_string(),
        }
    }

    fn host_validation(input: &str, error: impl Display) -> Self {
        Self::HostValidation {
            input: input.to_string(),
            reason: error.to_string(),
        }
    }

    fn empty_parameter(field: &'static str) -> Self {
        Self::EmptyParameter { field }
    }
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidOrder { .. } => "BUILD_ORDER",
            Self::EmptyParameter { .. } => "EMPTY_PARAM",
            Self::TagValidation { .. } => "TAG_VALIDATION",
            Self::ParamValidation { .. } => "PARAM_VALIDATION",
            Self::HostValidation { .. } => "HOST_VALIDATION",
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::components::Commands;

    use super::MessageBuilder;

    #[test]
    fn base() {
        let message = MessageBuilder::new(Commands::PRIVMSG)
            .with_tags(|tags| {
                tags.add("tag1", Some("value1"))?
                    .add("tag2", Some(""))?
                    .add_flag("flag")
            })
            .unwrap()
            .with_source("name", |source| source.with_host("example.com"))
            .unwrap()
            .with_trailing("")
            .unwrap();

        let actual = message.to_bytes();

        assert_eq!(
            "@tag1=value1;tag2=;flag :name@example.com PRIVMSG\r\n",
            actual
        );
    }
}
