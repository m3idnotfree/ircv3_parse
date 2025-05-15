use std::{
    borrow::Cow,
    error::Error,
    fmt::{Debug, Display},
    sync::atomic::{AtomicUsize, Ordering},
};

mod kind;

pub use kind::{
    ErrorKind, ParserErrorKind, ProtocolErrorKind, ResourceErrorKind, SecurityErrorKind,
};

static ERROR_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(PartialEq)]
pub struct IRCv3Error<I> {
    pub input: I,
    pub error: ErrorKind,
    pub reason: Cow<'static, str>,
    pub position: Option<usize>,
    pub component: Cow<'static, str>,
    pub error_id: u64,
}

impl<I> IRCv3Error<I> {
    pub fn new(input: I, error: ErrorKind, reason: impl Into<Cow<'static, str>>) -> Self {
        let counter = ERROR_COUNTER.fetch_add(1, Ordering::Relaxed);
        Self {
            input,
            error,
            reason: reason.into(),
            position: None,
            component: "parser".into(),
            error_id: create_error_id(error, counter),
        }
    }

    pub fn with_position(
        input: I,
        error: ErrorKind,
        reason: impl Into<Cow<'static, str>>,
        position: usize,
    ) -> Self {
        let counter = ERROR_COUNTER.fetch_add(1, Ordering::Relaxed);

        Self {
            input,
            error,
            reason: reason.into(),
            position: Some(position),
            component: "parser".into(),
            error_id: create_error_id(error, counter),
        }
    }
    pub fn with_component(
        input: I,
        error: ErrorKind,
        reason: impl Into<Cow<'static, str>>,
        component: impl Into<Cow<'static, str>>,
    ) -> Self {
        let counter = ERROR_COUNTER.fetch_add(1, Ordering::Relaxed);

        Self {
            input,
            error,
            reason: reason.into(),
            position: None,
            component: component.into(),
            error_id: create_error_id(error, counter),
        }
    }

    pub fn detailed(
        input: I,
        error: ErrorKind,
        reason: impl Into<Cow<'static, str>>,
        position: usize,
        component: impl Into<Cow<'static, str>>,
    ) -> Self {
        let counter = ERROR_COUNTER.fetch_add(1, Ordering::Relaxed);

        Self {
            input,
            error,
            reason: reason.into(),
            position: Some(position),
            component: component.into(),
            error_id: create_error_id(error, counter),
        }
    }
    pub fn kind(&self) -> &ErrorKind {
        &self.error
    }
    pub fn code(&self) -> &'static str {
        self.error.error_code()
    }
    pub fn map_input<J, F>(self, f: F) -> IRCv3Error<J>
    where
        F: FnOnce(I) -> J,
    {
        IRCv3Error {
            input: f(self.input),
            error: self.error,
            reason: self.reason,
            position: self.position,
            component: self.component,
            error_id: self.error_id,
        }
    }
    pub fn is_alert_worthy(&self) -> bool {
        self.error.is_alert_worthy()
    }
}

fn create_error_id(error: ErrorKind, counter: usize) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    format!("{:?}_{}", error, counter).hash(&mut hasher);
    hasher.finish()
}

impl<I: Display> Display for IRCv3Error<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.error.error_code(), self.reason)
    }
}

impl<I: Display> Debug for IRCv3Error<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            f.debug_struct("IRCv3Error")
                .field("error_id", &format!("{:016x}", self.error_id))
                .field("code", &self.error.error_code())
                .field("error", &self.error)
                .field("reason", &self.reason)
                .field("position", &self.position)
                .field("component", &self.component)
                .field(
                    "input_excerpt",
                    &format_args!("{}", get_input_excerpt(&self.input, self.position)),
                )
                .finish()
        } else {
            write!(
                f,
                "IRC[{}] {}: {} ({})",
                self.error.error_code(),
                self.component,
                self.reason,
                if let Some(pos) = self.position {
                    format!("at pos {}", pos)
                } else {
                    "unknown position".to_string()
                }
            )
        }
    }
}

fn get_input_excerpt<I: Display>(input: &I, position: Option<usize>) -> String {
    let input_str = input.to_string();

    if let Some(pos) = position {
        let start = pos.saturating_sub(10);
        let end = (pos + 10).min(input_str.len());

        if start < end {
            format!("{}⟨!⟩{}", &input_str[start..pos], &input_str[pos..end])
        } else {
            input_str.chars().take(20).collect()
        }
    } else {
        input_str.chars().take(20).collect()
    }
}

impl<I: Display> Error for IRCv3Error<I> {}

impl IRCv3Error<&'_ str> {
    /// ╭── IRC Parser Error ───────────────────────────────────────────╮
    /// │ \[NickValidator\]                                               │
    /// │ Error: Nickname must start with a letter (a-z, A-Z)           │
    /// │ Position: 0                                                   │
    /// │                                                               │
    /// │ Input: "123nick!user@host"                                    │
    /// │        ^ ERROR HERE                                           │
    /// ╰───────────────────────────────────────────────────────────────╯
    pub fn visual_error(&self) -> String {
        let input = self.input;

        const BOX_WIDTH: usize = 64;

        let create_line = |content: &str| -> String {
            let content_len = content.chars().count();
            if content_len <= BOX_WIDTH - 2 {
                format!("│ {}{} │", content, " ".repeat(BOX_WIDTH - 2 - content_len))
            } else {
                let truncated: String = content.chars().take(BOX_WIDTH - 5).collect();
                format!("│ {}... │", truncated)
            }
        };

        let header_text = format!(
            "IRC Parser Error [{}]",
            format_args!("{:016x}", self.error_id)
        );
        let header_padding_len = BOX_WIDTH - 2 - header_text.len() - 2;
        let header = format!("╭── {} {}╮", header_text, "─".repeat(header_padding_len));

        let footer = format!("╰{}╯", "─".repeat(BOX_WIDTH));

        let mut result = String::new();
        result.push('\n');
        result.push_str(&header);
        result.push('\n');

        result.push_str(&create_line(&format!("Code: {}", self.error.error_code())));
        result.push('\n');
        result.push_str(&create_line(&format!("Component: {}", self.component)));
        result.push('\n');
        result.push_str(&create_line(&format!("Error: {}", self.reason)));
        result.push('\n');

        if let Some(pos) = self.position {
            result.push_str(&create_line(&format!("Position: {}", pos)));
            result.push('\n');

            result.push_str(&create_line(""));
            result.push('\n');

            let start = if pos > 20 { pos - 20 } else { 0 };
            let end = std::cmp::min(input.len(), pos + 30);
            let excerpt = &input[start..end];

            let input_text = format!("Input: \"{}\"", excerpt);
            result.push_str(&create_line(&input_text));
            result.push('\n');

            let pointer_position = if pos > 20 { 20 } else { pos };
            let mut pointer = " ".repeat(pointer_position);
            pointer.push_str("^ ERROR HERE");

            let pointer_text = format!("       {}", pointer);
            result.push_str(&create_line(&pointer_text));
            result.push('\n');
        } else {
            result.push_str(&create_line(""));
            result.push('\n');

            let input_excerpt: String = input.chars().take(50).collect();
            let input_text = format!("Input: \"{}\"", input_excerpt);
            result.push_str(&create_line(&input_text));
            result.push('\n');
        }

        result.push_str(&create_line(""));
        result.push('\n');

        result.push_str(&footer);

        result
    }

    pub fn log_format(&self) -> String {
        format!(
            "IRC-ERROR [{}] {}/{} {} {}",
            self.error.error_code(),
            self.component,
            if let Some(pos) = self.position {
                format!("pos:{}", pos)
            } else {
                "pos:?".to_string()
            },
            self.reason,
            if self.input.len() <= 30 {
                format!("input:\"{}\"", self.input)
            } else {
                format!("input:\"{}...\"", &self.input[..27])
            }
        )
    }

    pub fn structured_format(&self) -> String {
        use serde_json::{json, to_string};

        let json = json!({
            "error_id": format!("{:016x}", self.error_id),
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "code": self.error.error_code(),
            "component": self.component,
            "reason": self.reason,
            "position": self.position,
            "input_excerpt": if let Some(pos) = self.position {
                let start = pos.saturating_sub(10);
                let end = (pos + 10).min(self.input.len());
                self.input[start..end].to_string()
            } else {
                self.input.chars().take(20).collect::<String>()
            }
        });

        to_string(&json).unwrap_or_else(|_| {
            format!(
                "{{\"error\":\"JSON serialization failed\",\"code\":\"{}\"}}",
                self.error
            )
        })
    }
}

#[cfg(feature = "tracing")]
pub mod tracing {
    use std::time::Instant;

    use ::tracing::{debug, error, event, info, Level};

    use super::*;

    pub struct ErrorEvent<'a> {
        pub error_id: u64,
        pub code: &'static str,
        pub component: &'a str,
        pub reason: &'a str,
        pub position: Option<usize>,
        pub input_excerpt: String,
        pub timestamp: chrono::DateTime<chrono::Utc>,
    }

    impl<'a> ErrorEvent<'a> {
        pub fn from_error(error: &'a IRCv3Error<&'a str>) -> Self {
            let input_excerpt = if let Some(pos) = error.position {
                let start = pos.saturating_sub(10);
                let end = (pos + 10).min(error.input.len());
                error.input[start..end].to_string()
            } else {
                error.input.chars().take(20).collect::<String>()
            };

            Self {
                error_id: error.error_id,
                code: error.error.error_code(),
                component: &error.component,
                reason: &error.reason,
                position: error.position,
                input_excerpt,
                timestamp: chrono::Utc::now(),
            }
        }
    }

    pub struct ErrorCounter {
        error_counts: std::collections::HashMap<&'static str, usize>,
        last_flush: Instant,
        flush_interval: std::time::Duration,
    }

    impl ErrorCounter {
        pub fn new(flush_interval_secs: u64) -> Self {
            Self {
                error_counts: std::collections::HashMap::new(),
                last_flush: Instant::now(),
                flush_interval: std::time::Duration::from_secs(flush_interval_secs),
            }
        }

        pub fn increment(&mut self, error: &ErrorKind) {
            let code = error.error_code();
            *self.error_counts.entry(code).or_insert(0) += 1;

            self.check_flush();
        }

        fn check_flush(&mut self) {
            if self.last_flush.elapsed() >= self.flush_interval {
                self.flush();
            }
        }

        fn flush(&mut self) {
            for (code, count) in &self.error_counts {
                info!(
                    error_code = code,
                    count = count,
                    interval_secs = self.flush_interval.as_secs(),
                    "IRC parser error count"
                );
            }

            self.error_counts.clear();
            self.last_flush = Instant::now();
        }
    }

    pub trait TracingErrorExt {
        fn log_with_tracing(&self) -> &Self;
        fn count_in_metrics(&self) -> &Self;
    }

    impl TracingErrorExt for IRCv3Error<&'_ str> {
        fn log_with_tracing(&self) -> &Self {
            let event_data = ErrorEvent::from_error(self);
            match self.error.suggested_log_level() {
                "ERROR" => {
                    event!(
                        Level::ERROR,
                        error_id = format!("{:016x}", event_data.error_id),
                        error_code = %event_data.code,
                        component = %event_data.component,
                        reason = %event_data.reason,
                        position = ?event_data.position,
                        input_excerpt = %event_data.input_excerpt,
                        timestamp_rfc3339 = %event_data.timestamp.to_rfc3339(),
                        "IRC protocol error"
                    );
                }
                "WARN" => {
                    event!(
                        Level::WARN,
                        error_id = format!("{:016x}", event_data.error_id),
                        error_code = %event_data.code,
                        component = %event_data.component,
                        reason = %event_data.reason,
                        position = ?event_data.position,
                        input_excerpt = %event_data.input_excerpt,
                        timestamp_rfc3339 = %event_data.timestamp.to_rfc3339(),
                        "IRC protocol error"
                    );
                }
                "INFO" => {
                    event!(
                        Level::INFO,
                        error_id = format!("{:016x}", event_data.error_id),
                        error_code = %event_data.code,
                        component = %event_data.component,
                        reason = %event_data.reason,
                        position = ?event_data.position,
                        input_excerpt = %event_data.input_excerpt,
                        timestamp_rfc3339 = %event_data.timestamp.to_rfc3339(),
                        "IRC protocol error"
                    );
                }
                _ => {
                    event!(
                        Level::DEBUG,
                        error_id = format!("{:016x}", event_data.error_id),
                        error_code = %event_data.code,
                        component = %event_data.component,
                        reason = %event_data.reason,
                        position = ?event_data.position,
                        input_excerpt = %event_data.input_excerpt,
                        timestamp_rfc3339 = %event_data.timestamp.to_rfc3339(),
                        "IRC protocol error"
                    );
                }
            };

            if self.error.is_alert_worthy() {
                error!(
                    alert = true,
                    error_code = %event_data.code,
                    reason = %event_data.reason,
                    "IRC ALERT: Potential security or system issue detected"
                );
            }

            self
        }

        fn count_in_metrics(&self) -> &Self {
            debug!(
                error_code = %self.error.error_code(),
                metric = true,
                component = %self.component,
                "irc_error_count"
            );

            self
        }
    }

    pub fn batch_errors<'a, I>(errors: I)
    where
        I: IntoIterator<Item = &'a IRCv3Error<&'a str>>,
    {
        let mut error_counts: std::collections::HashMap<&'static str, usize> =
            std::collections::HashMap::new();
        let mut alert_worthy = false;

        for error in errors {
            let code = error.error.error_code();
            *error_counts.entry(code).or_insert(0) += 1;

            if error.error.is_alert_worthy() {
                alert_worthy = true;
            }
        }

        info!(
            error_codes = ?error_counts.keys().collect::<Vec<_>>(),
            counts = ?error_counts.values().collect::<Vec<_>>(),
            alert_worthy = alert_worthy,
            "Batched IRC parser errors"
        );

        if alert_worthy {
            error!(
                alert = true,
                error_codes = ?error_counts.keys().collect::<Vec<_>>(),
                "IRC ALERT: Batch contains security or system issues"
            );
        }
    }
}
